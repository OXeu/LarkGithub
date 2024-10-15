//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_level/get>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::core_hr::CoreHrService;

impl<'c, IStore: Store, IClient: HttpClient> CoreHrService<'c, IStore, IClient> {
    /// **api 版本: 2024-02-06T02:28:54+00:00**
    ///
    /// ## 查询单个职级
    ///
    /// 根据 ID 查询单个职级
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_level/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/job-management/job_level/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fjob-management%2Fjob_level%2Fget>
    pub async fn get_core_hr_job_level(
        &self,
        req: GetCoreHrJobLevelReq,
    ) -> Result<(GetCoreHrJobLevelResp, CommonResponse), Error> {
        self.get_core_hr_job_level_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_core_hr_job_level](#method.get_core_hr_job_level) 函数
    pub async fn get_core_hr_job_level_with_opt(
        &self,
        req: GetCoreHrJobLevelReq,
        method_option: MethodOption,
    ) -> Result<(GetCoreHrJobLevelResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_core_hr_job_level(&req) {
                tracing::info!("[lark] CoreHr#GetCoreHrJobLevel **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#GetCoreHrJobLevel call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "GetCoreHrJobLevel",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/job_levels/:job_level_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCoreHrJobLevelRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetCoreHrJobLevelReq {
    /// 职级 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1515"
    #[api(kind = "path", name = "job_level_id")]
    pub job_level_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCoreHrJobLevelRespInner {
    #[serde(flatten)]
    data: Option<GetCoreHrJobLevelResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCoreHrJobLevelResp {
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
    /// 职级信息
    #[serde(
        rename = "job_level",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_level: JobLevelSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobLevelSubResp {
    /// 职级 ID
    ///
    /// **示例值**: "4692446793125560154"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 职级数值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "9999"
    #[serde(
        rename = "level_order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub level_order: i64,
    /// 编码
    ///
    /// **示例值**: "VQzo/BSonp8l6PmcZ+VlDhkd2595LMkhyBAGX6HAlCY="
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 名称
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Vec<I18nSubResp>,
    /// 描述
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: Vec<I18nSubResp>,
    /// 是否启用
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "active",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub active: bool,
    /// 自定义字段
    #[serde(
        rename = "custom_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_fields: Vec<ObjectFieldDataSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 名称信息的语言
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh-CN"
    #[serde(
        rename = "lang",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub lang: String,
    /// 名称信息的内容
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "张三"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ObjectFieldDataSubResp {
    /// 字段名
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "name"
    #[serde(
        rename = "field_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub field_name: String,
    /// 字段值，是json转义后的字符串，根据元数据定义不同，字段格式不同(如123, 123.23, "true", [\"id1\",\"id2\"], "2006-01-02 15:04:05")
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "\"Sandy\""
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::core_hr::CoreHrServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetCoreHrJobLevelReq) -> Result<(GetCoreHrJobLevelResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetCoreHrJobLevelReq) -> Result<(GetCoreHrJobLevelResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_core_hr_job_level<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetCoreHrJobLevelReq, GetCoreHrJobLevelResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_core_hr_job_level(
            &self,
            req: &GetCoreHrJobLevelReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetCoreHrJobLevelReq, GetCoreHrJobLevelResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::get_core_hr_job_level::{
            GetCoreHrJobLevelReq, GetCoreHrJobLevelResp, GetCoreHrJobLevelRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_get_core_hr_job_level(|_| {
                Ok((GetCoreHrJobLevelResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .core_hr()
            .get_core_hr_job_level(GetCoreHrJobLevelReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .get_core_hr_job_level(GetCoreHrJobLevelReq::default())
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

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "job_level": {
            "id": "4692446793125560154",
            "level_order": 9999,
            "code": "VQzo/BSonp8l6PmcZ+VlDhkd2595LMkhyBAGX6HAlCY=",
            "name": [
                {
                    "lang": "zh-CN",
                    "value": "张三"
                }
            ],
            "description": [
                {
                    "lang": "zh-CN",
                    "value": "张三"
                }
            ],
            "active": true,
            "custom_fields": [
                {
                    "field_name": "name",
                    "value": "\"Sandy\""
                }
            ]
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCoreHrJobLevelRespInner>(RESP);
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
