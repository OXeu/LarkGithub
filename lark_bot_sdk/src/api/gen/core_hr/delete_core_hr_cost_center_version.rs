//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center-version/delete>
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
    /// **api 版本: 2024-02-06T02:27:28+00:00**
    ///
    /// ## 撤销成本中心版本
    ///
    /// 撤销成本中心版本
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center-version/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/organization-management/cost_center/cost_center-version/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Forganization-management%2Fcost_center%2Fcost_center-version%2Fdelete>
    pub async fn delete_core_hr_cost_center_version(
        &self,
        req: DeleteCoreHrCostCenterVersionReq,
    ) -> Result<(DeleteCoreHrCostCenterVersionResp, CommonResponse), Error> {
        self.delete_core_hr_cost_center_version_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_core_hr_cost_center_version](#method.delete_core_hr_cost_center_version) 函数
    pub async fn delete_core_hr_cost_center_version_with_opt(
        &self,
        req: DeleteCoreHrCostCenterVersionReq,
        method_option: MethodOption,
    ) -> Result<(DeleteCoreHrCostCenterVersionResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_delete_core_hr_cost_center_version(&req)
            {
                tracing::info!("[lark] CoreHr#DeleteCoreHrCostCenterVersion **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#DeleteCoreHrCostCenterVersion call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "DeleteCoreHrCostCenterVersion",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v2/cost_centers/:cost_center_id/versions/:version_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteCoreHrCostCenterVersionRespInner, _) =
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
pub struct DeleteCoreHrCostCenterVersionReq {
    /// 成本中心ID
    ///
    /// **示例值**: "6862995757234914824"
    #[api(kind = "path", name = "cost_center_id")]
    pub cost_center_id: String,
    /// 版本ID
    ///
    /// **示例值**: "6862995757234914824"
    #[api(kind = "path", name = "version_id")]
    pub version_id: String,

    /// 操作原因
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "随着组织架构调整，该成本中心不再使用"
    #[api(kind = "body", name = "operation_reason")]
    pub operation_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteCoreHrCostCenterVersionRespInner {
    #[serde(flatten)]
    data: Option<DeleteCoreHrCostCenterVersionResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteCoreHrCostCenterVersionResp {
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

    use self::gen::core_hr::CoreHrServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            DeleteCoreHrCostCenterVersionReq,
        ) -> Result<(DeleteCoreHrCostCenterVersionResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeleteCoreHrCostCenterVersionReq,
                )
                    -> Result<(DeleteCoreHrCostCenterVersionResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_core_hr_cost_center_version<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            DeleteCoreHrCostCenterVersionReq,
            DeleteCoreHrCostCenterVersionResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_core_hr_cost_center_version(
            &self,
            req: &DeleteCoreHrCostCenterVersionReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                DeleteCoreHrCostCenterVersionReq,
                DeleteCoreHrCostCenterVersionResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::delete_core_hr_cost_center_version::{
            DeleteCoreHrCostCenterVersionReq, DeleteCoreHrCostCenterVersionResp,
            DeleteCoreHrCostCenterVersionRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_delete_core_hr_cost_center_version(|_| {
                Ok((
                    DeleteCoreHrCostCenterVersionResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .delete_core_hr_cost_center_version(DeleteCoreHrCostCenterVersionReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .delete_core_hr_cost_center_version(DeleteCoreHrCostCenterVersionReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "operation_reason": "随着组织架构调整，该成本中心不再使用"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::DeleteCoreHrCostCenterVersionReqBody>(REQ) {
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
        let res = serde_json::from_str::<DeleteCoreHrCostCenterVersionRespInner>(RESP);
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
