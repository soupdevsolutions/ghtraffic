# HELLO WORLD LAMBDA ROLE/POLICIES
resource "aws_iam_role" "hello_world" {
  assume_role_policy = data.aws_iam_policy_document.hello_world_assume_policy.json
}

data "aws_iam_policy_document" "hello_world_assume_policy" {
  statement {
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com", "events.amazonaws.com"]
    }
  }
}

data "aws_iam_policy_document" "hello_world_policy_document" {
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

resource "aws_iam_policy" "hello_world_policy" {
  name   = "hello_world_policy"
  policy = data.aws_iam_policy_document.hello_world_policy_document.json
}

resource "aws_iam_role_policy_attachment" "hello_world_policy_attachment" {
  role       = aws_iam_role.hello_world.name
  policy_arn = aws_iam_policy.hello_world_policy.arn
}
