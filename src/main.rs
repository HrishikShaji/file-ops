// mod api;
// mod async_api;
// mod csv;
// mod json;

// use async_api::Result;

// use reqwest::Client;
// use reqwest::Error;

mod download;

#[tokio::main]
async fn main() {
    download::download_image().await;
}
