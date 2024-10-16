use std::{collections::HashMap, env, sync::OnceLock};

use chrono::DateTime;
use lark_bot_sdk_patch::{
    api::bitable::{
        search_bitable_record::{
            AppTableRecordSubResp, ConditionSubReq, FilterInfoSubReq, SearchBitableRecordReq,
        },
        update_bitable_record::{UpdateBitableRecordReq, UpdateBitableRecordResp},
    },
    core::{model::CommonResponse, DefaultLarkClient, Lark},
};
use regex::Regex;
use serde_json::Map;
use tracing::{debug, warn};

use crate::{issue::IssueReq, utils::for_string};
pub fn client() -> &'static DefaultLarkClient {
    static CLIENT: OnceLock<DefaultLarkClient> = OnceLock::new();
    let app_id = env::var("LARK_APP_ID").unwrap_or(String::new());
    let app_secret = env::var("LARK_APP_SECRET").unwrap_or(String::new());
    CLIENT.get_or_init(|| Lark::new(app_id, app_secret))
}

pub async fn fetch_records() -> Vec<AppTableRecordSubResp> {
    let app_token = env::var("LARK_BITABLE_TOKEN").unwrap_or(String::new());
    let table_id = env::var("LARK_BITABLE_TABLE_ID").unwrap_or(String::new());
    let field_names = env::var("LARK_BITABLE_FIELDS").unwrap_or(String::new());
    let conjunction = env::var("LARK_BITABLE_CONDITION_CONJUNCTION").unwrap_or(String::new());
    let cond_str = env::var("LARK_BITABLE_CONDITIONS").unwrap_or(String::new());
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
            if v.len() < 2 {
                None
            } else if v.len() == 2 {
                let field = v.remove(0);
                let op = v.remove(0);
                Some(ConditionSubReq {
                    field_name: field.to_string(),
                    operator: op.to_string(),
                    value: vec![],
                })
            } else {
                let field = v.remove(0);
                let op = v.remove(0);
                let value: Vec<Option<String>> =
                    v.into_iter().map(|s| Some(s.to_string())).collect();
                Some(ConditionSubReq {
                    field_name: field.to_string(),
                    operator: op.to_string(),
                    value: value,
                })
            }
        })
        .filter(|v| v.is_some())
        .collect();
    let mut req = SearchBitableRecordReq {
        app_token: app_token.to_string(),
        table_id: table_id.to_string(),
        user_id_type: "open_id".into(),
        page_size: 500,
        automatic_fields: Some(true),
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
    debug!("请求: {:#?}", req);
    let mut records = Vec::new();
    loop {
        let (resp, _) = client()
            .bitable()
            .search_bitable_record(req.clone())
            .await
            .expect(format!("😰 获取多维表格数据失败\n 请求：{:?}\n响应：", req).as_str());
        let items = resp.data.items;
        items.iter().for_each(|item| records.push(item.clone()));
        req.page_token = resp.data.page_token;
        if req.page_token.is_empty() {
            break;
        }
    }
    records
}

pub fn get_issue_id(record: &AppTableRecordSubResp) -> Option<u64> {
    let issue_bind_field = env::var("LARK_GITHUB_BIND_FIELD").unwrap_or(String::new());
    if let Some(value) = record.fields.get(&issue_bind_field) {
        let link = for_string(value);
        if let Some(id_str) = link.split("/").last() {
            let id_res = id_str.parse::<u64>();
            if let Ok(id) = id_res {
                return Some(id);
            } else {
                warn!("Parse issue: {}, id({}) failed, {:?}", link, id_str, id_res);
            }
        }
    }
    None
}

pub async fn bind_issue(
    record_id: &str,
    issue: u64,
    title: String,
) -> Result<(UpdateBitableRecordResp, CommonResponse), lark_bot_sdk_patch::error::Error> {
    let app_token = env::var("LARK_BITABLE_TOKEN").unwrap_or(String::new());
    let table_id = env::var("LARK_BITABLE_TABLE_ID").unwrap_or(String::new());
    let github_bind_field = env::var("LARK_GITHUB_BIND_FIELD").unwrap_or(String::new());
    let owner = env::var("GH_OWNER").unwrap_or(String::new());
    let repo = env::var("GH_REPO").unwrap_or(String::new());
    let link = format!("https://github.com/{owner}/{repo}/issues/{issue}");
    let url_text = format!("#{issue} {title}");
    let mut obj_map = Map::new();
    obj_map.insert("link".to_string(), link.into());
    obj_map.insert("text".to_string(), url_text.into());
    let mut map: HashMap<String, serde_json::value::Value> = HashMap::new();
    warn!("绑定到 {} ", github_bind_field);
    map.insert(github_bind_field.into(), serde_json::Value::Object(obj_map));
    client()
        .bitable()
        .update_bitable_record(UpdateBitableRecordReq {
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            user_id_type: "open_id".into(),
            record_id: record_id.into(),
            fields: map,
            ..Default::default()
        })
        .await
}

pub fn format_record(record: &AppTableRecordSubResp) -> IssueReq {
    let title_format_str = env::var("ISSUE_TITLE_FORMAT").unwrap_or(String::new());
    let content_format_str = env::var("ISSUE_CONTENT_FORMAT").unwrap_or(String::new());
    let mut title = String::from(title_format_str);
    let mut content = String::from(content_format_str);
    record.fields.iter().for_each(|(field, value)| {
        let key = format!("\\{{{}(|:[^{{}}]*)}}", field);
        let re = Regex::new(&key).unwrap();
        let v = for_string(value);
        title = re.replace_all(&title, &v).to_string();
        content = re.replace_all(&content, &v).to_string();
        // replace time
        if let Ok(v) = for_string(value).parse::<i64>() {
            let key = format!("\\{{@{}(|:[^{{}}]*)}}", field);
            let re = Regex::new(&key).unwrap();
            let time = DateTime::from_timestamp_millis(v).unwrap();
            let time_str = time.format("%Y/%m/%d %H:%M").to_string();
            title = re.replace_all(&title, &time_str).to_string();
            content = re.replace_all(&content, &time_str).to_string();
        }
    });
    let created_at = DateTime::from_timestamp_millis(record.created_time).unwrap();
    let updated_at = DateTime::from_timestamp_millis(record.last_modified_time).unwrap();
    content = content.replace(
        "{created_at}",
        &created_at.format("%Y/%m/%d %H:%M").to_string(),
    );
    content = content.replace(
        "{updated_at}",
        &updated_at.format("%Y/%m/%d %H:%M").to_string(),
    );
    content = content.replace("{created_by}", &record.created_by.en_name);
    content = content.replace("{updated_by}", &record.last_modified_by.en_name);
    let re = Regex::new("\\{([^:]+):([^{{}}]*)}").unwrap();
    title = re.replace_all(&title, "$2").to_string();
    content = re.replace_all(&content, "$2").to_string();
    debug!("Content: {}", content);
    let labels = get_labels(record);
    IssueReq {
        title,
        content,
        label: labels,
    }
}

fn get_labels(record: &AppTableRecordSubResp) -> Vec<String> {
    let label_fields_name = env::var("ISSUE_LABEL_FIELDS").unwrap_or(String::new());
    let fields: Vec<&str> = label_fields_name.split(",").collect();
    let mut label: Vec<String> = Vec::new();
    for field_name in fields {
        let label_opt = record.fields.get(field_name);
        if let Some(label_val) = label_opt {
            if label_val.is_array() {
                label_val
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .filter(|s| !s.is_empty())
                    .for_each(|l| {
                        label.push(l);
                    });
            } else if label_val.is_string() {
                if let Some(str) = label_val.as_str() {
                    label.push(str.into());
                }
            } else {
                warn!("😰 不支持的标签类型：{:?}", label_val);
            }
        }
    }
    label
}

#[test]
fn bracket() {
    let re = Regex::new("\\{([^:]+):([^{{}}]*)}").unwrap();
    let result = re.replace("{Hello:Default}", "$2");
    assert_eq!(result, "Default");
    let key = format!("\\{{{}(|:[^{{}}]*)}}", "Hello");
    let re = Regex::new(&key).unwrap();
    let result = re.replace_all("{Hello:Default}", "World");
    assert_eq!(result, "World");
}
