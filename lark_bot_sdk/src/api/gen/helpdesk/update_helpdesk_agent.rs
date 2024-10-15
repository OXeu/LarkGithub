//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent/patch>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::helpdesk::HelpdeskService;

impl<'c, IStore: Store, IClient: HttpClient> HelpdeskService<'c, IStore, IClient> {
    /// **api 版本: 2023-08-15T07:34:18+00:00**
    ///
    /// ## 更新客服信息
    ///
    /// 更新客服状态等信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/helpdesk-v1/agent-function/agent/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhelpdesk-v1%2Fagent-function%2Fagent%2Fpatch>
    pub async fn update_helpdesk_agent(
        &self,
        req: UpdateHelpdeskAgentReq,
    ) -> Result<(UpdateHelpdeskAgentResp, CommonResponse), Error> {
        self.update_helpdesk_agent_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_helpdesk_agent](#method.update_helpdesk_agent) 函数
    pub async fn update_helpdesk_agent_with_opt(
        &self,
        req: UpdateHelpdeskAgentReq,
        method_option: MethodOption,
    ) -> Result<(UpdateHelpdeskAgentResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_helpdesk_agent(&req) {
                tracing::info!("[lark] Helpdesk#UpdateHelpdeskAgent **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Helpdesk#UpdateHelpdeskAgent call api");

        let req = ApiRequest {
            scope: "Helpdesk",
            api: "UpdateHelpdeskAgent",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/helpdesk/v1/agents/:agent_id",
            param_data: req.gen_param(),
            method_option,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateHelpdeskAgentRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateHelpdeskAgentReq {
    /// 客服id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_14777d82ffef0f707de5a8c7ff2c5ebe"
    #[api(kind = "path", name = "agent_id")]
    pub agent_id: String,

    /// agent status，1：在线；2：离线
    ///
    /// **示例值**: "1"
    #[api(kind = "body", name = "status")]
    pub status: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateHelpdeskAgentRespInner {
    #[serde(flatten)]
    data: Option<UpdateHelpdeskAgentResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateHelpdeskAgentResp {
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

    use self::gen::helpdesk::HelpdeskServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(UpdateHelpdeskAgentReq) -> Result<(UpdateHelpdeskAgentResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateHelpdeskAgentReq,
                ) -> Result<(UpdateHelpdeskAgentResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HelpdeskServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_helpdesk_agent<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdateHelpdeskAgentReq, UpdateHelpdeskAgentResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_helpdesk_agent(
            &self,
            req: &UpdateHelpdeskAgentReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateHelpdeskAgentReq, UpdateHelpdeskAgentResp, Arc<dyn MockFunc>>(
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
        api::gen::helpdesk::update_helpdesk_agent::{
            UpdateHelpdeskAgentReq, UpdateHelpdeskAgentResp, UpdateHelpdeskAgentRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .helpdesk()
            .mock()
            .mock_update_helpdesk_agent(|_| {
                Ok((
                    UpdateHelpdeskAgentResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .helpdesk()
            .update_helpdesk_agent(UpdateHelpdeskAgentReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .helpdesk()
            .update_helpdesk_agent(UpdateHelpdeskAgentReq::default())
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
        if let Err(e) = serde_json::from_str::<super::UpdateHelpdeskAgentReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateHelpdeskAgentRespInner>(RESP);
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