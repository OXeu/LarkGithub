use github::{create_issue, fetch_issue_updated_time, init_gh, update_issues};
use lark::{bind_issue, fetch_records, format_record, get_issue_id};
use std::env;
use tracing::{error, info};
use utils::parse_timestamp;
extern crate dotenvy;
extern crate tokio;
mod github;
mod issue;
mod lark;
mod uploader;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    if cfg!(debug_assertions) {
        let _ = dotenvy::dotenv();
    }
    init_gh().await;
    let update_enable = env::var("ISSUE_UPDATE").unwrap_or(String::new()) == "true";
    let force_update = env::var("FORCE_UPDATE").unwrap_or(String::new()) == "true";
    let bot_name = env::var("LARK_BOT_NAME").unwrap_or(String::new());
    let records = fetch_records().await;
    info!("✨ 获取到 {:#?} 条记录", records.len());
    info!("");
    for record in records.iter() {
        let issue = format_record(&record);
        let issue_title = issue.title.clone();
        info!("🤔 {}", issue_title);
        if let Some(id) = get_issue_id(record) {
            if !update_enable {
                info!("🐞 更新已关闭");
                continue;
            }
            if id == !0 {
                error!("🥀 非当前仓库 issue，跳过更新");
                continue;
            }
            match fetch_issue_updated_time(id).await {
                Ok(time) => {
                    let record_time = parse_timestamp(record.last_modified_time).unwrap();
                    if (record_time > time && record.last_modified_by.name != bot_name) || force_update {
                        if force_update {
                            info!("🐢 强制更新");
                        }
                        if let Err(err) = update_issues(id, issue).await {
                            error!("🥀 更新失败：{:?}\n{:?}", record, err);
                        } else {
                            info!("🐢 更新成功");
                        }
                    } else {
                        info!("🐤 已是最新");
                    }
                }
                Err(err) => error!("🥀 获取 issue(#{}):{} 失败 {:#?}", id, issue_title, err),
            }
        } else {
            match create_issue(issue).await {
                Ok((id, title)) => {
                    if let Err(err) = bind_issue(&record.record_id, id, title).await {
                        error!(
                            "🥀 绑定失败 issue(#{}):{} \n{:?}\n{:?}",
                            id, issue_title, record, err
                        );
                    } else {
                        info!("🐢 创建成功 issue(#{}):{} ", id, issue_title);
                    }
                }
                Err(err) => error!("🥀 创建失败 issue: {} ：{:?}", issue_title, err),
            }
        }
        info!("")
    }
    info!("🎉同步完成")
}
