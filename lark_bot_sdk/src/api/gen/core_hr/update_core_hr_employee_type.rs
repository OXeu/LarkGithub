//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employee_type/patch>
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
    /// **api 版本: 2024-02-06T02:19:06+00:00**
    ///
    /// ## 更新人员类型
    ///
    /// 更新人员类型。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employee_type/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/basic-infomation/employee_type/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fbasic-infomation%2Femployee_type%2Fpatch>
    pub async fn update_core_hr_employee_type(
        &self,
        req: UpdateCoreHrEmployeeTypeReq,
    ) -> Result<(UpdateCoreHrEmployeeTypeResp, CommonResponse), Error> {
        self.update_core_hr_employee_type_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_core_hr_employee_type](#method.update_core_hr_employee_type) 函数
    pub async fn update_core_hr_employee_type_with_opt(
        &self,
        req: UpdateCoreHrEmployeeTypeReq,
        method_option: MethodOption,
    ) -> Result<(UpdateCoreHrEmployeeTypeResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_core_hr_employee_type(&req) {
                tracing::info!("[lark] CoreHr#UpdateCoreHrEmployeeType **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#UpdateCoreHrEmployeeType call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "UpdateCoreHrEmployeeType",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/employee_types/:employee_type_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateCoreHrEmployeeTypeRespInner, _) =
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
pub struct UpdateCoreHrEmployeeTypeReq {
    /// 雇员类型 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6969828847931885087"
    #[api(kind = "path", name = "employee_type_id")]
    pub employee_type_id: String,
    /// 根据client_token是否一致来判断是否为同一请求
    ///
    /// **示例值**: "12454646"
    #[api(
        kind = "query",
        name = "client_token",
        v_type = "var",
        option = "false"
    )]
    pub client_token: String,
    /// 名称
    #[api(kind = "body", name = "name")]
    pub name: Vec<Option<I18nSubReq>>,
    /// 是否为默认人员类型，每个租户只能定义一个默认人员类型
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "default_employee_type")]
    pub default_employee_type: Option<bool>,
    /// 启用
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "active")]
    pub active: Option<bool>,
    /// 编码
    ///
    /// **示例值**: "1245"
    #[api(kind = "body", name = "code")]
    pub code: Option<String>,
    /// 自定义字段
    #[api(kind = "body", name = "custom_fields")]
    pub custom_fields: Vec<Option<ObjectFieldDataSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubReq {
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
pub struct ObjectFieldDataSubReq {
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

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateCoreHrEmployeeTypeRespInner {
    #[serde(flatten)]
    data: Option<UpdateCoreHrEmployeeTypeResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateCoreHrEmployeeTypeResp {
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
    /// 人员类型
    #[serde(
        rename = "employee_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employee_type: EmployeeTypeSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EmployeeTypeSubResp {
    /// 雇员类型ID
    ///
    /// **示例值**: "6919732473504990727"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 名称
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Vec<I18nSubResp>,
    /// 是否为默认人员类型，每个租户只能定义一个默认人员类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "default_employee_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub default_employee_type: bool,
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
    /// 编码
    ///
    /// **示例值**: "1245"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
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
        Fn(
            UpdateCoreHrEmployeeTypeReq,
        ) -> Result<(UpdateCoreHrEmployeeTypeResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateCoreHrEmployeeTypeReq,
                ) -> Result<(UpdateCoreHrEmployeeTypeResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_core_hr_employee_type<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateCoreHrEmployeeTypeReq,
            UpdateCoreHrEmployeeTypeResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_core_hr_employee_type(
            &self,
            req: &UpdateCoreHrEmployeeTypeReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateCoreHrEmployeeTypeReq,
                UpdateCoreHrEmployeeTypeResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::update_core_hr_employee_type::{
            UpdateCoreHrEmployeeTypeReq, UpdateCoreHrEmployeeTypeResp,
            UpdateCoreHrEmployeeTypeRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_update_core_hr_employee_type(|_| {
                Ok((
                    UpdateCoreHrEmployeeTypeResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .update_core_hr_employee_type(UpdateCoreHrEmployeeTypeReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .update_core_hr_employee_type(UpdateCoreHrEmployeeTypeReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "name": [
        {
            "lang": "zh-CN",
            "value": "张三"
        }
    ],
    "default_employee_type": true,
    "active": true,
    "code": "1245",
    "custom_fields": [
        {
            "field_name": "name",
            "value": "\"Sandy\""
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateCoreHrEmployeeTypeReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "employee_type": {
            "id": "6919732473504990727",
            "name": [
                {
                    "lang": "zh-CN",
                    "value": "张三"
                }
            ],
            "default_employee_type": true,
            "active": true,
            "code": "1245",
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
        let res = serde_json::from_str::<UpdateCoreHrEmployeeTypeRespInner>(RESP);
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
