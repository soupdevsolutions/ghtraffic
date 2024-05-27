# GENERIC RESOURCES
resource "aws_apigatewayv2_api" "api" {
  name          = "rust-lambda-starter API"
  description   = "rust-lambda-starter API"
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
}

resource "aws_apigatewayv2_deployment" "api_deployment" {
  api_id      = aws_apigatewayv2_api.api.id
  description = "rust-lambda-starter API deployment"

  lifecycle {
    create_before_destroy = true
  }

  triggers = {
    redeployment = sha1(
      jsonencode([
        file("api_gateway.tf"),
      ])
    )
  }
}

# HELLO WORLD
resource "aws_apigatewayv2_integration" "hello_world_integration" {
  api_id           = aws_apigatewayv2_api.api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Hello World"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.hello_world_lambda.invoke_arn

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "hello_world_route" {
  api_id    = aws_apigatewayv2_api.api.id
  route_key = "GET /hello"
  target    = "integrations/${aws_apigatewayv2_integration.hello_world_integration.id}"
}

resource "aws_lambda_permission" "hello_world_api_permission" {
  function_name = aws_lambda_function.hello_world_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.api.execution_arn}/*/*"
}
