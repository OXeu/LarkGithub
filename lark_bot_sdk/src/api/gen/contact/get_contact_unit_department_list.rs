//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/list_department>
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
    /// **api 版本: 2024-07-05T08:16:18+00:00**
    ///
    /// ## 获取单位绑定的部门列表
    ///
    /// 调用该接口获取指定单位绑定的部门列表。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/list_department>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/contact-v3/unit/list_department>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcontact-v3%2Funit%2Flist_department>
    pub async fn get_contact_unit_department_list(
        &self,
        req: GetContactUnitDepartmentListReq,
    ) -> Result<(GetContactUnitDepartmentListResp, CommonResponse), Error> {
        self.get_contact_unit_department_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_contact_unit_department_list](#method.get_contact_unit_department_list) 函数
    pub async fn get_contact_unit_department_list_with_opt(
        &self,
        req: GetContactUnitDepartmentListReq,
        method_option: MethodOption,
    ) -> Result<(GetContactUnitDepartmentListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_contact_unit_department_list(&req) {
                tracing::info!("[lark] Contact#GetContactUnitDepartmentList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#GetContactUnitDepartmentList call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "GetContactUnitDepartmentList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/unit/list_department",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetContactUnitDepartmentListRespInner, _) =
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
pub struct GetContactUnitDepartmentListReq {
    /// 单位 ID。
    ///
    /// 当你在创建单位时，可以在返回结果中获取单位 ID。你也可以调用[获取单位列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/list)接口，获取单位 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "BU121"
    #[api(kind = "query", name = "unit_id", v_type = "var", option = "false")]
    pub unit_id: String,
    /// 此次调用中的部门 ID 类型。关于部门 ID 的详细介绍，可参见[部门 ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/field-overview#23857fe0)。
    ///
    /// **示例值**: "open_department_id"
    ///
    /// **可选值**:
    ///
    /// `department_id`: 支持用户自定义配置的部门 ID。自定义配置时可复用已删除的 department_id，因此在未删除的部门范围内 department_id 具有唯一性。
    ///
    /// `open_department_id`: 由系统自动生成的部门 ID，ID 前缀固定为 `od-`，在租户内全局唯一。
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "AQD9/Rn9eij9Pm39ED40/dk53s4Ebp882DYfFaPFbz00L4CMZJrqGdzNyc8BcZtDbwVUvRmQTvyMYicnGWrde9X56TgdBuS+JKiSIkdexPw="
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 分页大小，用于限制一次请求所返回的数据条目数。
    ///
    /// **示例值**: "50"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetContactUnitDepartmentListRespInner {
    #[serde(flatten)]
    data: Option<GetContactUnitDepartmentListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetContactUnitDepartmentListResp {
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
    /// 单位绑定的部门列表。
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "departmentlist",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub departmentlist: Vec<UnitDepartmentSubResp>,
    /// 是否还有更多项
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "AQD9/Rn9eij9Pm39ED40/dk53s4Ebp882DYfFaPFbz00L4CMZJrqGdzNyc8BcZtDbwVUvRmQTvyMYicnGWrde9X56TgdBuS+JdtW="
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UnitDepartmentSubResp {
    /// 单位 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "BU121"
    #[serde(
        rename = "unit_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub unit_id: String,
    /// 部门 ID。你可以调用[获取单个部门信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/get)接口，获取部门详情。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "od-4e6ac4d14bcd5071a37a39de902c7141"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
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
            GetContactUnitDepartmentListReq,
        ) -> Result<(GetContactUnitDepartmentListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetContactUnitDepartmentListReq,
                )
                    -> Result<(GetContactUnitDepartmentListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_contact_unit_department_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetContactUnitDepartmentListReq,
            GetContactUnitDepartmentListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_contact_unit_department_list(
            &self,
            req: &GetContactUnitDepartmentListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetContactUnitDepartmentListReq,
                GetContactUnitDepartmentListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::contact::get_contact_unit_department_list::{
            GetContactUnitDepartmentListReq, GetContactUnitDepartmentListResp,
            GetContactUnitDepartmentListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_get_contact_unit_department_list(|_| {
                Ok((
                    GetContactUnitDepartmentListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .contact()
            .get_contact_unit_department_list(GetContactUnitDepartmentListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .get_contact_unit_department_list(GetContactUnitDepartmentListReq::default())
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
        "departmentlist": [
            {
                "unit_id": "BU121",
                "department_id": "od-4e6ac4d14bcd5071a37a39de902c7141"
            }
        ],
        "has_more": true,
        "page_token": "AQD9/Rn9eij9Pm39ED40/dk53s4Ebp882DYfFaPFbz00L4CMZJrqGdzNyc8BcZtDbwVUvRmQTvyMYicnGWrde9X56TgdBuS+JdtW="
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetContactUnitDepartmentListRespInner>(RESP);
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