data "jsonnet_file" "dashboard" {
  source = "${path.module}/dashboard.jsonnet"

  ext_str = {
    dashboard_title = "Verify Server - ${title(module.this.stage)}"
    dashboard_uid   = "verify-${module.this.stage}"

    prometheus_uid = grafana_data_source.prometheus.uid
    cloudwatch_uid = grafana_data_source.cloudwatch.uid

    environment   = module.this.stage
    notifications = jsonencode(var.notification_channels)

    ecs_service_name = var.ecs_service_name
    redis_cluster_id = var.redis_cluster_id
  }
}

resource "grafana_dashboard" "at_a_glance" {
  overwrite   = true
  message     = "Updated by Terraform"
  config_json = data.jsonnet_file.dashboard.rendered
}
