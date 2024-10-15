//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/delete>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::message::MessageService;

impl<'c, IStore: Store, IClient: HttpClient> MessageService<'c, IStore, IClient> {
    /// **api 版本: 2024-06-21T09:04:22+00:00**
    ///
    /// ## 撤回消息
    ///
    /// 机器人撤回机器人自己发送的消息或群主撤回群内消息。
    ///
    /// 注意事项:
    ///
    /// - 需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)  ，撤回消息时机器人仍需要在会话内
    ///
    /// - 机器人可以撤回单聊和群组内，自己发送 且 发送时间不超过租户管理员配置的可撤回时限的消息（默认为24小时）
    ///
    /// - 若机器人要撤回群内他人发送的消息，则机器人必须是该群的群主、管理员 或者 创建者，且消息发送时间不超过1年
    ///
    /// - 无法撤回通过「[批量发送消息](https://open.feishu.cn/document/ukTMukTMukTM/ucDO1EjL3gTNx4yN4UTM)」接口发送的消息
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/im-v1/message/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fim-v1%2Fmessage%2Fdelete>
    pub async fn delete_message(
        &self,
        req: DeleteMessageReq,
    ) -> Result<(DeleteMessageResp, CommonResponse), Error> {
        self.delete_message_with_opt(req, Default::default()).await
    }

    /// 参见 [delete_message](#method.delete_message) 函数
    pub async fn delete_message_with_opt(
        &self,
        req: DeleteMessageReq,
        method_option: MethodOption,
    ) -> Result<(DeleteMessageResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_message(&req) {
                tracing::info!("[lark] Message#DeleteMessage **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Message#DeleteMessage call api");

        let req = ApiRequest {
            scope: "Message",
            api: "DeleteMessage",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/messages/:message_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteMessageRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteMessageReq {
    /// 待撤回的消息的ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "om_dc13264520392913993dd051dba21dcf"
    #[api(kind = "path", name = "message_id")]
    pub message_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteMessageRespInner {
    #[serde(flatten)]
    data: Option<DeleteMessageResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteMessageResp {
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

    use self::gen::message::MessageServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(DeleteMessageReq) -> Result<(DeleteMessageResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(DeleteMessageReq) -> Result<(DeleteMessageResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MessageServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_message<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, DeleteMessageReq, DeleteMessageResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_message(
            &self,
            req: &DeleteMessageReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteMessageReq, DeleteMessageResp, Arc<dyn MockFunc>>(
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
        api::gen::message::delete_message::{
            DeleteMessageReq, DeleteMessageResp, DeleteMessageRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .message()
            .mock()
            .mock_delete_message(|_| Ok((DeleteMessageResp::default(), CommonResponse::default())))
            .build();
        let res = lark
            .message()
            .delete_message(DeleteMessageReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .message()
            .delete_message(DeleteMessageReq::default())
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
        let res = serde_json::from_str::<DeleteMessageRespInner>(RESP);
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