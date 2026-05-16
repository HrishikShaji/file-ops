// mod api;
mod async_api;
// mod csv;
// mod json;

use async_api::Result;

use reqwest::Client;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<()> {
    // api::fetch_api();
    //async_api::fetch().await;
    //csv::read_csv_file();
    //json::create_json();
    //json::parse_json();
    // async_api::fetch_github_stargazers().await?;

    async_api::auth_check().await?;

    Ok(())
}
