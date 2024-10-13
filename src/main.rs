use github::init_gh;
use lark::fetch_records;
extern crate dotenvy;
extern crate tokio;
#[macro_use]
extern crate dotenvy_macro;
mod github;
mod lark;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().unwrap();
    init_gh();
    let records = fetch_records().await;
}
