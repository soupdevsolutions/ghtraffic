use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

async fn handler(_event: Request) -> Result<Response<Body>, Error> {

    let client_id = std::env::var("GH_CLIENT_ID").expect("GH_CLIENT_ID is not set"); 

    let data = format!(r#"
    <html>
        <head>
            <title>soup.dev - ghTraffic</title>
        </head>
        <body>
            <h1>Welcome to ghTraffic by soup.dev</h1>
            <a href="https://github.com/login/oauth/authorize?client_id={client_id}">Login with GitHub</a>
        </body>
    </html>
    "#);

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
