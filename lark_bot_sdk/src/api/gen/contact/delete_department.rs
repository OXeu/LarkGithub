//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/delete>
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
    /// **api 版本: 2024-07-05T08:11:14+00:00**
    ///
    /// ## 删除部门
    ///
    /// 调用该接口从通讯录中删除指定的部门。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/contact-v3/department/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcontact-v3%2Fdepartment%2Fdelete>
    pub async fn delete_department(
        &self,
        req: DeleteDepartmentReq,
    ) -> Result<(DeleteDepartmentResp, CommonResponse), Error> {
        self.delete_department_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_department](#method.delete_department) 函数
    pub async fn delete_department_with_opt(
        &self,
        req: DeleteDepartmentReq,
        method_option: MethodOption,
    ) -> Result<(DeleteDepartmentResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_department(&req) {
                tracing::info!("[lark] Contact#DeleteDepartment **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#DeleteDepartment call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "DeleteDepartment",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/departments/:department_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteDepartmentRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteDepartmentReq {
    /// 部门 ID，ID 类型需要与查询参数 department_id_type 的取值保持一致。ID 获取方式说明：
    ///
    /// - 调用[创建部门](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/create)接口后，可从返回结果中获取到部门 ID 信息。
    ///
    /// - 部门 API 提供了多种获取其他部门 ID 的方式，如[获取子部门列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/children)、[获取父部门信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/parent)、[搜索部门](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/search)，你可以选择合适的 API 进行查询。
    ///
    /// **示例值**: "D096"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `64` 字符
    #[api(kind = "path", name = "department_id")]
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
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteDepartmentRespInner {
    #[serde(flatten)]
    data: Option<DeleteDepartmentResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteDepartmentResp {
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
        Fn(DeleteDepartmentReq) -> Result<(DeleteDepartmentResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(DeleteDepartmentReq) -> Result<(DeleteDepartmentResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_department<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, DeleteDepartmentReq, DeleteDepartmentResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_department(
            &self,
            req: &DeleteDepartmentReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteDepartmentReq, DeleteDepartmentResp, Arc<dyn MockFunc>>(
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
        api::gen::contact::delete_department::{
            DeleteDepartmentReq, DeleteDepartmentResp, DeleteDepartmentRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_delete_department(|_| {
                Ok((DeleteDepartmentResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .contact()
            .delete_department(DeleteDepartmentReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .delete_department(DeleteDepartmentReq::default())
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
    "data": {}
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<DeleteDepartmentRespInner>(RESP);
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