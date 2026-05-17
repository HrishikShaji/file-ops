mod scrape;

#[tokio::main]
async fn main() {
    scrape::scrape_site().await;
}
