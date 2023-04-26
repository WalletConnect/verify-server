terraform {
  required_version = "~> 1.0"

  required_providers {
    grafana = {
      source  = "grafana/grafana"
      version = "~> 1.24"
    }
  }
}

locals {
  opsgenie_notification_channel = "l_iaPw6nk"
  notifications = (
    var.environment == "prod" ?
    "[{\"uid\": \"${local.opsgenie_notification_channel}\"}]" :
    "[]"
  )
}

resource "grafana_data_source" "prometheus" {
  type = "prometheus"
  name = "${var.environment}-${var.app_name}-amp"
  url  = "https://aps-workspaces.eu-central-1.amazonaws.com/workspaces/${var.prometheus_workspace_id}/"

  json_data_encoded = jsonencode({
    httpMethod    = "GET"
    manageAlerts  = false
    sigV4Auth     = true
    sigV4AuthType = "ec2_iam_role"
    sigV4Region   = "eu-central-1"
  })
}

resource "grafana_data_source" "cloudwatch" {
  type = "cloudwatch"
  name = "${var.environment}-${var.app_name}-cloudwatch"

  json_data {
    default_region = "eu-central-1"
  }
}

# JSON Dashboard. When exporting from Grafana make sure that all
# variables are replaced properly using template syntax
data "template_file" "grafana_dashboard_template" {
  template = file("monitoring/grafana-dashboard.json.tpl")
  vars = {
    environment                = var.environment
    prometheus_data_source_uid = grafana_data_source.prometheus.uid
    cloudwatch_data_source_uid = grafana_data_source.cloudwatch.uid
  }
}

resource "grafana_dashboard" "at_a_glance" {
  overwrite   = true
  message     = "Updated by Terraform"
  config_json = data.template_file.grafana_dashboard_template.rendered
}
