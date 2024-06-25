#-------------------------------------------------------------------------------
# Configuration

variable "grafana_auth" {
  description = "The API Token for the Grafana instance"
  type        = string
  default     = ""
}


#-------------------------------------------------------------------------------
# Service

variable "name" {
  description = "The name of the application"
  type        = string
  default     = "verify-server"
}

variable "region" {
  description = "AWS region to deploy to"
  type        = string
}

variable "image_version" {
  description = "The ECS tag of the image to deploy"
  type        = string
}

variable "task_cpu" {
  description = "The number of CPU units to allocate to the task"
  type        = number
}

variable "task_memory" {
  description = "The amount of memory to allocate to the task"
  type        = number
}

variable "app_autoscaling_desired_count" {
  description = "The desired number of tasks to run"
  type        = number
  default     = 1
}

variable "app_autoscaling_min_capacity" {
  description = "The minimum number of tasks to run when autoscaling"
  type        = number
  default     = 1
}

variable "app_autoscaling_max_capacity" {
  description = "The maximum number of tasks to run when autoscaling"
  type        = number
  default     = 1
}

#-------------------------------------------------------------------------------
# Application

variable "app_secret" {
  description = "The application secret"
  type        = string
  sensitive   = true
}

variable "log_level" {
  description = "Defines logging level for the application"
  type        = string
}

variable "ofac_blocked_countries" {
  description = "The list of countries to block"
  type        = string
  default     = ""
}

#-------------------------------------------------------------------------------
# Cloudflare KV for V2 migration

variable "cf_kv_endpoint" {
  description = "The endpoint of the Cloudflare KV worker"
  type        = string
}

#-------------------------------------------------------------------------------
# Project Registry

variable "project_registry_api_url" {
  description = "The url of the project registry API"
  type        = string
}

variable "project_registry_api_auth_token" {
  description = "The auth token for the project registry API"
  type        = string
  sensitive   = true
}

#-------------------------------------------------------------------------------
# Data API

variable "data_api_url" {
  description = "The url of the data API"
  type        = string
}

variable "data_api_auth_token" {
  description = "The auth token for the data API"
  type        = string
  sensitive   = true
}


#-------------------------------------------------------------------------------
# Analytics

variable "geoip_db_key" {
  description = "The name to the GeoIP database"
  type        = string
}


#-------------------------------------------------------------------------------
# Alerting / Monitoring

variable "notification_channels" {
  description = "The notification channels to send alerts to"
  type        = list(any)
  default     = []
}

variable "webhook_cloudwatch_p2" {
  description = "The webhook to send CloudWatch P2 alerts to"
  type        = string
  default     = ""
}

variable "webhook_prometheus_p2" {
  description = "The webhook to send Prometheus P2 alerts to"
  type        = string
  default     = ""
}
