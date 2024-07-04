use askama::Template;

use crate::github::{UserRepository, UserRepositoryViews};

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
    pub repositories: Vec<UserRepository>,
}

#[derive(Template)]
#[template(path = "repo_info.html")]
pub struct RepoViewsTemplate {
    pub repo_name: String,
    pub views: UserRepositoryViews,
}
