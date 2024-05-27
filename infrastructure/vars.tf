variable "AWS_REGION" {
  description = "AWS region for all resources."

  type    = string
  default = "eu-west-1"
}

variable "AWS_ACCESS_KEY_ID" {
  type = string
}

variable "AWS_SECRET_ACCESS_KEY" {
  type = string
}
