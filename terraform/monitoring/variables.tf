variable "monitoring_role_arn" {
  description = "The ARN of the monitoring role."
  type        = string
}

variable "notification_channels" {
  description = "The notification channels to send alerts to"
  type        = list(any)
}

variable "prometheus_endpoint" {
  description = "The endpoint for the Prometheus server."
  type        = string
}

variable "ecs_service_name" {
  description = "The name of the ECS service."
  type        = string
}

variable "redis_cluster_id" {
  description = "The ID of the Redis cluster."
  type        = string
}
