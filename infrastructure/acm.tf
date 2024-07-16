resource "aws_acm_certificate" "ghtraffic" {
  domain_name       = "ghtraffic.com"
  validation_method = "DNS"
}

resource "aws_acm_certificate_validation" "ghtraffic" {
  certificate_arn = aws_acm_certificate.ghtraffic.arn
  validation_record_fqdns = [for record in aws_route53_record.ghtraffic : record.fqdn]
}
