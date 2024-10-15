//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/notification/execute_send>
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
    /// **api 版本: 2023-08-15T07:34:24+00:00**
    ///
    /// ## 执行推送
    ///
    /// 审核通过后调用此接口设置推送时间，等待调度系统调度，发送消息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/notification/execute_send>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/helpdesk-v1/notification/execute_send>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhelpdesk-v1%2Fnotification%2Fexecute_send>
    pub async fn execute_send_helpdesk_notification(
        &self,
        req: ExecuteSendHelpdeskNotificationReq,
    ) -> Result<(ExecuteSendHelpdeskNotificationResp, CommonResponse), Error> {
        self.execute_send_helpdesk_notification_with_opt(req, Default::default())
            .await
    }

    /// 参见 [execute_send_helpdesk_notification](#method.execute_send_helpdesk_notification) 函数
    pub async fn execute_send_helpdesk_notification_with_opt(
        &self,
        req: ExecuteSendHelpdeskNotificationReq,
        method_option: MethodOption,
    ) -> Result<(ExecuteSendHelpdeskNotificationResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_execute_send_helpdesk_notification(&req)
            {
                tracing::info!("[lark] Helpdesk#ExecuteSendHelpdeskNotification **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Helpdesk#ExecuteSendHelpdeskNotification call api");

        let req = ApiRequest {
            scope: "Helpdesk",
            api: "ExecuteSendHelpdeskNotification",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/helpdesk/v1/notifications/:notification_id/execute_send",
            param_data: req.gen_param(),
            method_option,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (ExecuteSendHelpdeskNotificationRespInner, _) =
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
pub struct ExecuteSendHelpdeskNotificationReq {
    /// 创建接口返回的唯一id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6985032626234982420"
    #[api(kind = "path", name = "notification_id")]
    pub notification_id: String,

    /// 发送时间戳(毫秒)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1624326025000"
    #[api(kind = "body", name = "send_at")]
    pub send_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct ExecuteSendHelpdeskNotificationRespInner {
    #[serde(flatten)]
    data: Option<ExecuteSendHelpdeskNotificationResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExecuteSendHelpdeskNotificationResp {
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
        Fn(
            ExecuteSendHelpdeskNotificationReq,
        ) -> Result<(ExecuteSendHelpdeskNotificationResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    ExecuteSendHelpdeskNotificationReq,
                )
                    -> Result<(ExecuteSendHelpdeskNotificationResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HelpdeskServiceMocker<'c, IStore, IClient> {
        pub fn mock_execute_send_helpdesk_notification<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            ExecuteSendHelpdeskNotificationReq,
            ExecuteSendHelpdeskNotificationResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_execute_send_helpdesk_notification(
            &self,
            req: &ExecuteSendHelpdeskNotificationReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                ExecuteSendHelpdeskNotificationReq,
                ExecuteSendHelpdeskNotificationResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::helpdesk::execute_send_helpdesk_notification::{
            ExecuteSendHelpdeskNotificationReq, ExecuteSendHelpdeskNotificationResp,
            ExecuteSendHelpdeskNotificationRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .helpdesk()
            .mock()
            .mock_execute_send_helpdesk_notification(|_| {
                Ok((
                    ExecuteSendHelpdeskNotificationResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .helpdesk()
            .execute_send_helpdesk_notification(ExecuteSendHelpdeskNotificationReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .helpdesk()
            .execute_send_helpdesk_notification(ExecuteSendHelpdeskNotificationReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "send_at": "1624326025000"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::ExecuteSendHelpdeskNotificationReqBody>(REQ) {
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
        let res = serde_json::from_str::<ExecuteSendHelpdeskNotificationRespInner>(RESP);
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
