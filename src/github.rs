use chrono::{DateTime, Utc};
use octocrab::{models::issues::Issue, Octocrab};

use crate::{issue::IssueReq, utils::fetch_image};
pub async fn create_issue(issue: IssueReq) -> Result<u64, octocrab::Error> {
    let octocrab = init_gh().await;
    let owner = dotenv!("GH_OWNER");
    let repo = dotenv!("GH_REPO");
    let content = fetch_image(&issue.content).await;
    let result = octocrab
        .issues(owner, repo)
        .create(issue.title)
        .body(content)
        .labels(Some(issue.label))
        .send()
        .await?;
    return Ok(result.number);
}

pub async fn update_issues(id: u64, issue: IssueReq) -> Result<(), octocrab::Error> {
    let octocrab = init_gh().await;
    let owner = dotenv!("GH_OWNER");
    let repo = dotenv!("GH_REPO");
    let content = fetch_image(&issue.content).await;
    octocrab
        .issues(owner, repo)
        .update(id)
        .body(&content)
        .labels(&issue.label)
        .send()
        .await?;
    return Ok(());
}

pub async fn fetch_issue_updated_time(id: u64) -> Result<DateTime<Utc>, octocrab::Error> {
    let octocrab = init_gh().await;
    let owner = dotenv!("GH_OWNER");
    let repo = dotenv!("GH_REPO");
    let result: Issue = octocrab.issues(owner, repo).get(id).await?;
    return Ok(result.updated_at);
}

pub async fn init_gh() -> Octocrab {
    let token = dotenv!("GITHUB_TOKEN");
    octocrab::Octocrab::builder()
        .personal_token(token.into())
        .build()
        .unwrap()
}
