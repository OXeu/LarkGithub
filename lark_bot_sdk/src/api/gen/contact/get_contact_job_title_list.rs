//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_title/list>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::contact::ContactService;

impl<'c, IStore: Store, IClient: HttpClient> ContactService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-05T08:23:49+00:00**
    ///
    /// ## 获取租户职务列表
    ///
    /// 调用该接口获取当前租户下的职务信息，包括职务的 ID、名称、多语言名称以及启用状态。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_title/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_title/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Freference%2Fcontact-v3%2Fjob_title%2Flist>
    pub async fn get_contact_job_title_list(
        &self,
        req: GetContactJobTitleListReq,
    ) -> Result<(GetContactJobTitleListResp, CommonResponse), Error> {
        self.get_contact_job_title_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_contact_job_title_list](#method.get_contact_job_title_list) 函数
    pub async fn get_contact_job_title_list_with_opt(
        &self,
        req: GetContactJobTitleListReq,
        method_option: MethodOption,
    ) -> Result<(GetContactJobTitleListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_contact_job_title_list(&req) {
                tracing::info!("[lark] Contact#GetContactJobTitleList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#GetContactJobTitleList call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "GetContactJobTitleList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/job_titles",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetContactJobTitleListRespInner, _) =
            self.cli.do_req(req).await?;
        let data = match resp.data {
            Some(data) => data,
            None => {
                return Err(Error::ErrResponse(
                    anyhow::anyhow!("missing response data"),
                    common_resp,
                ));
            }
        };
        Ok((data, common_resp))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, lark_bot_sdk_macros::ApiReqParams)]
pub struct GetContactJobTitleListReq {
    /// 分页大小，用于限制一次请求所返回的数据条目数。
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: ""xxx""
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetContactJobTitleListRespInner {
    #[serde(flatten)]
    data: Option<GetContactJobTitleListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetContactJobTitleListResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: DataSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DataSubResp {
    /// 职务列表。
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<JobTitleSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "1r5QdASJi1sp5aJn"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobTitleSubResp {
    /// 职务 ID。
    ///
    /// **示例值**: "b5565c46b749"
    #[serde(
        rename = "job_title_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_title_id: String,
    /// 职务名称。
    ///
    /// **示例值**: "高级工程师"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `100` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 多语言职务名称。
    #[serde(
        rename = "i18n_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_name: Vec<I18nContentSubResp>,
    /// 是否启用职务。
    ///
    /// **可能值有**：
    ///
    /// - true：启用
    ///
    /// - false：禁用
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nContentSubResp {
    /// 语言版本。例如：
    ///
    /// - zh_cn：中文
    ///
    /// - en_us：英语
    ///
    /// - ja_jp：日语
    ///
    /// **示例值**: "zh_cn"
    #[serde(
        rename = "locale",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub locale: String,
    /// 多语言版本对应的值。
    ///
    /// **示例值**: "专家"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::contact::ContactServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetContactJobTitleListReq) -> Result<(GetContactJobTitleListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetContactJobTitleListReq,
                ) -> Result<(GetContactJobTitleListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_contact_job_title_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetContactJobTitleListReq,
            GetContactJobTitleListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_contact_job_title_list(
            &self,
            req: &GetContactJobTitleListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetContactJobTitleListReq,
                GetContactJobTitleListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::contact::get_contact_job_title_list::{
            GetContactJobTitleListReq, GetContactJobTitleListResp, GetContactJobTitleListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_get_contact_job_title_list(|_| {
                Ok((
                    GetContactJobTitleListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .contact()
            .get_contact_job_title_list(GetContactJobTitleListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .get_contact_job_title_list(GetContactJobTitleListReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = "{}";

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<()>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{"code":0,
"msg":"success",
"data":{"items":[{"job_title_id":"b5565c46b749",
"name":"高级工程师",
"i18n_name":[{
    "locale": "zh_cn",
    "value": "专家"
}],
"status":true}],
"page_token":"1r5QdASJi1sp5aJn",
"has_more":true}}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetContactJobTitleListRespInner>(RESP);
        if let Err(e) = res {
            panic!("{}", e);
        }
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(RESP) {
            if v.get("data").is_some() {
                assert!(res.unwrap().data.is_some());
            }
        }
    }
}
