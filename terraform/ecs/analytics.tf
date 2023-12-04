# Analytics Bucket Access
resource "aws_iam_policy" "analytics-data-lake_bucket_access" {
  name        = "${var.app_name}_analytics-data-lake_bucket_access"
  path        = "/"
  description = "Allows ${var.app_name} to read/write from ${var.data_lake_bucket_name}"

  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Sid" : "ListObjectsInAnalyticsBucket",
        "Effect" : "Allow",
        "Action" : ["s3:ListBucket"],
        "Resource" : ["arn:aws:s3:::${var.data_lake_bucket_name}"]
      },
      {
        "Sid" : "AllObjectActionsInAnalyticsBucket",
        "Effect" : "Allow",
        "Action" : "s3:PutObject",
        "Resource" : ["arn:aws:s3:::${var.data_lake_bucket_name}/verify-server/*"]
      },
      {
        "Sid" : "AllGenerateDataKeyForAnalyticsBucket",
        "Effect" : "Allow",
        "Action" : ["kms:GenerateDataKey"],
        "Resource" : "arn:aws:kms:eu-central-1:898587786287:key/06e7c9fd-943d-47bf-bcf4-781b44411ba4"
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "analytics-data-lake_policy-attach" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = aws_iam_policy.analytics-data-lake_bucket_access.arn
}

# GeoIP Bucket Access
resource "aws_iam_policy" "geoip_bucket_access" {
  name        = "${var.app_name}_geoip_bucket_access"
  path        = "/"
  description = "Allows ${var.app_name} to read from ${var.geoip_db_bucket_name}"

  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Sid" : "ListObjectsInGeoipBucket",
        "Effect" : "Allow",
        "Action" : ["s3:ListBucket"],
        "Resource" : ["arn:aws:s3:::${var.geoip_db_bucket_name}"]
      },
      {
        "Sid" : "AllObjectActionsInGeoipBucket",
        "Effect" : "Allow",
        "Action" : ["s3:CopyObject", "s3:GetObject", "s3:HeadObject"],
        "Resource" : ["arn:aws:s3:::${var.geoip_db_bucket_name}/*"]
      },
    ]
  })
}

resource "aws_iam_role_policy_attachment" "geoip-bucket-policy-attach" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = aws_iam_policy.geoip_bucket_access.arn
}
