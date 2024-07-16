data "aws_route53_zone" "ghtraffic" {
  name         = "ghtraffic.com"
  private_zone = false
}

resource "aws_route53_record" "ghtraffic" {
  for_each = {
    for dvo in aws_acm_certificate.ghtraffic.domain_validation_options : dvo.domain_name => {
      name   = dvo.resource_record_name
      record = dvo.resource_record_value
      type   = dvo.resource_record_type
    }
  }

  allow_overwrite = true
  name            = each.value.name
  records         = [each.value.record]
  ttl             = 60
  type            = each.value.type
  zone_id         = data.aws_route53_zone.ghtraffic.zone_id
}

resource "aws_route53_record" "ghtraffic_alias" {
  name    = aws_api_gateway_domain_name.ghtraffic.domain_name
  type    = "A"
  zone_id = aws_route53_zone.ghtraffic.id

  alias {
    evaluate_target_health = true
    name                   = aws_api_gateway_domain_name.ghtraffic.cloudfront_domain_name
    zone_id                = aws_api_gateway_domain_name.ghtraffic.cloudfront_zone_id
  }
}
