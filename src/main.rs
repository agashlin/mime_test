#![deny(warnings)]

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};

async fn hello(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut builder = Response::builder();
    if req.uri().path() == "/" {
        return Ok(builder
            .header("Content-Type", "text/html")
            .body(Body::from(
                r#"Hello!
<ul>
<li><a href="/test.html">/html</a></li>
<li><a href="/att/test.html">/att/html</a></li>
<li><a href="/test.shtml">/shtml</a></li>
<li><a href="/att/test.shtml">/att/shtml</a></li>
<li><a href="/test.blah">/blah (text/html)</a></li>
<li><a href="/att/test.blah">/att/blah (text/html)</a></li>
<li><a href="/test.blah2">/blah2 (application/pdf)</a></li>
<li><a href="/att/test.blah2">/att/blah2 (application/pdf)</a></li>
<li><a href="/test.blah3">/blah3 (application/octet-stream)</a></li>
<li><a href="/att/test.blah3">/att/blah3 (application/octet-stream)</a></li>
<li><a href="/test.xml">/xml</a></li>
<li><a href="/att/test.xml">/att/xml</a></li>
<li><a href="/test-app.xml">/app-xml</a></li>
<li><a href="/att/test-app.xml">/att/app-xml</a></li>
<li><a href="/test.png">/png</a></li>
<li><a href="/att/test.png">/att/png</a></li>
<li><a href="/nomime/test.png">/nomime/png</a></li>
<li><a href="/att/nomime/test.png">/att/nomime/img</a></li>
<li><a href="/test.svg">/svg</a></li>
<li><a href="/att/test.svg">/att/svg</a></li>
<li><a href="/test.webp">/webp</a></li>
<li><a href ="/att/test.webp">/att/webp</a></li>
<li><a href="/test.pdf">/pdf</a></li>
<li><a href="/att/test.pdf">/att/pdf</a></li>
<li><a href="/nomime/test.pdf">/nomime/pdf</a></li>
<li><a href="/att/nomime/test.pdf">/att/nomime/pdf</a></li>
<li><a href="/test.c">/c (text/x-c)</a></li>
<li><a href="/att/test.c">/att/c (text/x-c)</a></li>
<li><a href="/test-csrc.c">/c (text/x-csrc)</a></li>
<li><a href="/att/test-csrc.c">/att/c (text/x-csrc)</a></li>
</ul>"#,
            ))
            .unwrap());
    }

    if req.uri().path().starts_with("/att/") {
        builder = builder.header("Content-Disposition", "attachment");
    }

    if req.uri().path().ends_with("/test.html") {
        return Ok(builder
            .header("Content-Type", "text/html")
            .body(Body::from("Hello"))
            .unwrap());
    }

    if req.uri().path().ends_with("/test.shtml") {
        return Ok(builder
            .header("Content-Type", "text/html")
            .body(Body::from("Hello"))
            .unwrap());
    }

    if req.uri().path().ends_with("/test.blah") {
        return Ok(builder
            .header("Content-Type", "text/html")
            .body(Body::from("Hello"))
            .unwrap());
    }

    if req.uri().path().ends_with("/test.blah2") {
        return Ok(builder
            .header("Content-Type", "application/pdf")
            .body(Body::from("Hello"))
            .unwrap());
    }

    if req.uri().path().ends_with("/test.blah3") {
        return Ok(builder
            .header("Content-Type", "application/octet-stream")
            .body(Body::from("Hello"))
            .unwrap());
    }

    if req.uri().path().ends_with("/test.xml") {
        return Ok(builder
            .header("Content-Type", "text/xml")
            .body(Body::from(
                r#"<?xml version = "1.0" encoding = "utf-8"?>

<something>
</something>
"#,
            ))
            .unwrap());
    }

    if req.uri().path().ends_with("/test-app.xml") {
        return Ok(builder
            .header("Content-Type", "application/xml")
            .body(Body::from(
                r#"<?xml version = "1.0" encoding = "utf-8"?>

<something>
</something>
"#,
            ))
            .unwrap());
    }

    if req.uri().path().ends_with("/nomime/test.png") {
        return Ok(builder.body(Body::from("Hello World!")).unwrap());
    }

    if req.uri().path().ends_with("/nomime/test.pdf") {
        return Ok(builder.body(Body::from("Hello World!")).unwrap());
    }

    if req.uri().path().ends_with("/test.png") {
        return Ok(builder
            .header("Content-Type", "image/png")
            .body(Body::from("Hello World!"))
            .unwrap());
    }

    if req.uri().path().ends_with("/test.svg") {
        return Ok(builder
            .header("Content-Type", "image/svg+xml")
            .body(Body::from("Hello World!"))
            .unwrap());
    }

    if req.uri().path().ends_with("/test.webp") {
        return Ok(builder
            .header("Content-Type", "image/webp")
            .body(Body::from("Hello World!"))
            .unwrap());
    }

    if req.uri().path().ends_with("/test.pdf") {
        return Ok(builder
            .header("Content-Type", "application/pdf")
            .body(Body::from("Hello World!"))
            .unwrap());
    }

    if req.uri().path().ends_with("/test.c") {
        return Ok(builder
            .header("Content-Type", "text/x-c")
            .body(Body::from("Hello World!"))
            .unwrap());
    }

    if req.uri().path().ends_with("/test-csrc.c") {
        return Ok(builder
            .header("Content-Type", "text/x-csrc")
            .body(Body::from("Hello World!"))
            .unwrap());
    }


    let status = StatusCode::NOT_FOUND;
    Ok(builder
        .status(status)
        .body(Body::from(status.canonical_reason().unwrap()))
        .unwrap())
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
