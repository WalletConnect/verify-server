variable "project_registry_url" {
  type = string
}

variable "project_registry_auth_token" {
  type      = string
  sensitive = true
}

variable "domain_whitelist" {
  type = string
}

variable "region" {
  type    = string
  default = "eu-central-1"
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
