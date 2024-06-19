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
  api_id      = aws_apigatewayv2_api.api.id
  name        = "$default"
  auto_deploy = true
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


# GET REPOSITORIES DATA
resource "aws_apigatewayv2_integration" "get_repositories_data_integration" {
  api_id           = aws_apigatewayv2_api.api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Get repositories data"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.get_repositories_data_lambda.invoke_arn

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "get_repositories_data_route" {
  api_id    = aws_apigatewayv2_api.api.id
  route_key = "GET /repos"
  target    = "integrations/${aws_apigatewayv2_integration.get_repositories_data_integration.id}"
}

resource "aws_lambda_permission" "get_repositories_data_api_permission" {
  function_name = aws_lambda_function.get_repositories_data_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api.execution_arn}/*/*"
}


# GET REPOSITORY DATA
resource "aws_apigatewayv2_integration" "get_repository_data_integration" {
  api_id           = aws_apigatewayv2_api.api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Get repository data"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.get_repository_data_lambda.invoke_arn

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "get_repository_data_route" {
  api_id    = aws_apigatewayv2_api.api.id
  route_key = "GET /repo"
  target    = "integrations/${aws_apigatewayv2_integration.get_repository_data_integration.id}"
}

resource "aws_lambda_permission" "get_repository_data_api_permission" {
  function_name = aws_lambda_function.get_repository_data_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api.execution_arn}/*/*"
}
