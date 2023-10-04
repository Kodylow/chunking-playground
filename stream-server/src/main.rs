use std::{convert::Infallible, time::Duration};

use axum::{routing::get, Router};
use futures_util::stream::{self};
use hyper::{Body, Response, Server};
use tokio::time::sleep;

async fn stream_response() -> Result<Response<Body>, Infallible> {
    let chunked_body = vec![
        "Hello",
        ", ",
        "world",
        "! ",
        "This",
        " is",
        " a",
        " much",
        " longer",
        " chunked",
        " body",
        " for",
        " the",
        " stream",
        ".",
        " It",
        " should",
        " provide",
        " a",
        " more",
        " realistic",
        " example",
        " of",
        " how",
        " this",
        " might",
        " work",
        " in",
        " a",
        " real",
        " world",
        " application",
        ".",
    ];
    let stream = stream::unfold(chunked_body.into_iter(), |mut chunks| async move {
        if let Some(chunk) = chunks.next() {
            sleep(Duration::from_millis(500)).await;
            Some((Ok::<_, Infallible>(chunk), chunks))
        } else {
            None
        }
    });
    let body = Body::wrap_stream(stream);
    Ok(Response::new(body))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(stream_response));

    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Server is running on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
