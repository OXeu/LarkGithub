//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/bind_department>
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
    /// **api 版本: 2024-07-05T08:16:17+00:00**
    ///
    /// ## 建立部门与单位的绑定关系
    ///
    /// 调用该接口建立部门与单位的绑定关系。一个部门同时只能绑定一个单位。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/bind_department>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/contact-v3/unit/bind_department>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcontact-v3%2Funit%2Fbind_department>
    pub async fn bind_contact_unit_department(
        &self,
        req: BindContactUnitDepartmentReq,
    ) -> Result<(BindContactUnitDepartmentResp, CommonResponse), Error> {
        self.bind_contact_unit_department_with_opt(req, Default::default())
            .await
    }

    /// 参见 [bind_contact_unit_department](#method.bind_contact_unit_department) 函数
    pub async fn bind_contact_unit_department_with_opt(
        &self,
        req: BindContactUnitDepartmentReq,
        method_option: MethodOption,
    ) -> Result<(BindContactUnitDepartmentResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_bind_contact_unit_department(&req) {
                tracing::info!("[lark] Contact#BindContactUnitDepartment **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#BindContactUnitDepartment call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "BindContactUnitDepartment",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/unit/bind_department",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BindContactUnitDepartmentRespInner, _) =
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
pub struct BindContactUnitDepartmentReq {
    /// 单位 ID。
    ///
    /// 当你在创建单位时，可以在返回结果中获取单位 ID。你也可以调用[获取单位列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/list)接口，获取单位 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "BU121"
    #[api(kind = "body", name = "unit_id")]
    pub unit_id: String,
    /// 单位关联的部门 ID，ID 类型与 department_id_type 的取值保持一致。
    ///
    /// 部门 API 提供了多种获取部门 ID 的方式，如[获取子部门列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/children)、[获取父部门信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/parent)、[搜索部门](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/search)，你可以选择合适的 API 进行查询。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "od-4e6ac4d14bcd5071a37a39de902c7141"
    #[api(kind = "body", name = "department_id")]
    pub department_id: String,
    /// 此次调用中的部门 ID 类型。关于部门 ID 的详细介绍，可参见[部门 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview#23857fe0)。
    ///
    /// **示例值**: "open_department_id"
    ///
    /// **可选值**:
    ///
    /// `department_id`: 支持用户自定义配置的部门 ID。自定义配置时可复用已删除的 department_id，因此在未删除的部门范围内 department_id 具有唯一性。
    ///
    /// `open_department_id`: 由系统自动生成的部门 ID，ID 前缀固定为 `od-`，在租户内全局唯一。
    #[api(kind = "body", name = "department_id_type")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BindContactUnitDepartmentRespInner {
    #[serde(flatten)]
    data: Option<BindContactUnitDepartmentResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BindContactUnitDepartmentResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: (),
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
            BindContactUnitDepartmentReq,
        ) -> Result<(BindContactUnitDepartmentResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BindContactUnitDepartmentReq,
                )
                    -> Result<(BindContactUnitDepartmentResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_bind_contact_unit_department<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BindContactUnitDepartmentReq,
            BindContactUnitDepartmentResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_bind_contact_unit_department(
            &self,
            req: &BindContactUnitDepartmentReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BindContactUnitDepartmentReq,
                BindContactUnitDepartmentResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::contact::bind_contact_unit_department::{
            BindContactUnitDepartmentReq, BindContactUnitDepartmentResp,
            BindContactUnitDepartmentRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_bind_contact_unit_department(|_| {
                Ok((
                    BindContactUnitDepartmentResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .contact()
            .bind_contact_unit_department(BindContactUnitDepartmentReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .bind_contact_unit_department(BindContactUnitDepartmentReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "unit_id": "BU121",
    "department_id": "od-4e6ac4d14bcd5071a37a39de902c7141",
    "department_id_type": "open_department_id"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BindContactUnitDepartmentReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {}
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<BindContactUnitDepartmentRespInner>(RESP);
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