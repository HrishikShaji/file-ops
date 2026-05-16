// mod api;
// mod async_api;
// mod csv;
// mod json;

use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // api::fetch_api();
    //async_api::fetch().await;
    //csv::read_csv_file();
    //json::create_json();
    //json::parse_json();

    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );

    println!("{}", request_url);

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "rust-web-api-client-demo")
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    println!("Users : {:?}", users);
    Ok(())
}
