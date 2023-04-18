use axum::{
    routing::get,
    Router, extract::Path, body::StreamBody, response::IntoResponse,
};

async fn handler(Path(file): Path<String>) -> impl IntoResponse {
    // The real url we have to download the data from
    let url = format!("https://github.com/OpenBangla/OpenBangla-Keyboard/releases/download/2.0.0/{file}");

    let res = reqwest::get(url).await.unwrap();

    let stream = res.bytes_stream();

    let stream = StreamBody::new(stream);

    stream
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/file/:file", get(handler));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}