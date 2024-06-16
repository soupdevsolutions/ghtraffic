# RENDER WEBSITE LAMBDA ROLE/POLICIES
resource "aws_iam_role" "render_website" {
  assume_role_policy = data.aws_iam_policy_document.render_website_assume_policy.json
}

data "aws_iam_policy_document" "render_website_assume_policy" {
  statement {
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com", "events.amazonaws.com"]
    }
  }
}

data "aws_iam_policy_document" "render_website_policy_document" {
  statement {
    actions = [
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:PutLogEvents",
      "logs:PutMetricFilter",
      "logs:PutRetentionPolicy"
    ]
    resources = [
      "arn:aws:logs:*:*:log-group:/aws/lambda/*"
    ]
  }
}

resource "aws_iam_policy" "render_website_policy" {
  name   = "render_website_policy"
  policy = data.aws_iam_policy_document.render_website_policy_document.json
}

resource "aws_iam_role_policy_attachment" "render_website_policy_attachment" {
  role       = aws_iam_role.render_website.name
  policy_arn = aws_iam_policy.render_website_policy.arn
}


# GET GITHUB DATA LAMBDA ROLE/POLICIES
resource "aws_iam_role" "get_github_data" {
  assume_role_policy = data.aws_iam_policy_document.get_github_data_assume_policy.json
}

data "aws_iam_policy_document" "get_github_data_assume_policy" {
  statement {
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com", "events.amazonaws.com"]
    }
  }
}

data "aws_iam_policy_document" "get_github_data_policy_document" {
  statement {
    actions = [
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:PutLogEvents",
      "logs:PutMetricFilter",
      "logs:PutRetentionPolicy"
    ]
    resources = [
      "arn:aws:logs:*:*:log-group:/aws/lambda/*"
    ]
  }
}

resource "aws_iam_policy" "get_github_data_policy" {
  name   = "get_github_data_policy"
  policy = data.aws_iam_policy_document.get_github_data_policy_document.json
}

resource "aws_iam_role_policy_attachment" "get_github_data_policy_attachment" {
  role       = aws_iam_role.get_github_data.name
  policy_arn = aws_iam_policy.get_github_data_policy.arn
}
