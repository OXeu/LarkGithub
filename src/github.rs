use jsonwebtoken::EncodingKey;
use octocrab::{auth::{AppAuth, Auth}, params, Octocrab};
use tracing::info;

use crate::utils::decode_hex;
pub async fn create_issue(title: String, content: String) {
    let octocrab = octocrab::instance();
    let owner = dotenv!("GH_OWNER");
    let repo = dotenv!("GH_REPO");
    let page = octocrab
        .issues("OXeu", "vcb_cn")
        .list()
        .state(params::State::Closed)
        .page(50u32)
        .send()
        .await;
    println!("Issues: {:?}", page);
    let result = octocrab
        .issues(owner, repo)
        .create(title)
        .body(content)
        .send()
        .await
        .expect(&format!("在 {}/{} 中创建 issue 失败", owner, repo));
    info!("创建 issus 成功：{:?}", result);
}

pub fn init_gh() {
    let gh_app_id = dotenv!("GH_APP_ID");
    let gh_app_id_hex = gh_app_id.parse().expect(&format!(
        "Expect number for github app id, but got {}",
        gh_app_id
    ));
    let gh_app_key = dotenv!("GH_APP_SECRETS");
    let hex_key = decode_hex(gh_app_key).expect(&format!(
        "Expect hex string for github app secret, but got {}",
        gh_app_key
    ));
    let octocrab = Octocrab::builder()
    .with_auth(Auth::App(AppAuth {
        app_id: octocrab::models::AppId(gh_app_id_hex),
        key: EncodingKey::from_secret(&hex_key),
    }))
    .build();
}

#[tokio::test]
async fn test_gh_create_issue() {
    init_gh();
    create_issue(
        "Test Issue".to_string(),
        "# Markdown Support Request\nPlease".to_string(),
    )
    .await;
}
