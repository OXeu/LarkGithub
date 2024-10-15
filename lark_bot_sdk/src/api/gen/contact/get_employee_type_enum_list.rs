//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/list>
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
    /// **api 版本: 2024-07-05T08:09:20+00:00**
    ///
    /// ## 查询人员类型
    ///
    /// 调用该接口查询当前租户下所有的人员类型信息，包括选项 ID、类型、编号以及内容等。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/contact-v3/employee_type_enum/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcontact-v3%2Femployee_type_enum%2Flist>
    pub async fn get_employee_type_enum_list(
        &self,
        req: GetEmployeeTypeEnumListReq,
    ) -> Result<(GetEmployeeTypeEnumListResp, CommonResponse), Error> {
        self.get_employee_type_enum_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_employee_type_enum_list](#method.get_employee_type_enum_list) 函数
    pub async fn get_employee_type_enum_list_with_opt(
        &self,
        req: GetEmployeeTypeEnumListReq,
        method_option: MethodOption,
    ) -> Result<(GetEmployeeTypeEnumListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_employee_type_enum_list(&req) {
                tracing::info!("[lark] Contact#GetEmployeeTypeEnumList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#GetEmployeeTypeEnumList call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "GetEmployeeTypeEnumList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/employee_type_enums",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetEmployeeTypeEnumListRespInner, _) =
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
pub struct GetEmployeeTypeEnumListReq {
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "3"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 分页大小，用于限制一次请求返回的条目数。
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetEmployeeTypeEnumListRespInner {
    #[serde(flatten)]
    data: Option<GetEmployeeTypeEnumListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetEmployeeTypeEnumListResp {
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
    /// 人员类型的选项信息。
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<EmployeeTypeEnumSubResp>,
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
    /// **示例值**: "3"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EmployeeTypeEnumSubResp {
    /// 选项 ID。后续可以使用该 ID 更新、删除选项。
    ///
    /// **示例值**: "exGeIjow7zIqWMy+ONkFxA=="
    #[serde(
        rename = "enum_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enum_id: String,
    /// 选项的编号值。后续可使用该编号配置用户的人员类型属性。例如，调用[创建用户](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/create)接口时，employee_type 参数值对应的就是当前的 enum_value。
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
        Fn(
            GetEmployeeTypeEnumListReq,
        ) -> Result<(GetEmployeeTypeEnumListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetEmployeeTypeEnumListReq,
                ) -> Result<(GetEmployeeTypeEnumListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_employee_type_enum_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetEmployeeTypeEnumListReq,
            GetEmployeeTypeEnumListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_employee_type_enum_list(
            &self,
            req: &GetEmployeeTypeEnumListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetEmployeeTypeEnumListReq,
                GetEmployeeTypeEnumListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::contact::get_employee_type_enum_list::{
            GetEmployeeTypeEnumListReq, GetEmployeeTypeEnumListResp,
            GetEmployeeTypeEnumListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_get_employee_type_enum_list(|_| {
                Ok((
                    GetEmployeeTypeEnumListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .contact()
            .get_employee_type_enum_list(GetEmployeeTypeEnumListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .get_employee_type_enum_list(GetEmployeeTypeEnumListReq::default())
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
        ],
        "has_more": true,
        "page_token": "3"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetEmployeeTypeEnumListRespInner>(RESP);
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