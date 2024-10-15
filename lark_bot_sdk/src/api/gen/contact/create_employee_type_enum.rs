//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/create>
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
    /// **api 版本: 2024-07-05T08:09:19+00:00**
    ///
    /// ## 新增人员类型
    ///
    /// 调用该接口新增一个自定义的人员类型。人员类型是用户属性之一，用于灵活标记用户的身份类型。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/contact-v3/employee_type_enum/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcontact-v3%2Femployee_type_enum%2Fcreate>
    pub async fn create_employee_type_enum(
        &self,
        req: CreateEmployeeTypeEnumReq,
    ) -> Result<(CreateEmployeeTypeEnumResp, CommonResponse), Error> {
        self.create_employee_type_enum_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_employee_type_enum](#method.create_employee_type_enum) 函数
    pub async fn create_employee_type_enum_with_opt(
        &self,
        req: CreateEmployeeTypeEnumReq,
        method_option: MethodOption,
    ) -> Result<(CreateEmployeeTypeEnumResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_employee_type_enum(&req) {
                tracing::info!("[lark] Contact#CreateEmployeeTypeEnum **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#CreateEmployeeTypeEnum call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "CreateEmployeeTypeEnum",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/employee_type_enums",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateEmployeeTypeEnumRespInner, _) =
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
pub struct CreateEmployeeTypeEnumReq {
    /// 人员类型的选项内容。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "专家"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `100` 字符
    #[api(kind = "body", name = "content")]
    pub content: String,
    /// 人员类型的选项类型。新增人员类型时固定取值为 `2` 即可。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2"
    ///
    /// **可选值**:
    ///
    /// `Defualt`: 内置类型，只读。新增人员类型时不支持选择该类型。
    ///
    /// `Custom`: 自定义。
    #[api(kind = "body", name = "enum_type")]
    pub enum_type: i64,
    /// 人员类型的选项激活状态。只有已激活的选项可以用于配置用户属性。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Active`: 激活
    ///
    /// `Inactive`: 未激活
    #[api(kind = "body", name = "enum_status")]
    pub enum_status: i64,
    /// 选项内容的国际化配置。
    ///
    /// **说明**：在飞书客户端查看用户人员类型时，系统会根据客户端语言环境，自动展示相匹配的选项语言。如果相应语言不在选项国际化配置当中，则会展示默认选项内容（即 content 字段）。
    #[api(kind = "body", name = "i18n_content")]
    pub i18n_content: Vec<Option<I18nContentSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nContentSubReq {
    /// 语言版本。例如：
    ///
    /// - zh_cn：中文
    ///
    /// - en_us：英文
    ///
    /// - ja_jp：日文
    ///
    /// **示例值**: "zh_cn"
    #[serde(
        rename = "locale",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub locale: Option<String>,
    /// 语言版本对应的内容。
    ///
    /// **数据校验规则：**
    ///
    /// 长度范围：`1` 字符 ～ `100` 字符
    ///
    /// **示例值**: "专家（中文）"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateEmployeeTypeEnumRespInner {
    #[serde(flatten)]
    data: Option<CreateEmployeeTypeEnumResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateEmployeeTypeEnumResp {
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
    /// 新建的人员类型信息。
    #[serde(
        rename = "employee_type_enum",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employee_type_enum: EmployeeTypeEnumSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EmployeeTypeEnumSubResp {
    /// 人员类型的选项 ID。后续可以使用该 ID 更新、删除选项。
    ///
    /// **示例值**: "exGeIjow7zIqWMy+ONkFxA=="
    #[serde(
        rename = "enum_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enum_id: String,
    /// 人员类型的选项编号，新增人员类型后，由系统生成的编号。后续可使用该编号配置用户的人员类型属性。例如，调用[创建用户](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/create)接口时，employee_type 参数值对应的就是当前的 enum_value。
    ///
    /// **示例值**: "2"
    #[serde(
        rename = "enum_value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enum_value: String,
    /// 选项内容。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "专家"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `100` 字符
    #[serde(
        rename = "content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub content: String,
    /// 选项类型。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2"
    ///
    /// **可选值**:
    ///
    /// `Defualt`: 内置类型
    ///
    /// `Custom`: 自定义
    #[serde(
        rename = "enum_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enum_type: i64,
    /// 选项的激活状态。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Active`: 激活
    ///
    /// `Inactive`: 未激活
    #[serde(
        rename = "enum_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enum_status: i64,
    /// 选项内容的国际化配置。
    #[serde(
        rename = "i18n_content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_content: Vec<I18nContentSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nContentSubResp {
    /// 语言版本。例如：
    ///
    /// - zh_cn：中文
    ///
    /// - en_us：英文
    ///
    /// - ja_jp：日文
    ///
    /// **示例值**: "zh_cn"
    #[serde(
        rename = "locale",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub locale: String,
    /// 语言版本对应的内容。
    ///
    /// **示例值**: "专家（中文）"
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
        Fn(CreateEmployeeTypeEnumReq) -> Result<(CreateEmployeeTypeEnumResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    CreateEmployeeTypeEnumReq,
                ) -> Result<(CreateEmployeeTypeEnumResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_employee_type_enum<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            CreateEmployeeTypeEnumReq,
            CreateEmployeeTypeEnumResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_employee_type_enum(
            &self,
            req: &CreateEmployeeTypeEnumReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                CreateEmployeeTypeEnumReq,
                CreateEmployeeTypeEnumResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::contact::create_employee_type_enum::{
            CreateEmployeeTypeEnumReq, CreateEmployeeTypeEnumResp, CreateEmployeeTypeEnumRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_create_employee_type_enum(|_| {
                Ok((
                    CreateEmployeeTypeEnumResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .contact()
            .create_employee_type_enum(CreateEmployeeTypeEnumReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .create_employee_type_enum(CreateEmployeeTypeEnumReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "content": "专家",
    "enum_type": 2,
    "enum_status": 1,
    "i18n_content": [
        {
            "locale": "zh_cn",
            "value": "专家（中文）"
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateEmployeeTypeEnumReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "employee_type_enum": {
            "enum_id": "exGeIjow7zIqWMy+ONkFxA==",
            "enum_value": "2",
            "content": "专家",
            "enum_type": 2,
            "enum_status": 1,
            "i18n_content": [
                {
                    "locale": "zh_cn",
                    "value": "专家（中文）"
                }
            ]
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateEmployeeTypeEnumRespInner>(RESP);
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