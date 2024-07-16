data "aws_route53_zone" "ghtraffic" {
  name = "ghtraffic.com"
}

resource "aws_route53_record" "ghtraffic" {
  name    = aws_api_gateway_domain_name.ghtraffic.domain_name
  type    = "A"
  zone_id = data.aws_route53_zone.ghtraffic.id

  alias {
    evaluate_target_health = true
    name                   = aws_api_gateway_domain_name.ghtraffic.cloudfront_domain_name
    zone_id                = aws_api_gateway_domain_name.ghtraffic.cloudfront_zone_id
  }
}
