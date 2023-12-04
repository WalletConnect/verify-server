variable "region" {
  type = string
}

variable "app_name" {
  type = string
}

variable "environment" {
  type = string
}

variable "image" {
  type = string
}

variable "redis_url" {
  type      = string
  sensitive = true
}

variable "project_registry_url" {
  type = string
}

variable "project_registry_auth_token" {
  type      = string
  sensitive = true
}

variable "data_api_url" {
  type = string
}

variable "data_api_auth_token" {
  type      = string
  sensitive = true
}

variable "secret" {
  type = string
}

variable "prometheus_endpoint" {
  type = string
}

variable "vpc_id" {
  type = string
}

variable "vpc_cidr" {
  type = string
}

variable "route53_zone_id" {
  type = string
}

variable "fqdn" {
  type = string
}

variable "acm_certificate_arn" {
  type = string
}

variable "backup_acm_certificate_arn" {
  type = string
}

variable "backup_fqdn" {
  type = string
}

variable "backup_route53_zone_id" {
  type = string
}

variable "public_subnets" {
  type = set(string)
}

variable "private_subnets" {
  type = set(string)
}

variable "cpu" {
  type = number
}

variable "memory" {
  type = number
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

#---------------------------------------
# Analytics

variable "data_lake_bucket_name" {
  description = "The name of the S3 bucket where the analytics data is stored"
  type        = string
}

variable "data_lake_kms_key_arn" {
  description = "The ARN of the KMS encryption key for data-lake bucket."
  type        = string
}
