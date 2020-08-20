#![deny(warnings)]

use std::collections::HashMap;
use std::convert::{Infallible, TryInto};
use std::fs::File;
use std::io::Read;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};

use percent_encoding::percent_decode;

static FILE_TYPES: &[&str] = &[
    "html", "txt", "png", "pdf", "svg", "xml", "webp", "avif", "c",
];

fn error_response(
    message: Option<&'static str>,
    status: StatusCode,
) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .header("Content-Type", "text/plain; charset=UTF-8")
        .status(status)
        .body(Body::from(
            message.unwrap_or_else(|| status.canonical_reason().unwrap_or("")),
        ))
        .unwrap())
}

async fn handler(
    req: Request<Body>,
    files: &HashMap<&str, &'static [u8]>,
) -> Result<Response<Body>, Infallible> {
    if req.uri().path() == "/" {
        return Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from(
                r#"<!doctype html><html>
<head>
<title>File type tests</title>
<link rel="icon" type="image/png" href="data:;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQAQAAAAA3iMLMAAAAH0lEQVR4AWP8zwBCD9RB6IM8FP1Qh6I/zlD0rxGiEgClfRHhbGAkhQAAAABJRU5ErkJggg=="/>
</head>
<body>
<h1>File type tests</h1>
<h2>Prebuilt</h2>
<ul>
<li>content-disposition: attachment
    <ul>
    <li>default content-type and extension
        <ul>
        <li><a href="https://mime.ty.ax/dl/test.pdf?ty=pdf&ct=application%2Fpdf&cd=attachment">PDF</a></li>
        <li><a href="https://mime.ty.ax/dl/test.xml?ty=xml&ct=text%2Fxml&cd=attachment">XML</a></li>
        <li><a href="https://mime.ty.ax/dl/test.svg?ty=svg&ct=image%2Fsvg%2Bxml&cd=attachment">SVG</a></li>
        <li><a href="https://mime.ty.ax/dl/test.webp?ty=webp&ct=image%2Fwebp&cd=attachment">WebP</a></li>
        <li><a href="https://mime.ty.ax/dl/test.avif?ty=webp&ct=image%2Favif&cd=attachment">AVIF</a></li>
        </ul>
    </li>
    <li><a href="https://mime.ty.ax/dl/test.pdf?ty=pdf&ct=application%2Foctet-stream&cd=attachment">.pdf application/octet-stream</a></li>
    <li><a href="https://mime.ty.ax/dl/test.svg?ty=svg&ct=application%2Foctet-stream&cd=attachment">.svg application/octet-stream</a></li>
    <li><a href="https://mime.ty.ax/dl/test.bin?ty=webp&ct=image%2Fwebp&cd=attachment">.bin WebP</a></li>
    </ul>
</li>
</ul>
<hr>
<h2>Custom</h2>
<form id="form">
<p>
<label for="file-type">File Type</label><br>
<select id="file-type" size=9>
  <option value="html">HTML</option>
  <option value="txt">TXT</option>
  <option value="png">PNG</option>
  <option value="pdf" selected>PDF</option>
  <option value="svg">SVG</option>
  <option value="xml">XML</option>
  <option value="webp">WebP</option>
  <option value="avif">AVIF</option>
  <option value="c">C source</option>
</select>
</p>

<p>
<label for="extension">File Extension<label><br>
<select id="extension" size=5>
  <option value="by-type" id="extension-by-type-option" selected></option>
  <option value=".bin">.bin</option>
  <option value=".png">.png</option>
  <option value=".txt">.txt</option>
  <option value="">&lt;none&gt;</option>
</select>
</p>

<p>
<label for="content-type">content-type</label><br>
<select id="content-type" size=7>
  <option value="by-type" id="content-type-by-type-option" selected></option>
  <option value="none">&lt;none&gt;</option>
  <option value="application/octet-stream">application/octet-stream</option>
  <option value="application/xml">application/xml</option>
  <option value="text/plain">text/plain</option>
  <option value="text/x-c">text/x-c</option>
  <option value="text/x-csrc">text/x-csrc</option>
</select>
</p>

<p>
<label for="content-disposition">content-disposition</label><br>
<select id="content-disposition" size=3>
  <option value="none">&lt;none&gt;</option>
  <option value="attachment" selected>attachment</option>
  <option value="attachment-name" id="attachment-name-option"></option>
</select>
</form>
</p>

<a id="file-link" href="about:blank"></a>

<script>
"use strict";

let form = document.getElementById("form");
let allInputs = ["file-type",    "extension",    "content-type",   "content-disposition"].map(
     n => document.getElementById(n));
let [fileTypeInput, extensionInput, contentTypeInput, contentDispositionInput] = allInputs;
let extensionByTypeOption = document.getElementById("extension-by-type-option");
let contentTypeByTypeOption = document.getElementById("content-type-by-type-option");
let attachmentNameOption = document.getElementById("attachment-name-option");
let fileLink = document.getElementById("file-link");

let kv = function(key, keyArray, valArray) {
    let idx = keyArray.indexOf(key);
    if (keyArray.length !== valArray.length) {
        throw new Error("array size mismatch");
    }
    if (idx == -1) {
        throw new Error(key + " not found");
    }
    return valArray[idx];
};

let types = [
    {
        key: "html",
        ext: ".html",
        ct: "text/html",
    },
    {
        key: "txt",
        ext: ".txt",
        ct: "text/plain",
    },
    {
        key: "png",
        ext: ".png",
        ct: "image/png",
    },
    {
        key: "pdf",
        ext: ".pdf",
        ct: "application/pdf",
    },
    {
        key: "svg",
        ext: ".svg",
        ct: "image/svg+xml",
    },
    {
        key: "xml",
        ext: ".xml",
        ct: "text/xml",
    },
    {
        key: "webp",
        ext: ".webp",
        ct: "image/webp",
    },
    {
        key: "avif",
        ext: ".avif",
        ct: "image/avif",
    },
    {
        key: "c",
        ext: ".c",
        ct: "text/x-c",
    },
];

let typeKeys = [];
let extensions = [];
let contentTypes = [];
for (const {key, ext, ct} of types) {
    typeKeys.push(key);
    extensions.push(ext);
    contentTypes.push(ct);
}

extensions.forType = ty => kv(ty, typeKeys, extensions);
contentTypes.forType = ty => kv(ty, typeKeys, contentTypes);

let handleChange = function() {
    let fileType = fileTypeInput.value;
    let defaultExtension = extensions.forType(fileType);
    let extension = extensionInput.value;
    let defaultContentType = contentTypes.forType(fileType);
    let contentType = contentTypeInput.value;
    if (extension === "by-type") {
        extension = defaultExtension;
    }
    if (contentType === "by-type") {
        contentType = defaultContentType;
    }
    let fileName = `test${extension}`;
    let attachmentNameStr = `attachment; filename="${fileName}"`;

    let contentDisposition = contentDispositionInput.value;
    if (contentDisposition === "attachment-name") {
        contentDisposition = attachmentNameStr;
        fileName = `shouldBeIgnored`;
    }

    extensionByTypeOption.textContent = `by type (${defaultExtension})`;
    contentTypeByTypeOption.textContent = `by type (${defaultContentType})`;
    attachmentNameOption.textContent = attachmentNameStr;

    let link = `/dl/${fileName}?ty=${encodeURIComponent(fileType)}&ct=${encodeURIComponent(contentType)}&cd=${encodeURIComponent(contentDisposition)}`;

    fileLink.href = link;
    fileLink.textContent = link;
};
allInputs.map(el => el.addEventListener('change', handleChange));

handleChange();

</script>"#,
            ))
            .unwrap());
    }

    if !req.uri().path().starts_with("/dl/") {
        return error_response(None, StatusCode::NOT_FOUND);
    }

    let query: Result<_, &'static str> =
        req.uri()
            .query()
            .ok_or("missing query")
            .and_then(|query_str| {
                let mut query_parts: HashMap<&str, String> = query_str
                    .split('&')
                    .map(|component| {
                        let parts_vec: Vec<&str> = component.split('=').collect();
                        if parts_vec.len() != 2 {
                            Err("malformed query")
                        } else {
                            let val = percent_decode(parts_vec[1].as_bytes())
                                .decode_utf8()
                                .map_err(|_| "decode failed")
                                .map(|cow| cow.into_owned())?;
                            Ok((parts_vec[0], val))
                        }
                    })
                    .collect::<Result<_, _>>()?;
                let mut vals = ["ty", "ct", "cd"]
                    .iter()
                    .map(|k| query_parts.remove(k).ok_or("missing query part"))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter();
                if query_parts.len() > 0 {
                    return Err("extra query parts".into());
                }
                Ok((
                    vals.next().unwrap(),
                    vals.next().unwrap(),
                    vals.next().unwrap(),
                ))
            });
    if let Err(e) = query {
        return error_response(Some(e), StatusCode::BAD_REQUEST);
    }
    let (file_type, content_type, content_disposition) = query.unwrap();

    // TODO: check for reasonable values here
    let mut builder = Response::builder();
    if content_disposition != "none" {
        builder = builder.header("Content-Disposition", content_disposition);
    }
    if content_type != "none" {
        builder = builder.header("Content-Type", content_type);
    }

    let content = files.get(file_type.as_str());
    if content.is_none() {
        return error_response(Some("Bad file type"), StatusCode::BAD_REQUEST);
    }
    let content = content.unwrap();

    builder
        .body(Body::from(*content))
        .or_else(|_| error_response(None, StatusCode::INTERNAL_SERVER_ERROR))
}

#[tokio::main]
pub async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let files = FILE_TYPES
        .iter()
        .map(
            |ty: &&str| -> Result<(&str, &'static [u8]), Box<dyn std::error::Error + Send + Sync>> {
                let file_name = format!("files/example.{}", ty);
                let mut file = File::open(&file_name)
                    .map_err(|e| format!("failed opening {}: {}", &file_name, e))?;
                let mut bytes = Vec::with_capacity(file.metadata()?.len().try_into()?);
                file.read_to_end(&mut bytes)?;
                Ok((*ty, Box::leak(bytes.into_boxed_slice())))
            },
        )
        .collect::<Result<_, _>>()?;
    let files: &'static HashMap<&str, &'static [u8]> = Box::leak(Box::new(files));

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(move |_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async move { Ok::<_, Infallible>(service_fn(move |req| handler(req, files))) }
    });

    let addr = ([127, 0, 0, 1], 4000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
