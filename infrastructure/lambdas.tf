# HELLO WORLD
resource "aws_lambda_function" "hello_world_lambda" {
  function_name = "InitiatePayment"

  filename         = "data/lambdas/hello_world.zip"
  source_code_hash = filebase64sha256("data/lambdas/hello_world.zip")

  handler = "handler"
  runtime = "provided.al2023"

  role = aws_iam_role.hello_world.arn
}
