use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate();

#[derive(Template)]
#[template(path = "welcome.html")]
pub struct WelcomeTemplate {
    pub login_uri: String,
    pub client_id: String,
}

#[derive(Template)]
#[template(path = "authenticated.html")]
pub struct AuthenticatedTemplate {
    pub token: String,
}
