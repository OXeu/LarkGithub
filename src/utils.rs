use bytes::BufMut;
use lark_bot_sdk::api::drive::download_drive_media::DownloadDriveMediaReq;
use regex::Regex;
use reqwest::multipart::Part;
use serde_json::Value;
use tracing::error;

use crate::lark::client;

pub fn for_string(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(true) => String::from("true"),
        Value::Bool(false) => String::from("false"),
        Value::Number(number) => number.to_string(),
        Value::String(s) => s.to_owned(),
        Value::Array(vec) => vec
            .iter()
            .map(|v| for_string(v))
            .reduce(|sum, s: String| sum + &s)
            .unwrap_or(String::new()),
        Value::Object(map) => {
            if map.contains_key("text") && map.contains_key("type") {
                for_string(map.get("text").unwrap())
            } else if map.contains_key("text") && map.contains_key("link") {
                for_string(map.get("link").unwrap())
            } else if map.contains_key("file_token") {
                let token = for_string(map.get("file_token").unwrap());
                let name = for_string(map.get("name").unwrap_or(&Value::Null));
                let typ = for_string(map.get("type").unwrap());
                if typ.starts_with("image/") {
                    // 图片
                    return format!("@{}![{}]({})", typ, name, token);
                }
                format!("[{}](https://internal-api-drive-stream.feishu.cn/space/api/box/stream/download/preview/{}?preview_type=16)", name, token)
            } else {
                map.iter()
                    .map(|(k, v)| format!("{}: {}", k, for_string(v)))
                    .reduce(|sum, s| format!("{}\n{}", sum, s))
                    .unwrap_or(String::new())
            }
        }
    }
}
use futures_util::StreamExt;
pub async fn fetch_image(content: &str) -> String {
    let re = Regex::new("@([^!]*)!\\[([^\\]]*)\\]\\(([^)]+)\\)").unwrap();
    let mut content_result = content.to_string();
    for (full, [typ, name, token]) in re.captures_iter(&content).map(|c| c.extract()) {
        let typ_str = typ.to_string();
        let name_str = name.to_string();
        match client()
            .drive()
            .download_drive_media(DownloadDriveMediaReq {
                file_token: token.into(),
                ..Default::default()
            })
            .await
        {
            Ok((value, _)) => {
                let mut buf = vec![];
                let mut data = value.data;
                while let Some(data) = data.next().await {
                    let bytes = &data.unwrap()[..];
                    buf.put_slice(bytes);
                }
                let client = reqwest::Client::builder()
                    .user_agent(
                        "curl/8.10.1",
                    )
                    .build().unwrap();
                let part = Part::stream(buf).file_name(name_str.clone()).mime_str(&typ_str).unwrap();
                let form = reqwest::multipart::Form::new().part("file", part);
                match client.post("https://0x0.st").multipart(form).send().await {
                    Ok(resp) => {
                        let url = resp.text().await.unwrap();
                        let img = format!(r#"![{}]({})"#, name_str, url.trim());
                        content_result = content_result.replace(full, &img);
                    }
                    Err(err) => {
                        error!("Upload image failed, {:#?}", err)
                    }
                }
            }
            Err(err) => error!("Download Resource Failed, {:?}", err),
        }
    }
    content_result
}

#[tokio::test]
async fn test_fetch_image() {
    let content = r#"
    {需求描述:无}

    ## 详细描述
     {需求详细描述（可附文档）:无}

    @image/jpeg![b4858e94459b3648cbe43ca05b3a8972.jpg](IpZfb8SAKorlf0xwOgNch69enDh)

    --- 
    于 **{@需求提出日期}** 创建
    "#;
    println!("{}", fetch_image(content.into()).await);
    assert!(false);
}
