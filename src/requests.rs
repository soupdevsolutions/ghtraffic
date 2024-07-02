use lambda_http::Request;

pub fn get_cookie(event: &Request, cookie_name: &str) -> Option<String> {
    let cookie = event
        .headers()
        .get("Cookie")?
        .to_str()
        .ok()?
        .split(';')
        .find(|cookie| cookie.starts_with(cookie_name))?
        .split('=')
        .last()
        .map(String::from)?;

    Some(cookie)
}

pub fn create_set_cookie_header(cookie_name: &str, cookie_value: &str, duration: u32) -> String {
    format!("{}={}; Max-Age={}; HttpOnly", cookie_name, cookie_value, duration) 
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http::header::HeaderValue;
    use lambda_http::http::{HeaderMap, Request};

    #[test]
    fn test_get_cookie_returns_value_for_single_cookie() {
        let mut headers = HeaderMap::new();
        headers.insert("Cookie", HeaderValue::from_static("token=1234"));
        let event = Request::get("https://soup.dev")
            .header("Cookie", "token=1234")
            .body(lambda_http::Body::Empty)
            .unwrap();

        assert_eq!(get_cookie(&event, "token"), Some("1234".to_string()));
    }

    #[test]
    fn test_get_cookie_returns_none_when_cookie_is_not_present() {
        let event = Request::get("https://soup.dev")
            .body(lambda_http::Body::Empty)
            .unwrap();

        assert_eq!(get_cookie(&event, "token"), None);
    }

    #[test]
    fn test_get_cookie_returns_value_for_multiple_cookies() {
        let mut headers = HeaderMap::new();
        headers.insert("Cookie", HeaderValue::from_static("token=1234;name=John"));
        let event = Request::get("https://soup.dev")
            .header("Cookie", "token=1234;name=John")
            .body(lambda_http::Body::Empty)
            .unwrap();

        assert_eq!(get_cookie(&event, "name"), Some("John".to_string()));
    }
}
