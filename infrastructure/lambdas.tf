# RENDER WEBSITE
resource "aws_lambda_function" "render_website_lambda" {
  function_name = "RenderWebsite"

  filename         = "data/lambdas/render_website.zip"
  source_code_hash = filebase64sha256("data/lambdas/render_website.zip")

  handler = "handler"
  runtime = "provided.al2023"

  role = aws_iam_role.render_website.arn

  environment {
    variables = {
      GITHUB_CLIENT_ID     = var.GITHUB_CLIENT_ID
      GITHUB_CLIENT_SECRET = var.GITHUB_CLIENT_SECRET
    }
  }
}

# GET GITHUB DATA
resource "aws_lambda_function" "get_github_data_lambda" {
  function_name = "GetGithubData"

  filename         = "data/lambdas/get_github_data.zip"
  source_code_hash = filebase64sha256("data/lambdas/get_github_data.zip")

  handler = "handler"
  runtime = "provided.al2023"

  role = aws_iam_role.get_githuv_data.arn

  environment {
    variables = {
      GITHUB_CLIENT_ID     = var.GITHUB_CLIENT_ID
      GITHUB_CLIENT_SECRET = var.GITHUB_CLIENT_SECRET
    }
  }
}
