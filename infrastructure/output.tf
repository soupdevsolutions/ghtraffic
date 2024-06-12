output "api_gateway_url" {
  value = aws_apigatewayv2_stage.api_stage.invoke_url
}

output "templates_bucket_name" {
  value = aws_s3_bucket.htmx_templates.bucket
}
