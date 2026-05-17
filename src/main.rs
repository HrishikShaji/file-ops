// mod api;
// mod async_api;
// mod csv;
// mod json;

// use async_api::Result;

// use reqwest::Client;
// use reqwest::Error;

use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // api::fetch_api();
    //async_api::fetch().await;
    //csv::read_csv_file();
    //json::create_json();
    //json::parse_json();
    // async_api::fetch_github_stargazers().await?;
    // async_api::auth_check().await?;

    let temp_dir = Builder::new().prefix("example").tempdir()?;

    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("File name to download : {}", fname);
        let fname = temp_dir.path().join(fname);
        println!("will be located under: {:?}", fname);

        File::create(fname)?
    };

    let content = response.text().await?;

    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}
