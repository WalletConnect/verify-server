locals {
  app_name              = "verify"
  fqdn                  = terraform.workspace == "prod" ? var.public_url : "${terraform.workspace}.${var.public_url}"
  backup_fqdn           = replace(local.fqdn, ".com", ".org")
  latest_release_name   = data.github_release.latest_release.name
  version               = coalesce(var.image_version, substr(local.latest_release_name, 1, length(local.latest_release_name)))
  geoip_db_bucket_name  = "${terraform.workspace}.relay.geo.ip.database.private.${terraform.workspace}.walletconnect"
  data_lake_bucket_name = "walletconnect.data-lake.${terraform.workspace}"
}

data "assert_test" "workspace" {
  test  = terraform.workspace != "default"
  throw = "default workspace is not valid in this project"
}

data "github_release" "latest_release" {
  repository  = "bouncer"
  owner       = "walletconnect"
  retrieve_by = "latest"
}

module "tags" {
  source = "github.com/WalletConnect/terraform-modules.git//modules/tags"

  application = local.app_name
  env         = terraform.workspace
}

module "dns" {
  source = "github.com/WalletConnect/terraform-modules.git//modules/dns"

  hosted_zone_name = var.public_url
  fqdn             = local.fqdn
}

module "backup_dns" {
  source = "github.com/WalletConnect/terraform-modules.git//modules/dns"

  hosted_zone_name = replace(var.public_url, ".com", ".org")
  fqdn             = local.backup_fqdn
}

resource "aws_prometheus_workspace" "prometheus" {
  alias = "prometheus-${terraform.workspace}-${local.app_name}"
}

module "o11y" {
  source = "./monitoring"

  environment             = terraform.workspace
  app_name                = local.app_name
  prometheus_workspace_id = aws_prometheus_workspace.prometheus.id
}

module "vpc" {
  source = "terraform-aws-modules/vpc/aws"
  name   = "${terraform.workspace}-${local.app_name}"

  cidr = "10.0.0.0/16"

  azs             = var.azs
  private_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets  = ["10.0.4.0/24", "10.0.5.0/24", "10.0.6.0/24"]

  private_subnet_tags = {
    Visibility = "private"
  }
  public_subnet_tags = {
    Visibility = "public"
  }

  enable_dns_support     = true
  enable_dns_hostnames   = true
  enable_nat_gateway     = true
  single_nat_gateway     = true
  one_nat_gateway_per_az = false
}

data "aws_ecr_repository" "repository" {
  name = "bouncer"
}

module "redis" {
  source = "./redis"

  redis_name                  = "${terraform.workspace}-${local.app_name}"
  app_name                    = local.app_name
  node_type                   = "cache.t4g.micro"
  vpc_id                      = module.vpc.vpc_id
  allowed_egress_cidr_blocks  = [module.vpc.vpc_cidr_block]
  allowed_ingress_cidr_blocks = [module.vpc.vpc_cidr_block]
  private_subnets             = module.vpc.private_subnets
}

module "ecs" {
  source = "./ecs"

  app_name                    = "${terraform.workspace}-${local.app_name}"
  environment                 = terraform.workspace
  prometheus_endpoint         = aws_prometheus_workspace.prometheus.prometheus_endpoint
  image                       = "${data.aws_ecr_repository.repository.repository_url}:${local.version}"
  acm_certificate_arn         = module.dns.certificate_arn
  cpu                         = var.cpu
  fqdn                        = local.fqdn
  memory                      = var.memory
  private_subnets             = module.vpc.private_subnets
  public_subnets              = module.vpc.public_subnets
  region                      = var.region
  route53_zone_id             = module.dns.zone_id
  backup_acm_certificate_arn  = module.backup_dns.certificate_arn
  backup_fqdn                 = local.backup_fqdn
  backup_route53_zone_id      = module.backup_dns.zone_id
  vpc_cidr                    = module.vpc.vpc_cidr_block
  vpc_id                      = module.vpc.vpc_id
  redis_url                   = module.redis.endpoint
  project_registry_url        = var.project_registry_url
  project_registry_auth_token = var.project_registry_auth_token
  data_api_url                = var.data_api_url
  data_api_auth_token         = var.data_api_auth_token
  secret                      = var.secret
  geoip_db_bucket_name        = local.geoip_db_bucket_name
  geoip_db_key                = var.geoip_db_key
  data_lake_bucket_name       = local.data_lake_bucket_name
}
