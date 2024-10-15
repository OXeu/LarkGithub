use github::{create_issue, fetch_issue_updated_time, init_gh, update_issues};
use lark::{bind_issue, fetch_records, format_record, get_issue_id};
use tracing::{error, info};
extern crate dotenvy;
extern crate tokio;
#[macro_use]
extern crate dotenvy_macro;
mod github;
mod issue;
mod lark;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    if cfg!(debug_assertions) {
        dotenvy::dotenv().unwrap();
    }
    init_gh().await;
    let update_enable = dotenv!("ISSUE_UPDATE") == "true";
    let records = fetch_records().await;
    info!("✨ 获取到 {:#?} 条记录", records.len());
    info!("");
    for record in records.iter() {
        let issue = format_record(&record);
        let issue_title = issue.title.clone();
        info!("🤔 {}", issue_title);
        if let Some(id) = get_issue_id(record) {
            if update_enable {
                match fetch_issue_updated_time(id).await {
                    Ok(time) => {
                        if record.last_modified_time > time.timestamp_millis() {
                            if let Err(err) = update_issues(id, issue).await {
                                error!("😰 更新失败：{:?}\n{:?}", record, err);
                            } else {
                                info!("😎 更新成功");
                            }
                        } else {
                            info!("🦜 已是最新");
                        }
                    }
                    Err(err) => error!("😰 获取 issue(#{}):{} 失败 {:#?}", id, issue_title, err),
                }
            } else {
                info!("🫢 更新已关闭");
            }
        } else {
            match create_issue(issue).await {
                Ok(id) => {
                    if let Err(err) = bind_issue(&record.record_id, id).await {
                        error!(
                            "😰 绑定失败 issue(#{}):{} \n{:?}\n{:?}",
                            id, issue_title, record, err
                        );
                    } else {
                        info!("😎 创建成功 issue(#{}):{} ", id, issue_title);
                    }
                }
                Err(err) => error!("😰 创建失败 issue: {} ：{:?}", issue_title, err),
            }
        }
        info!("")
    }
    info!("🎉同步完成")
}
