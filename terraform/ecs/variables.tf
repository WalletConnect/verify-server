#-------------------------------------------------------------------------------
# Cluster

variable "ecr_repository_url" {
  description = "The URL of the ECR repository where the app image is stored"
  type        = string
}

variable "image_version" {
  description = "The version of the app image to deploy"
  type        = string
}

variable "task_execution_role_name" {
  description = "The name of the task execution role"
  type        = string
}

variable "task_cpu" {
  description = "The number of CPU units to reserve for the container."
  type        = number
}

variable "task_memory" {
  description = "The amount of memory (in MiB) to reserve for the container."
  type        = number
}

variable "autoscaling_desired_count" {
  description = "Minimum number of instances in the autoscaling group"
  type        = number
  default     = 2
}

variable "autoscaling_min_capacity" {
  description = "Minimum number of instances in the autoscaling group"
  type        = number
  default     = 2
}

variable "autoscaling_max_capacity" {
  description = "Maximum number of instances in the autoscaling group"
  type        = number
  default     = 8
}

variable "cloudwatch_logs_key_arn" {
  description = "The ARN of the KMS key to use for encrypting CloudWatch logs"
  type        = string
}

variable "cloudwatch_retention_in_days" {
  description = "The number of days to retain CloudWatch logs for the DB instance"
  type        = number
  default     = 14
}

#-------------------------------------------------------------------------------
# DNS

variable "route53_zones" {
  description = "The FQDNs to use for the app"
  type        = map(string)
}

variable "route53_zones_certificates" {
  description = "The ARNs of the ACM certificates to use for HTTPS"
  type        = map(string)
}

#-------------------------------------------------------------------------------
# Network

variable "vpc_id" {
  description = "The ID of the VPC to deploy to"
  type        = string
}

variable "public_subnets" {
  description = "The IDs of the public subnets"
  type        = list(string)
}

variable "private_subnets" {
  description = "The IDs of the private subnets"
  type        = list(string)
}

variable "allowed_app_ingress_cidr_blocks" {
  description = "A list of CIDR blocks to allow ingress access to the application."
  type        = string
}

variable "allowed_lb_ingress_cidr_blocks" {
  description = "A list of CIDR blocks to allow ingress access to the load-balancer."
  type        = string
}

#-------------------------------------------------------------------------------
# Application

variable "app_secret" {
  description = "The application secret"
  type        = string
  sensitive   = true
}

variable "port" {
  description = "The port the app listens on"
  type        = number
}

variable "log_level" {
  description = "The log level for the app"
  type        = string
}

variable "project_registry_api_url" {
  description = "The url of the project registry API"
  type        = string
}

variable "project_registry_api_auth_token" {
  description = "The auth token for the project registry API"
  type        = string
  sensitive   = true
}

variable "data_api_url" {
  description = "The url of the data API"
  type        = string
}

variable "data_api_auth_token" {
  description = "The auth token for the data API"
  type        = string
  sensitive   = true
}

variable "attestation_cache_url" {
  description = "The endpoint of the attestation cache"
  type        = string
}

variable "project_registry_cache_url" {
  description = "The url of the project registry cache"
  type        = string
}

variable "scam_guard_cache_url" {
  description = "The url of the scam guard cache"
  type        = string
}

variable "cf_kv_account_id" {
  description = "The account ID of the Cloudflare KV store"
  type        = string
}

variable "cf_kv_namespace_id" {
  description = "The namespace ID of the Cloudflare KV store"
  type        = string
}

variable "cf_kv_bearer_token" {
  description = "The Cloudflare API bearer token"
  type        = string
}

variable "ofac_blocked_countries" {
  description = "The list of countries under OFAC sanctions"
  type        = string
}

#-------------------------------------------------------------------------------
# Analytics

variable "analytics_datalake_bucket_name" {
  description = "The name of the S3 bucket to use for the analytics datalake"
  type        = string
}

variable "analytics_datalake_kms_key_arn" {
  description = "The ARN of the KMS key to use with the datalake bucket"
  type        = string
}

#-------------------------------------------------------------------------------
# Autoscaling

variable "autoscaling_cpu_target" {
  description = "The target CPU utilization for the autoscaling group"
  type        = number
  default     = 50
}

variable "autoscaling_cpu_scale_in_cooldown" {
  description = "The cooldown period (in seconds) before a scale in is possible"
  type        = number
  default     = 180
}

variable "autoscaling_cpu_scale_out_cooldown" {
  description = "The cooldown period (in seconds) before a scale out is possible"
  type        = number
  default     = 180
}

variable "autoscaling_memory_target" {
  description = "The target memory utilization for the autoscaling group"
  type        = number
  default     = 50
}

variable "autoscaling_memory_scale_in_cooldown" {
  description = "The cooldown period (in seconds) before a scale in is possible"
  type        = number
  default     = 180
}

variable "autoscaling_memory_scale_out_cooldown" {
  description = "The cooldown period (in seconds) before a scale out is possible"
  type        = number
  default     = 180
}

#-------------------------------------------------------------------------------
# Monitoring

variable "prometheus_endpoint" {
  description = "The endpoint of the Prometheus server to use for monitoring"
  type        = string
}

#---------------------------------------
# GeoIP

variable "geoip_db_bucket_name" {
  description = "The name of the S3 bucket where the GeoIP database is stored"
  type        = string
}

variable "geoip_db_key" {
  description = "The key of the GeoIP database in the S3 bucket"
  type        = string
}
