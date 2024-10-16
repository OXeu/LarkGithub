use std::env;

use reqwest::multipart::Part;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResp {
    pub data: UploadRespData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadRespData {
    pub url: String,
}

pub async fn upload(buf: Vec<u8>, name: &str, typ: &str) -> String {
    let img_uploader = env::var("IMG_URL").unwrap_or(String::from("https://sm.ms/api/v2/upload"));
    let file_field = env::var("IMG_FILE_FIELD").unwrap_or(String::from("smfile"));
    let token = env::var("IMG_AUTH_TOKEN").unwrap_or(String::new());

    let client = reqwest::Client::builder()
        .user_agent("curl/8.10.1")
        .build()
        .unwrap();
    let part = Part::stream(buf)
        .file_name(name.to_string())
        .mime_str(typ)
        .unwrap();
    let form = reqwest::multipart::Form::new().part(file_field, part);
    match client
        .post(img_uploader)
        .header("Authorization", token)
        .multipart(form)
        .send()
        .await
    {
        Ok(resp) => {
            let url = resp.json::<UploadResp>().await.unwrap().data.url;
            format!(r#"![{}]({})"#, name, url)
        }
        Err(err) => {
            error!("Upload image failed, {:#?}", err);
            format!("![{}]({:?})", name, err)
        }
    }
}
