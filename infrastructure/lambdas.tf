# RENDER WEBSITE
resource "aws_lambda_function" "render_website_lambda" {
  function_name = "InitiatePayment"

  filename         = "data/lambdas/render_website.zip"
  source_code_hash = filebase64sha256("data/lambdas/render_website.zip")

  handler = "handler"
  runtime = "provided.al2023"

  role = aws_iam_role.render_website.arn

  environment {
    variables = {
      GH_CLIENT_ID = var.GH_CLIENT_ID
    }
  }
}
