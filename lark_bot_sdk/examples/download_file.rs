use std::sync::OnceLock;

use dotenv_codegen::dotenv;
use futures_util::StreamExt;
use lark_bot_sdk::api::file::download_file::DownloadFileReq;
use lark_bot_sdk::core::{DefaultLarkClient, Lark};
use lark_bot_sdk::error::Error::ErrHttpCode;
use tokio::io::AsyncWriteExt;

fn client() -> &'static DefaultLarkClient {
    static CLIENT: OnceLock<DefaultLarkClient> = OnceLock::new();

    CLIENT.get_or_init(|| Lark::new(env::var("app_id").unwrap_or(String::new()), env::var("app_secret").unwrap_or(String::new())))
}

// 注意，下载接口的错误类型为 **ErrHttpCode**
#[tokio::main]
async fn main() {
    let req = DownloadFileReq {
        file_key: env::var("download_file_id").unwrap_or(String::new()).into(),
    };
    let (mut resp, common_resp) = match client().file().download_file(req).await {
        Ok(resp) => resp,
        Err(e) => {
            if let ErrHttpCode(base_resp, common_resp) = &e {
                println!("{:?}\n{:?}", base_resp, common_resp);
                return;
            } else {
                panic!("{:?}", e)
            }
        }
    };
    println!("{:?}", resp);
    println!("{:?}\n", common_resp);
    let mut file = tokio::fs::File::create(format!(
        "source/test_{}",
        resp.name.unwrap_or("empty".into())
    ))
    .await
    .unwrap();
    while let Some(data) = resp.data.next().await {
        file.write_all(&data.unwrap()).await.unwrap();
    }
    println!("Download file success");
}
