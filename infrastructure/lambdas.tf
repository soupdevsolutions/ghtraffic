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

# GET REPOSITORIES DATA
resource "aws_lambda_function" "list_repositories_lambda" {
  function_name = "GetRepositoriesData"

  filename         = "data/lambdas/list_repositories.zip"
  source_code_hash = filebase64sha256("data/lambdas/list_repositories.zip")

  handler = "handler"
  runtime = "provided.al2023"

  role = aws_iam_role.list_repositories.arn

  environment {
    variables = {
      GITHUB_CLIENT_ID     = var.GITHUB_CLIENT_ID
      GITHUB_CLIENT_SECRET = var.GITHUB_CLIENT_SECRET
    }
  }
}


# GET REPOSITORY DATA
resource "aws_lambda_function" "calculate_traffic_lambda" {
  function_name = "GetRepositoryData"

  filename         = "data/lambdas/calculate_traffic.zip"
  source_code_hash = filebase64sha256("data/lambdas/calculate_traffic.zip")

  handler = "handler"
  runtime = "provided.al2023"

  role = aws_iam_role.calculate_traffic.arn

  environment {
    variables = {
      GITHUB_CLIENT_ID     = var.GITHUB_CLIENT_ID
      GITHUB_CLIENT_SECRET = var.GITHUB_CLIENT_SECRET
    }
  }
}
