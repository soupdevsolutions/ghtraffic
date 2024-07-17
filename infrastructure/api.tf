# GENERIC RESOURCES
resource "aws_apigatewayv2_api" "api" {
  name          = "GhTraffic API"
  description   = "GhTraffic API"
  protocol_type = "HTTP"

  cors_configuration {
    allow_origins = ["*"]
    allow_methods = ["*"]
    allow_headers = ["*"]
  }
}

resource "aws_apigatewayv2_domain_name" "domain_name" {
  domain_name = "ghtraffic.com"

  domain_name_configuration {
    certificate_arn = aws_acm_certificate.ghtraffic.arn
    endpoint_type   = "REGIONAL"
    security_policy = "TLS_1_2"
  }

  depends_on = [aws_acm_certificate_validation.ghtraffic]
}

resource "aws_apigatewayv2_stage" "api_stage" {
  api_id      = aws_apigatewayv2_api.api.id
  name        = "$default"
  auto_deploy = true
}


resource "aws_apigatewayv2_api_mapping" "api_mapping" {
  api_id      = aws_apigatewayv2_api.api.id
  domain_name = aws_apigatewayv2_domain_name.domain_name.id
  stage       = aws_apigatewayv2_stage.api_stage.id
}

# RENDER WEBSITE 
resource "aws_apigatewayv2_integration" "render_website_integration" {
  api_id           = aws_apigatewayv2_api.api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Render website"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.render_website_lambda.invoke_arn

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "render_website_route" {
  api_id    = aws_apigatewayv2_api.api.id
  route_key = "GET /"
  target    = "integrations/${aws_apigatewayv2_integration.render_website_integration.id}"
}

resource "aws_lambda_permission" "render_website_api_permission" {
  function_name = aws_lambda_function.render_website_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api.execution_arn}/*/*"
}


# GET REPOSITORIES DATA
resource "aws_apigatewayv2_integration" "list_repositories_integration" {
  api_id           = aws_apigatewayv2_api.api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Get repositories data"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.list_repositories_lambda.invoke_arn

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "list_repositories_route" {
  api_id    = aws_apigatewayv2_api.api.id
  route_key = "GET /repos"
  target    = "integrations/${aws_apigatewayv2_integration.list_repositories_integration.id}"
}

resource "aws_lambda_permission" "list_repositories_api_permission" {
  function_name = aws_lambda_function.list_repositories_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api.execution_arn}/*/*"
}


# GET REPOSITORY DATA
resource "aws_apigatewayv2_integration" "calculate_traffic_integration" {
  api_id           = aws_apigatewayv2_api.api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Get repository data"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.calculate_traffic_lambda.invoke_arn

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "calculate_traffic_route" {
  api_id    = aws_apigatewayv2_api.api.id
  route_key = "GET /repo"
  target    = "integrations/${aws_apigatewayv2_integration.calculate_traffic_integration.id}"
}

resource "aws_lambda_permission" "calculate_traffic_api_permission" {
  function_name = aws_lambda_function.calculate_traffic_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api.execution_arn}/*/*"
}
