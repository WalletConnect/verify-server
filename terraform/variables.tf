variable "cpu" {
  type = number
}

variable "memory" {
  type = number
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

variable "region" {
  type    = string
  default = "eu-central-1"
}

variable "secret" {
  type = string
}

variable "azs" {
  type    = list(string)
  default = ["eu-central-1a", "eu-central-1b", "eu-central-1c"]
}

variable "public_url" {
  type    = string
  default = "verify.walletconnect.com"
}

variable "grafana_endpoint" {
  type = string
}

variable "image_version" {
  type    = string
  default = ""
}

#---------------------------------------
# GeoIP

variable "geoip_db_key" {
  description = "The name to the GeoIP database"
  type        = string
  default     = "GeoLite2-City.mmdb"
}

#---------------------------------------
# Analytics

variable "data_lake_kms_key_arn" {
  description = "The ARN of the KMS encryption key for data-lake bucket."
  type        = string
}
