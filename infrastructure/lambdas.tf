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
resource "aws_lambda_function" "get_repositories_data_lambda" {
  function_name = "GetRepositoriesData"

  filename         = "data/lambdas/get_repositories_data.zip"
  source_code_hash = filebase64sha256("data/lambdas/get_repositories_data.zip")

  handler = "handler"
  runtime = "provided.al2023"

  role = aws_iam_role.get_repositories_data.arn

  environment {
    variables = {
      GITHUB_CLIENT_ID     = var.GITHUB_CLIENT_ID
      GITHUB_CLIENT_SECRET = var.GITHUB_CLIENT_SECRET
    }
  }
}


# GET REPOSITORY DATA
resource "aws_lambda_function" "get_repository_data_lambda" {
  function_name = "GetRepositoryData"

  filename         = "data/lambdas/get_repository_data.zip"
  source_code_hash = filebase64sha256("data/lambdas/get_repository_data.zip")

  handler = "handler"
  runtime = "provided.al2023"

  role = aws_iam_role.get_repository_data.arn

  environment {
    variables = {
      GITHUB_CLIENT_ID     = var.GITHUB_CLIENT_ID
      GITHUB_CLIENT_SECRET = var.GITHUB_CLIENT_SECRET
    }
  }
}
