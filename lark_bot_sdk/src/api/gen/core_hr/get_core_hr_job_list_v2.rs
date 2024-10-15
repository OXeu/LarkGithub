//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job/list>
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
    /// **api 版本: 2024-02-06T02:29:59+00:00**
    ///
    /// ## 批量查询职务（V2)
    ///
    /// 批量查询职务。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Fcorehr-v2%2Fjob%2Flist>
    pub async fn get_core_hr_job_list_v2(
        &self,
        req: GetCoreHrJobListV2Req,
    ) -> Result<(GetCoreHrJobListV2Resp, CommonResponse), Error> {
        self.get_core_hr_job_list_v2_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_core_hr_job_list_v2](#method.get_core_hr_job_list_v2) 函数
    pub async fn get_core_hr_job_list_v2_with_opt(
        &self,
        req: GetCoreHrJobListV2Req,
        method_option: MethodOption,
    ) -> Result<(GetCoreHrJobListV2Resp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_core_hr_job_list_v2(&req) {
                tracing::info!("[lark] CoreHr#GetCoreHrJobListV2 **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#GetCoreHrJobListV2 call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "GetCoreHrJobListV2",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/corehr/v2/jobs",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCoreHrJobListV2RespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetCoreHrJobListV2Req {
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "1231231987"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 每页获取记录数量，最大100
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "100"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: String,
    /// 名称
    ///
    /// **示例值**: "keyword"
    #[api(kind = "query", name = "name", v_type = "var", option = "false")]
    pub name: String,
    /// 语言
    ///
    /// **示例值**: "zh"
    #[api(
        kind = "query",
        name = "query_language",
        v_type = "var",
        option = "false"
    )]
    pub query_language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCoreHrJobListV2RespInner {
    #[serde(flatten)]
    data: Option<GetCoreHrJobListV2Resp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCoreHrJobListV2Resp {
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
    /// 查询的职务信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<JobSubResp>,
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "1234452132"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobSubResp {
    /// 实体在CoreHR内部的唯一键
    ///
    /// **示例值**: "4698040628992333549"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 编码
    ///
    /// **示例值**: "JP422119"
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
    /// 启用
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "active",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub active: bool,
    /// 职务头衔
    #[serde(
        rename = "job_title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_title: Vec<I18nSubResp>,
    /// 序列
    #[serde(
        rename = "job_family_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_family_id_list: Vec<String>,
    /// 职级
    #[serde(
        rename = "job_level_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_level_id_list: Vec<String>,
    /// 工时制度，引用WorkingHoursType的ID
    ///
    /// **示例值**: "6890452208593372679"
    #[serde(
        rename = "working_hours_type_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub working_hours_type_id: String,
    /// 生效时间
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-01-01 00:00:00"
    #[serde(
        rename = "effective_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub effective_time: String,
    /// 失效时间
    ///
    /// **示例值**: "2021-01-01 00:00:00"
    #[serde(
        rename = "expiration_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expiration_time: String,
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
    /// 语言
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh-CN"
    #[serde(
        rename = "lang",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub lang: String,
    /// 内容
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
    /// 字段值，是json转义后的字符串，根据元数据定义不同，字段格式不同(123, 123.23, true, [\"id1\",\"id2\], 2006-01-02 15:04:05])
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Sandy"
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
        Fn(GetCoreHrJobListV2Req) -> Result<(GetCoreHrJobListV2Resp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetCoreHrJobListV2Req) -> Result<(GetCoreHrJobListV2Resp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_core_hr_job_list_v2<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetCoreHrJobListV2Req, GetCoreHrJobListV2Resp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_core_hr_job_list_v2(
            &self,
            req: &GetCoreHrJobListV2Req,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetCoreHrJobListV2Req, GetCoreHrJobListV2Resp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::get_core_hr_job_list_v2::{
            GetCoreHrJobListV2Req, GetCoreHrJobListV2Resp, GetCoreHrJobListV2RespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_get_core_hr_job_list_v2(|_| {
                Ok((GetCoreHrJobListV2Resp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .core_hr()
            .get_core_hr_job_list_v2(GetCoreHrJobListV2Req::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .get_core_hr_job_list_v2(GetCoreHrJobListV2Req::default())
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
        "items": [
            {
                "id": "4698040628992333549",
                "code": "JP422119",
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
                "job_title": [
                    {
                        "lang": "zh-CN",
                        "value": "张三"
                    }
                ],
                "job_family_id_list": [
                    "4719519211875096301"
                ],
                "job_level_id_list": [
                    "4719519212005299950"
                ],
                "working_hours_type_id": "6890452208593372679",
                "effective_time": "2020-01-01 00:00:00",
                "expiration_time": "2021-01-01 00:00:00",
                "custom_fields": [
                    {
                        "field_name": "name",
                        "value": "Sandy"
                    }
                ]
            }
        ],
        "has_more": true,
        "page_token": "1234452132"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCoreHrJobListV2RespInner>(RESP);
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
