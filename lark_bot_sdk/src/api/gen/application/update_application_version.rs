//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/patch>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::application::ApplicationService;

impl<'c, IStore: Store, IClient: HttpClient> ApplicationService<'c, IStore, IClient> {
    /// **api 版本: 2023-07-21T07:21:41+00:00**
    ///
    /// ## 更新应用审核状态
    ///
    /// 通过接口来更新应用版本的审核结果：通过后应用可以直接上架；拒绝后则开发者可以看到拒绝理由，并在修改后再次申请发布。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/application-v6/application/patch-2>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fapplication-v6%2Fapplication%2Fpatch-2>
    pub async fn update_application_version(
        &self,
        req: UpdateApplicationVersionReq,
    ) -> Result<(UpdateApplicationVersionResp, CommonResponse), Error> {
        self.update_application_version_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_application_version](#method.update_application_version) 函数
    pub async fn update_application_version_with_opt(
        &self,
        req: UpdateApplicationVersionReq,
        method_option: MethodOption,
    ) -> Result<(UpdateApplicationVersionResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_application_version(&req) {
                tracing::info!("[lark] Application#UpdateApplicationVersion **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Application#UpdateApplicationVersion call api");

        let req = ApiRequest {
            scope: "Application",
            api: "UpdateApplicationVersion",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/application/v6/applications/:app_id/app_versions/:version_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateApplicationVersionRespInner, _) =
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
pub struct UpdateApplicationVersionReq {
    /// 应用 id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "cli_9f3ca975326b501b"
    #[api(kind = "path", name = "app_id")]
    pub app_id: String,
    /// 唯一标识应用版本的 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "oav_d317f090b7258ad0372aa53963cda70d"
    #[api(kind = "path", name = "version_id")]
    pub version_id: String,
    /// 用户 ID 类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "open_id"
    ///
    /// **可选值**:
    ///
    /// `open_id`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// `union_id`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)
    ///
    /// `user_id`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
    /// 操作者的 open_id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_4065981088f8ef67a504ba8bd6b24d85"
    #[api(kind = "query", name = "operator_id", v_type = "var", option = "false")]
    pub operator_id: String,
    /// 当修改版本状态为被驳回时，这一项必填
    ///
    /// **示例值**: "拒绝理由"
    #[api(
        kind = "query",
        name = "reject_reason",
        v_type = "var",
        option = "false"
    )]
    pub reject_reason: String,
    /// 版本状态
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `unknown`: 未知状态
    ///
    /// `audited`: 审核通过
    ///
    /// `reject`: 审核拒绝
    ///
    /// `under_audit`: 审核中
    ///
    /// `unaudit`: 未提交审核
    #[api(kind = "body", name = "status")]
    pub status: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateApplicationVersionRespInner {
    #[serde(flatten)]
    data: Option<UpdateApplicationVersionResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateApplicationVersionResp {
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

    use self::gen::application::ApplicationServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            UpdateApplicationVersionReq,
        ) -> Result<(UpdateApplicationVersionResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateApplicationVersionReq,
                ) -> Result<(UpdateApplicationVersionResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ApplicationServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_application_version<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateApplicationVersionReq,
            UpdateApplicationVersionResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_application_version(
            &self,
            req: &UpdateApplicationVersionReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateApplicationVersionReq,
                UpdateApplicationVersionResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::application::update_application_version::{
            UpdateApplicationVersionReq, UpdateApplicationVersionResp,
            UpdateApplicationVersionRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .application()
            .mock()
            .mock_update_application_version(|_| {
                Ok((
                    UpdateApplicationVersionResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .application()
            .update_application_version(UpdateApplicationVersionReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .application()
            .update_application_version(UpdateApplicationVersionReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "status": 1
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateApplicationVersionReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateApplicationVersionRespInner>(RESP);
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