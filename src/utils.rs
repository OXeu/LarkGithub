use bytes::BufMut;
use lark_bot_sdk_patch::api::drive::download_drive_media::DownloadDriveMediaReq;
use regex::Regex;
use serde_json::Value;
use tracing::error;

use crate::{lark::client, uploader::upload};

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
                let img = upload(buf, name, typ).await;
                content_result = content_result.replace(full, &img);

            }
            Err(err) => error!("Download Resource Failed, {:?}", err),
        }
    }
    content_result
}

#[tokio::test]
async fn test_fetch() {
        let _ = dotenvy::dotenv();
        tracing_subscriber::fmt::init();
    let content = r#"## 需求描述
图片大小与缓存优化 

## 详细描述
 随即打开一次首页，3张图大小超过2M，加载时间在1秒以上，且看起来没有本地缓存//图片不是存在后端的，放了图床

 @image/png![图片.png](NH9tbiGS1oN4MPxoQjzcZYH9nNc)

--- 
于 **2024/10/15 16:00** 创建
"#;
    let result = fetch_image(content).await;
    println!("{}", result);
}
