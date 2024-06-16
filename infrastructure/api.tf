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

resource "aws_apigatewayv2_stage" "api_stage" {
  api_id        = aws_apigatewayv2_api.api.id
  name          = "$default"
  deployment_id = aws_apigatewayv2_deployment.api_deployment.id
  auto_deploy   = true
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
  route_key = "GET /web"
  target    = "integrations/${aws_apigatewayv2_integration.render_website_integration.id}"
}

resource "aws_lambda_permission" "render_website_api_permission" {
  function_name = aws_lambda_function.render_website_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api.execution_arn}/*/*"
}


# GET GITHUB DATA
resource "aws_apigatewayv2_integration" "get_github_data_integration" {
  api_id           = aws_apigatewayv2_api.api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Get GitHub data"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.get_github_data_lambda.invoke_arn

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "get_github_data_route" {
  api_id    = aws_apigatewayv2_api.api.id
  route_key = "GET /github"
  target    = "integrations/${aws_apigatewayv2_integration.get_github_data_integration.id}"
}

resource "aws_lambda_permission" "get_github_data_api_permission" {
  function_name = aws_lambda_function.get_github_data_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api.execution_arn}/*/*"
}
