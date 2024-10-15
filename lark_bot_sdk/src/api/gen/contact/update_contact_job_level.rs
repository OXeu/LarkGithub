//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_level/update>
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
    /// **api 版本: 2024-07-05T08:21:32+00:00**
    ///
    /// ## 更新职级
    ///
    /// 调用该接口更新指定职级的信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_level/update>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/contact-v3/job_level/update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcontact-v3%2Fjob_level%2Fupdate>
    pub async fn update_contact_job_level(
        &self,
        req: UpdateContactJobLevelReq,
    ) -> Result<(UpdateContactJobLevelResp, CommonResponse), Error> {
        self.update_contact_job_level_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_contact_job_level](#method.update_contact_job_level) 函数
    pub async fn update_contact_job_level_with_opt(
        &self,
        req: UpdateContactJobLevelReq,
        method_option: MethodOption,
    ) -> Result<(UpdateContactJobLevelResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_contact_job_level(&req) {
                tracing::info!("[lark] Contact#UpdateContactJobLevel **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#UpdateContactJobLevel call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "UpdateContactJobLevel",
            method: http::Method::PUT,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/job_levels/:job_level_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateContactJobLevelRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateContactJobLevelReq {
    /// 职级 ID。获取方式：
    ///
    /// - 创建职级时，可以从返回结果中获取职级 ID。
    ///
    /// - 调用[获取租户职级列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_level/list)接口，查找指定职级的 ID 信息。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "mga5oa8ayjlp9rb"
    #[api(kind = "path", name = "job_level_id")]
    pub job_level_id: String,

    /// 职级的通用名称。如果未设置多语言名称，则默认展示该名称。
    ///
    /// **默认值**：空，表示不更新。
    ///
    /// **示例值**: "高级专家"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `255` 字符
    #[api(kind = "body", name = "name")]
    pub name: Option<String>,
    /// 职级的通用描述。如果未设置多语言描述，则默认展示该描述。
    ///
    /// **默认值**：空，表示不更新。
    ///
    /// **示例值**: "公司内部中高级职称，有一定专业技术能力的人员"
    #[api(kind = "body", name = "description")]
    pub description: Option<String>,
    /// 职级排序。数值越小，排序越靠前。
    ///
    /// **默认值**：空，表示不更新。
    ///
    /// **示例值**: "200"
    #[api(kind = "body", name = "order")]
    pub order: Option<i64>,
    /// 是否启用该职级。
    ///
    /// **可选值有**：
    ///
    /// - true：启用
    ///
    /// - false：不启用
    ///
    /// **默认值**：空，表示不更新。
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "status")]
    pub status: Option<bool>,
    /// 多语言职级名称。
    #[api(kind = "body", name = "i18n_name")]
    pub i18n_name: Vec<Option<I18nContentSubReq>>,
    /// 多语言职级描述。
    #[api(kind = "body", name = "i18n_description")]
    pub i18n_description: Vec<Option<I18nContentSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nContentSubReq {
    /// 语言版本。例如：
    ///
    /// - zh_cn：中文
    ///
    /// - en_us：英语
    ///
    /// - ja_jp：日语
    ///
    /// **默认值**：空，表示不更新。
    ///
    /// **示例值**: "zh_cn"
    #[serde(
        rename = "locale",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub locale: Option<String>,
    /// 语言版本对应的职级名称。
    ///
    /// **默认值**：空，表示不更新。
    ///
    /// **示例值**: "多语言内容"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateContactJobLevelRespInner {
    #[serde(flatten)]
    data: Option<UpdateContactJobLevelResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateContactJobLevelResp {
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
    /// 职级信息。
    #[serde(
        rename = "job_level",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_level: JobLevelSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobLevelSubResp {
    /// 职级名称。
    ///
    /// **示例值**: "高级专家"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `255` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 职级描述。
    ///
    /// **示例值**: "公司内部中高级职称，有一定专业技术能力的人员"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 职级排序。数值越小，排序越靠前。
    ///
    /// **示例值**: "200"
    #[serde(
        rename = "order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub order: i64,
    /// 是否启用职级。
    ///
    /// **可能值有**：
    ///
    /// true：启用
    ///
    /// false：不启用
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: bool,
    /// 职级 ID。后续可通过该 ID 删除、更新、查询职级。
    ///
    /// **示例值**: "mga5oa8ayjlp9rb"
    #[serde(
        rename = "job_level_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_level_id: String,
    /// 多语言名称。
    #[serde(
        rename = "i18n_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_name: Vec<I18nContentSubResp>,
    /// 多语言描述。
    #[serde(
        rename = "i18n_description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_description: Vec<I18nContentSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nContentSubResp {
    /// 语言版本。
    ///
    /// **示例值**: "zh_cn"
    #[serde(
        rename = "locale",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub locale: String,
    /// 语言版本对应的名称。
    ///
    /// **示例值**: "多语言内容"
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
        Fn(UpdateContactJobLevelReq) -> Result<(UpdateContactJobLevelResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateContactJobLevelReq,
                ) -> Result<(UpdateContactJobLevelResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_contact_job_level<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateContactJobLevelReq,
            UpdateContactJobLevelResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_contact_job_level(
            &self,
            req: &UpdateContactJobLevelReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateContactJobLevelReq, UpdateContactJobLevelResp, Arc<dyn MockFunc>>(
                self.cli.instance_id,
                req,
            )
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::contact::update_contact_job_level::{
            UpdateContactJobLevelReq, UpdateContactJobLevelResp, UpdateContactJobLevelRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_update_contact_job_level(|_| {
                Ok((
                    UpdateContactJobLevelResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .contact()
            .update_contact_job_level(UpdateContactJobLevelReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .update_contact_job_level(UpdateContactJobLevelReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "name": "高级专家",
    "description": "公司内部中高级职称，有一定专业技术能力的人员",
    "order": 200,
    "status": true,
    "i18n_name": [
        {
            "locale": "zh_cn",
            "value": "多语言内容"
        }
    ],
    "i18n_description": [
        {
            "locale": "zh_cn",
            "value": "多语言内容"
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateContactJobLevelReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "job_level": {
            "name": "高级专家",
            "description": "公司内部中高级职称，有一定专业技术能力的人员",
            "order": 200,
            "status": true,
            "job_level_id": "mga5oa8ayjlp9rb",
            "i18n_name": [
                {
                    "locale": "zh_cn",
                    "value": "多语言内容"
                }
            ],
            "i18n_description": [
                {
                    "locale": "zh_cn",
                    "value": "多语言内容"
                }
            ]
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateContactJobLevelRespInner>(RESP);
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