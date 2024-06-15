use ghtraffic::github::GithubClient;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

#[tracing::instrument]
async fn handler(github_client: &GithubClient, event: Request) -> anyhow::Result<Response<Body>> {
    tracing::info!("Received event: {:?}", event);
    let code = event
        .query_string_parameters()
        .first("code")
        .map(|v| v.to_string());

    let login_uri = github_client.get_login_uri()?;
    let mut body = format!(
        r#"
        <body>
            <h1>Welcome to ghTraffic by soup.dev</h1>
            <a href={login_uri}>Login with GitHub</a>
        </body>
    "#
    );

    if let Some(code) = code {
        let response = github_client.exchange_code(code).await?;
        body = format!(
            r#"
            <body>
                <h1>Successfully authenticated with GitHub</h1>
                <p>Access Token: {}</p>
            </body> 
        "#,
            response.access_token
        );
    }

    let data = format!(
        r#"
    <html>
        <head>
            <title>ghTraffic by soup.dev</title>
        </head>
        {body}
    </html>
    "#
    );

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(data.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let github_client = GithubClient::new();
    run(service_fn(|request| handler(&github_client, request))).await
}
