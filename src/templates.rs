use askama::Template;

use crate::github::{UserAggregatedViews, UserRepository};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub authenticated: bool,
    pub login_uri: Option<String>,
}

#[derive(Template)]
#[template(path = "repo_list.html")]
pub struct RepoListTemplate {
    pub repositories: Vec<UserRepository>,
}

#[derive(Template)]
#[template(path = "repo_info.html")]
pub struct RepoViewsTemplate {
    pub views: UserAggregatedViews,
}
