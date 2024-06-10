use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

async fn handler(_event: Request) -> Result<Response<Body>, Error> {
    let data = r#"
    <html>
        <head>
            <title>My Website</title>
        </head>
        <body>
            <h1>Hello, world!</h1>
        </body>
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
