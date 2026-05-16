use error_chain::error_chain;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub async fn fetch() -> Result<()> {
    let res = reqwest::get("https://httpbin.org/get").await?;
    println!("Async Status :{}", res.status());
    println!("Async Headers :\n {:#?}", res.headers());
    let body = res.text().await?;
    println!("Async Body :\n {}", body);

    Ok(())
}
