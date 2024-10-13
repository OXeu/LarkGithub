use std::sync::OnceLock;

use lark_bot_sdk::{
    api::bitable::search_bitable_record::{
        AppTableRecordSubResp, ConditionSubReq, FilterInfoSubReq, SearchBitableRecordReq,
    },
    core::{DefaultLarkClient, Lark},
};
fn client() -> &'static DefaultLarkClient {
    static CLIENT: OnceLock<DefaultLarkClient> = OnceLock::new();
    let app_id = dotenv!("LG_APP_ID");
    let app_secret = dotenv!("LG_APP_SECRET");
    CLIENT.get_or_init(|| Lark::new(app_id, app_secret))
}

pub async fn fetch_records() -> Vec<AppTableRecordSubResp> {
    let app_token = dotenv!("LG_BITABLE_TOKEN");
    let table_id = dotenv!("LG_BITABLE_TABLE_ID");
    let field_names = dotenv!("LG_BITABLE_FIELDS");
    let conjunction = dotenv!("LG_BITABLE_CONDITION_CONJUNCTION");
    let cond_str = dotenv!("LG_BITABLE_CONDITIONS");
    let fields: Vec<Option<String>> = field_names
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| Some(s.to_string()))
        .collect();
    let conds: Vec<&str> = cond_str.split(";").collect();
    let conditions: Vec<Option<ConditionSubReq>> = conds
        .into_iter()
        .map(|s: &str| {
            let mut v: Vec<&str> = s.split(",").collect();
            if v.len() < 3 {
                None
            } else {
                let field = v.remove(0);
                let op = v.remove(1);
                let value: Vec<Option<String>> =
                    v.into_iter().map(|s| Some(s.to_string())).collect();
                Some(ConditionSubReq {
                    field_name: field.to_string(),
                    operator: op.to_string(),
                    value: value,
                })
            }
        })
        .collect();
    let mut page_token = "".to_string();
    let mut req = SearchBitableRecordReq {
        app_token: app_token.to_string(),
        table_id: table_id.to_string(),
        user_id_type: "open_id".into(),
        page_size: 20,
        ..Default::default()
    };
    if !conjunction.is_empty() {
        req.filter = Some(FilterInfoSubReq {
            conjunction: Some(conjunction.to_string()),
            conditions: conditions.clone(),
        });
    }
    if !fields.is_empty() {
        req.field_names = fields.clone();
    }
    let mut records = Vec::new();
    loop {
        if !page_token.is_empty() {
            req.page_token = page_token;
        }
        let (resp, _) = client()
            .bitable()
            .search_bitable_record(req.clone())
            .await
            .expect(format!("获取多维表格数据失败\n 请求：{:?}\n响应：", req).as_str());
        page_token = resp.data.page_token;
        if !resp.data.has_more {
            break;
        }
        let items = resp.data.items;
        items.iter().for_each(|item| records.push(item.clone()));
    }
    records
}
