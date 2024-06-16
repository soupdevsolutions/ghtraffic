use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub code: Option<String>,
}

#[derive(Template)]
#[template(path = "welcome.html")]
pub struct WelcomeTemplate {
    pub login_uri: String,
}

#[derive(Template)]
#[template(path = "authenticated.html")]
pub struct AuthenticatedTemplate {
    pub token: String,
}
