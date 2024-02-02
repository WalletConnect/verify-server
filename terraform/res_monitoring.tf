module "monitoring" {
  source  = "./monitoring"
  context = module.this

  monitoring_role_arn   = data.terraform_remote_state.monitoring.outputs.grafana_workspaces.central.iam_role_arn
  notification_channels = var.notification_channels
  prometheus_endpoint   = aws_prometheus_workspace.prometheus.prometheus_endpoint
  ecs_service_name      = module.ecs.ecs_service_name
  redis_cluster_id      = module.redis.cluster_id
}
