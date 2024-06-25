locals {
  image = "${var.ecr_repository_url}:${var.image_version}"

  otel_port   = var.port + 1
  otel_cpu    = 128
  otel_memory = 128

  file_descriptor_soft_limit = pow(2, 18)
  file_descriptor_hard_limit = local.file_descriptor_soft_limit * 2
}

module "ecs_cpu_mem" {
  source  = "app.terraform.io/wallet-connect/ecs_cpu_mem/aws"
  version = "1.0.0"
  cpu     = var.task_cpu + local.otel_cpu
  memory  = var.task_memory + local.otel_memory
}

#-------------------------------------------------------------------------------
# ECS Cluster

resource "aws_ecs_cluster" "app_cluster" {
  name = "${module.this.id}-cluster"

  configuration {
    execute_command_configuration {
      logging = "OVERRIDE"

      log_configuration {
        cloud_watch_encryption_enabled = false
        cloud_watch_log_group_name     = aws_cloudwatch_log_group.cluster.name
      }
    }
  }

  # Exposes metrics such as the number of running tasks in CloudWatch
  setting {
    name  = "containerInsights"
    value = "enabled"
  }
}

#-------------------------------------------------------------------------------
# ECS Task definition

resource "aws_ecs_task_definition" "app_task" {
  family = module.this.id

  requires_compatibilities = ["FARGATE"]
  network_mode             = "awsvpc"
  cpu                      = module.ecs_cpu_mem.cpu
  memory                   = module.ecs_cpu_mem.memory
  execution_role_arn       = data.aws_iam_role.ecs_task_execution_role.arn
  task_role_arn            = data.aws_iam_role.ecs_task_execution_role.arn

  runtime_platform {
    operating_system_family = "LINUX"
  }

  container_definitions = jsonencode([
    {
      name      = module.this.id,
      image     = local.image,
      cpu       = var.task_cpu,
      memory    = var.task_memory,
      essential = true,

      environment = [
        { name = "SECRET", value = var.app_secret },

        { name = "PORT", value = tostring(var.port) },
        { name = "PROMETHEUS_PORT", value = tostring(local.otel_port) },

        { name = "LOG_LEVEL", value = var.log_level },

        { name = "GEOIP_DB_BUCKET", value = var.geoip_db_bucket_name },
        { name = "GEOIP_DB_KEY", value = var.geoip_db_key },

        { name = "PROJECT_REGISTRY_URL", value = var.project_registry_api_url },
        { name = "PROJECT_REGISTRY_AUTH_TOKEN", value = var.project_registry_api_auth_token },

        { name = "DATA_API_URL", value = var.data_api_url },
        { name = "DATA_API_AUTH_TOKEN", value = var.data_api_auth_token },

        { name = "ATTESTATION_CACHE_URL", value = var.attestation_cache_url },
        { name = "PROJECT_REGISTRY_CACHE_URL", value = var.project_registry_cache_url },
        { name = "SCAM_GUARD_CACHE_URL", value = var.scam_guard_cache_url },

        { name = "CF_KV_ENDPOINT", value = var.cf_kv_endpoint },

        { name = "DATA_LAKE_BUCKET", value = var.analytics_datalake_bucket_name },

        { name = "BLOCKED_COUNTRIES", value = var.ofac_blocked_countries },
      ],

      ulimits = [{
        name : "nofile",
        softLimit : local.file_descriptor_soft_limit,
        hardLimit : local.file_descriptor_hard_limit
      }],

      portMappings = [
        {
          containerPort = var.port,
          hostPort      = var.port
        },
        {
          containerPort = local.otel_port,
          hostPort      = local.otel_port
        }
      ],

      logConfiguration : {
        logDriver = "awslogs",
        options = {
          "awslogs-group"         = aws_cloudwatch_log_group.cluster.name,
          "awslogs-region"        = module.this.region,
          "awslogs-stream-prefix" = "ecs"
        }
      },

      dependsOn = [
        { containerName : "aws-otel-collector", condition : "START" },
      ]
    },

    # Forward telemetry data to AWS CloudWatch
    {
      name      = "aws-otel-collector",
      image     = "public.ecr.aws/aws-observability/aws-otel-collector:v0.31.0",
      cpu       = local.otel_cpu,
      memory    = local.otel_memory,
      essential = true,

      command = [
        "--config=/etc/ecs/ecs-amp-xray-prometheus.yaml"
        # Uncomment to enable debug logging in otel-collector
        # "--set=service.telemetry.logs.level=DEBUG"
      ],

      environment = [
        { name : "AWS_PROMETHEUS_SCRAPING_ENDPOINT", value : "0.0.0.0:${local.otel_port}" },
        { name : "AWS_PROMETHEUS_ENDPOINT", value : "${var.prometheus_endpoint}api/v1/remote_write" },
        { name : "AWS_REGION", value : module.this.region },
      ],

      logConfiguration = {
        logDriver = "awslogs",
        options = {
          "awslogs-group"         = aws_cloudwatch_log_group.otel.name,
          "awslogs-region"        = module.this.region,
          "awslogs-stream-prefix" = "ecs"
        }
      }
    },
  ])
}


#-------------------------------------------------------------------------------
# ECS Service

resource "aws_ecs_service" "app_service" {
  name            = "${module.this.id}-service"
  cluster         = aws_ecs_cluster.app_cluster.id
  task_definition = aws_ecs_task_definition.app_task.arn
  launch_type     = "FARGATE"
  desired_count   = var.autoscaling_desired_count
  propagate_tags  = "TASK_DEFINITION"

  # Wait for the service deployment to succeed
  wait_for_steady_state = true

  network_configuration {
    subnets          = var.private_subnets
    assign_public_ip = false
    security_groups  = [aws_security_group.app_ingress.id]
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.target_group.arn
    container_name   = aws_ecs_task_definition.app_task.family
    container_port   = var.port
  }

  # Allow external changes without Terraform plan difference
  lifecycle {
    ignore_changes = [desired_count]
  }
}
