mod api;
mod async_api;
mod csv;
mod json;

#[tokio::main]
async fn main() {
    // api::fetch_api();
    async_api::fetch().await;

    csv::read_csv_file();

    json::create_json();

    json::parse_json();
}
