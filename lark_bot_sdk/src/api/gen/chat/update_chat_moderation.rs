//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-moderation/update>
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
    /// **api 版本: 2024-06-21T09:04:26+00:00**
    ///
    /// ## 更新群发言权限
    ///
    /// 更新群组的发言权限设置，可设置为全员可发言、仅管理员可发言  或 指定用户可发言。
    ///
    /// 注意事项：
    ///
    /// - 需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
    ///
    /// - 若以用户授权调用接口，**当授权用户是群主**时，可更新群发言权限
    ///
    /// - 若以租户授权调用接口(即以机器人身份调用接口)，当**机器人是群主** 或者 **机器人是群组创建者、具备==更新应用所创建群的群信息==权限且仍在群内**时，可更新群发言权限
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-moderation/update>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/group/chat/update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fgroup%2Fchat%2Fupdate>
    pub async fn update_chat_moderation(
        &self,
        req: UpdateChatModerationReq,
    ) -> Result<(UpdateChatModerationResp, CommonResponse), Error> {
        self.update_chat_moderation_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_chat_moderation](#method.update_chat_moderation) 函数
    pub async fn update_chat_moderation_with_opt(
        &self,
        req: UpdateChatModerationReq,
        method_option: MethodOption,
    ) -> Result<(UpdateChatModerationResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_chat_moderation(&req) {
                tracing::info!("[lark] Chat#UpdateChatModeration **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Chat#UpdateChatModeration call api");

        let req = ApiRequest {
            scope: "Chat",
            api: "UpdateChatModeration",
            method: http::Method::PUT,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/chats/:chat_id/moderation",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateChatModerationRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateChatModerationReq {
    /// 群 ID，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "oc_a0553eda9014c201e6969b478895c230"
    #[api(kind = "path", name = "chat_id")]
    pub chat_id: String,
    /// 用户 ID 类型
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
    /// 群发言模式（all_members/only_owner/moderator_list，其中 moderator_list 表示部分用户可发言的模式）
    ///
    /// **示例值**: "moderator_list"
    #[api(kind = "body", name = "moderation_setting")]
    pub moderation_setting: Option<String>,
    /// 选择部分用户可发言模式时，添加的可发言用户列表（自动过滤不在群内的用户）。推荐使用 OpenID，获取方式可参考文档[如何获取 Open ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// **示例值**: "["ou_7d8a6e6df7621556ce0d21922b676706ccs"]"
    #[api(kind = "body", name = "moderator_added_list")]
    pub moderator_added_list: Vec<Option<String>>,
    /// 选择部分用户可发言模式时，移除的可发言用户列表（自动过滤不在群内的用户）。推荐使用 OpenID，获取方式可参考文档[如何获取 Open ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// **示例值**: "["ou_8d7h6e6df76215566d8dbs92k6767c4d87"]"
    #[api(kind = "body", name = "moderator_removed_list")]
    pub moderator_removed_list: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateChatModerationRespInner {
    #[serde(flatten)]
    data: Option<UpdateChatModerationResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateChatModerationResp {
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
        Fn(UpdateChatModerationReq) -> Result<(UpdateChatModerationResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateChatModerationReq,
                ) -> Result<(UpdateChatModerationResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ChatServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_chat_moderation<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateChatModerationReq,
            UpdateChatModerationResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_chat_moderation(
            &self,
            req: &UpdateChatModerationReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateChatModerationReq, UpdateChatModerationResp, Arc<dyn MockFunc>>(
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
        api::gen::chat::update_chat_moderation::{
            UpdateChatModerationReq, UpdateChatModerationResp, UpdateChatModerationRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .chat()
            .mock()
            .mock_update_chat_moderation(|_| {
                Ok((
                    UpdateChatModerationResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .chat()
            .update_chat_moderation(UpdateChatModerationReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .chat()
            .update_chat_moderation(UpdateChatModerationReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "moderation_setting": "moderator_list",
    "moderator_added_list": [
        "4d7a3c6g"
    ],
    "moderator_removed_list": [
        "4d7a3c6g"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateChatModerationReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateChatModerationRespInner>(RESP);
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