#![deny(warnings)]

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};

async fn hello(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut builder = Response::builder();
    if req.uri().path() == "/" {
        return Ok(builder
                  .header("Content-Type", "text/html")
                  .body(Body::from(r#"Hello!
<ul>
<li><a href="/test.xml">/xml</a></li>
<li><a href="/att/test.xml">/att/xml</a></li>
<li><a href="/image.png">/img</a></li>
<li><a href="/att/image.png">/att/img</a></li>
</ul>"#)).unwrap())
    }

    if req.uri().path().starts_with("/att/") {
        builder = builder.header("Content-Disposition", "attachment");
    }

    if req.uri().path().ends_with("/test.xml") {
        return Ok(builder
                  .header("Content-Type", "text/xml")
                  .body(Body::from(r#"<?xml version = "1.0" encoding = "utf-8"?>

<something>
</something>
"#)).unwrap());
    }
    if req.uri().path().ends_with("/test.png") {
       return Ok(builder
                 .header("Content-Type", "image/png")
                 .body(Body::from("Hello World!")).unwrap())
    }

    let status = StatusCode::NOT_FOUND;
    Ok(builder.status(status).body(Body::from(status.canonical_reason().unwrap())).unwrap())
}

#[tokio::main]
pub async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([0, 0, 0, 0], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
