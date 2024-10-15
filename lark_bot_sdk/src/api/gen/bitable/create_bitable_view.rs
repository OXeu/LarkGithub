//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/create>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::bitable::BitableService;

impl<'c, IStore: Store, IClient: HttpClient> BitableService<'c, IStore, IClient> {
    /// **api 版本: 2023-07-28T08:21:51+00:00**
    ///
    /// ## 新增视图
    ///
    /// 在数据表中新增一个视图
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/bitable-v1/app-table-view/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fbitable-v1%2Fapp-table-view%2Fcreate>
    pub async fn create_bitable_view(
        &self,
        req: CreateBitableViewReq,
    ) -> Result<(CreateBitableViewResp, CommonResponse), Error> {
        self.create_bitable_view_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_bitable_view](#method.create_bitable_view) 函数
    pub async fn create_bitable_view_with_opt(
        &self,
        req: CreateBitableViewReq,
        method_option: MethodOption,
    ) -> Result<(CreateBitableViewResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_bitable_view(&req) {
                tracing::info!("[lark] Bitable#CreateBitableView **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Bitable#CreateBitableView call api");

        let req = ApiRequest {
            scope: "Bitable",
            api: "CreateBitableView",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateBitableViewRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateBitableViewReq {
    /// base app token
    ///
    /// **示例值**: "appbcbWCzen6D8dezhoCH2RpMAh"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[api(kind = "path", name = "app_token")]
    pub app_token: String,
    /// table id
    ///
    /// **示例值**: "tblsRc9GRRXKqhvW"
    #[api(kind = "path", name = "table_id")]
    pub table_id: String,

    /// 视图名字
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "表格视图1"
    #[api(kind = "body", name = "view_name")]
    pub view_name: String,
    /// 视图类型
    ///
    /// **示例值**: "grid"
    ///
    /// **可选值**:
    ///
    /// `Grid`: 表格视图
    ///
    /// `Kanban`: 看板视图
    ///
    /// `Gallery`: 画册视图
    ///
    /// `Gantt`: 甘特视图
    ///
    /// `Form`: 表单视图
    #[api(kind = "body", name = "view_type")]
    pub view_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateBitableViewRespInner {
    #[serde(flatten)]
    data: Option<CreateBitableViewResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateBitableViewResp {
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
    /// 视图
    #[serde(
        rename = "view",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub view: AppTableViewSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppTableViewSubResp {
    /// 视图Id
    ///
    /// **示例值**: "vewTpR1urY"
    #[serde(
        rename = "view_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub view_id: String,
    /// 视图名字
    ///
    /// **示例值**: "甘特视图1"
    #[serde(
        rename = "view_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub view_name: String,
    /// 视图类型
    ///
    /// **示例值**: "gantt"
    #[serde(
        rename = "view_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub view_type: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::bitable::BitableServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(CreateBitableViewReq) -> Result<(CreateBitableViewResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(CreateBitableViewReq) -> Result<(CreateBitableViewResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> BitableServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_bitable_view<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CreateBitableViewReq, CreateBitableViewResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_bitable_view(
            &self,
            req: &CreateBitableViewReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateBitableViewReq, CreateBitableViewResp, Arc<dyn MockFunc>>(
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
        api::gen::bitable::create_bitable_view::{
            CreateBitableViewReq, CreateBitableViewResp, CreateBitableViewRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .bitable()
            .mock()
            .mock_create_bitable_view(|_| {
                Ok((CreateBitableViewResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .bitable()
            .create_bitable_view(CreateBitableViewReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .bitable()
            .create_bitable_view(CreateBitableViewReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "view_name": "表格视图1",
    "view_type": "grid"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateBitableViewReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "view": {
            "view_id": "vewTpR1urY",
            "view_name": "甘特视图1",
            "view_type": "gantt"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateBitableViewRespInner>(RESP);
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
