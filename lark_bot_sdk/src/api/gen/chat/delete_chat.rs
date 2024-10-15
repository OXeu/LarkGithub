//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/delete>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::chat::ChatService;

impl<'c, IStore: Store, IClient: HttpClient> ChatService<'c, IStore, IClient> {
    /// **api 版本: 2024-06-21T09:04:30+00:00**
    ///
    /// ## 解散群
    ///
    /// 解散群组。
    ///
    /// 注意事项：
    ///
    /// - 应用需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
    ///
    /// - 如果使用tenant_access_token，需要机器人符合以下任一情况才可解散群：
    ///
    /// - 机器人是群主
    ///
    /// - 机器人是群的创建者且具备==更新应用所创建群的群信息==权限
    ///
    /// - 如果使用user_access_token，需要对应的用户是群主才可解散群
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/group/chat/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fgroup%2Fchat%2Fdelete>
    pub async fn delete_chat(
        &self,
        req: DeleteChatReq,
    ) -> Result<(DeleteChatResp, CommonResponse), Error> {
        self.delete_chat_with_opt(req, Default::default()).await
    }

    /// 参见 [delete_chat](#method.delete_chat) 函数
    pub async fn delete_chat_with_opt(
        &self,
        req: DeleteChatReq,
        method_option: MethodOption,
    ) -> Result<(DeleteChatResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_chat(&req) {
                tracing::info!("[lark] Chat#DeleteChat **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Chat#DeleteChat call api");

        let req = ApiRequest {
            scope: "Chat",
            api: "DeleteChat",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/chats/:chat_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeleteChatRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeleteChatReq {
    /// 群 ID，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// **注意**：仅支持群模式为`group`的群组ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "oc_a0553eda9014c201e6969b478895c230"
    #[api(kind = "path", name = "chat_id")]
    pub chat_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeleteChatRespInner {
    #[serde(flatten)]
    data: Option<DeleteChatResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteChatResp {
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

    use self::gen::chat::ChatServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(DeleteChatReq) -> Result<(DeleteChatResp, CommonResponse), Error> + Send + Sync + 'static
    {
    }
    impl<
            T: Fn(DeleteChatReq) -> Result<(DeleteChatResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ChatServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_chat<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, DeleteChatReq, DeleteChatResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_chat(
            &self,
            req: &DeleteChatReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeleteChatReq, DeleteChatResp, Arc<dyn MockFunc>>(
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
        api::gen::chat::delete_chat::{DeleteChatReq, DeleteChatResp, DeleteChatRespInner},
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .chat()
            .mock()
            .mock_delete_chat(|_| Ok((DeleteChatResp::default(), CommonResponse::default())))
            .build();
        let res = lark.chat().delete_chat(DeleteChatReq::default()).await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark.chat().delete_chat(DeleteChatReq::default()).await;
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
        let res = serde_json::from_str::<DeleteChatRespInner>(RESP);
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
