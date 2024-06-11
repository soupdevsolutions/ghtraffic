use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

async fn handler(_event: Request) -> anyhow::Result<Response<Body>> {
    /*
        let client_id = std::env::var("GH_CLIENT_ID").expect("GH_CLIENT_ID is not set");
        let code = event
            .query_string_parameters()
            .first("code")
            .map(|v| v.to_string());

        let mut body = format!(
            r#"
            <body>
                <h1>Welcome to ghTraffic by soup.dev</h1>
                <a href="https://github.com/login/oauth/authorize?client_id={client_id}">Login with GitHub</a>
            </body>
        "#
        );

        if let Some(code) = code {
            let client = GithubClient::new();
            let response = client.exchange_code(code).await?;
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
    */
    let data = r#"
    <html>
        <head>
            <title>ghTraffix by soup.dev</title>
        </head>
        <body>Test</body>
    </html>
    "#;

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
    run(service_fn(handler)).await
}
