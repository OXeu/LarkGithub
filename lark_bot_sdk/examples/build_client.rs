use std::time::Duration;

use dotenv_codegen::dotenv;
use lark_bot_sdk::core::{http_client::DefaultClient, store::RWStoreMemory, LarkBuilder};

#[tokio::main]
async fn main() {
    let _client = LarkBuilder::default()
        .timeout(Duration::from_secs(10))
        .is_isv(false)
        .normal()
        .build(env::var("app_id").unwrap_or(String::new()), env::var("app_secret").unwrap_or(String::new()));

    let _custom_client = LarkBuilder::default().build_with_store_and_client(
        RWStoreMemory::default(),
        DefaultClient::default(),
        env::var("app_id").unwrap_or(String::new()),
        env::var("app_secret").unwrap_or(String::new()),
    );
}
