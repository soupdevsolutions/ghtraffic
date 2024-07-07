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


# GET REPOSITORIES DATA LAMBDA ROLE/POLICIES
resource "aws_iam_role" "list_repositories" {
  assume_role_policy = data.aws_iam_policy_document.list_repositories_assume_policy.json
}

data "aws_iam_policy_document" "list_repositories_assume_policy" {
  statement {
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com", "events.amazonaws.com"]
    }
  }
}

data "aws_iam_policy_document" "list_repositories_policy_document" {
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

resource "aws_iam_policy" "list_repositories_policy" {
  name   = "list_repositories_policy"
  policy = data.aws_iam_policy_document.list_repositories_policy_document.json
}

resource "aws_iam_role_policy_attachment" "list_repositories_policy_attachment" {
  role       = aws_iam_role.list_repositories.name
  policy_arn = aws_iam_policy.list_repositories_policy.arn
}


# GET REPOSITORY DATA LAMBDA ROLE/POLICIES
resource "aws_iam_role" "calculate_traffic" {
  assume_role_policy = data.aws_iam_policy_document.calculate_traffic_assume_policy.json
}

data "aws_iam_policy_document" "calculate_traffic_assume_policy" {
  statement {
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com", "events.amazonaws.com"]
    }
  }
}

data "aws_iam_policy_document" "calculate_traffic_policy_document" {
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

resource "aws_iam_policy" "calculate_traffic_policy" {
  name   = "calculate_traffic_policy"
  policy = data.aws_iam_policy_document.calculate_traffic_policy_document.json
}

resource "aws_iam_role_policy_attachment" "calculate_traffic_policy_attachment" {
  role       = aws_iam_role.calculate_traffic.name
  policy_arn = aws_iam_policy.calculate_traffic_policy.arn
}
