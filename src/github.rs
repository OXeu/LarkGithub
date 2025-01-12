use std::env;

use chrono::{DateTime, Utc};
use octocrab::{models::issues::Issue, Octocrab};
use tracing::info;

use crate::{issue::IssueReq, utils::fetch_image};
pub async fn create_issue(issue: IssueReq) -> Result<(u64, String), octocrab::Error> {
    let octocrab = init_gh().await;
    let owner = env::var("GH_OWNER").unwrap_or(String::new());
    let repo = env::var("GH_REPO").unwrap_or(String::new());
    let content = fetch_image(&issue.content).await;
    let result = octocrab
        .issues(owner, repo)
        .create(issue.title)
        .body(content)
        .labels(Some(issue.label))
        .send()
        .await?;
    return Ok((result.number, result.title));
}

pub async fn update_issues(id: u64, issue: IssueReq) -> Result<(), octocrab::Error> {
    let octocrab = init_gh().await;
    let owner = env::var("GH_OWNER").unwrap_or(String::new());
    let repo = env::var("GH_REPO").unwrap_or(String::new());
    let force_update_user = env::var("FORCE_UPDATE_USER").unwrap_or(String::new()) == "true";
    let content = fetch_image(&issue.content).await;
    let issue_now: Issue = octocrab.issues(owner.clone(), repo.clone()).get(id).await?;
    if (issue_now.user.r#type != "Bot" || issue_now.user.r#type == "User") && !force_update_user {
        info!("💦 跳过更新: issue 由用户创建");
        return Ok(());
    }
    if force_update_user {
        info!("🐢 强制更新(包括用户创建的 issue)");
    }
    octocrab
        .issues(owner, repo)
        .update(id)
        .title(&issue.title)
        .body(&content)
        .labels(&issue.label)
        .send()
        .await?;
    return Ok(());
}

pub async fn fetch_issue_updated_time(id: u64) -> Result<DateTime<Utc>, octocrab::Error> {
    let octocrab = init_gh().await;
    let owner = env::var("GH_OWNER").unwrap_or(String::new());
    let repo = env::var("GH_REPO").unwrap_or(String::new());
    let result: Issue = octocrab.issues(owner, repo).get(id).await?;
    return Ok(result.updated_at);
}

pub async fn init_gh() -> Octocrab {
    let token = env::var("GITHUB_TOKEN").unwrap_or(String::new());
    octocrab::Octocrab::builder()
        .personal_token(token.into())
        .build()
        .unwrap()
}

#[tokio::test]
async fn test() {
    tracing_subscriber::fmt::init();
    let _ = dotenvy::dotenv();
    let octocrab = init_gh().await;
    let owner = env::var("GH_OWNER").unwrap_or(String::new());
    let repo = env::var("GH_REPO").unwrap_or(String::new());
    info!("Owner: {}, Repo: {}", owner, repo);
    let issue_now: Issue = octocrab
        .issues(owner.clone(), repo.clone())
        .get(105)
        .await
        .unwrap();
    info!("Issue: {:#?}", issue_now);
}
