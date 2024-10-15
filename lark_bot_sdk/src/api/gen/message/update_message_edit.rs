//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/update>
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
    /// **api 版本: 2024-06-05T03:08:38+00:00**
    ///
    /// ## 编辑消息
    ///
    /// 编辑已发送的消息内容，当前支持编辑文本、富文本消息。如需更新消息卡片，请参考[更新应用发送的消息卡片](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/patch)。
    ///
    /// **注意事项**:
    ///
    /// - 一条消息最多可编辑20次
    ///
    /// - 仅可编辑操作者自己发送的消息
    ///
    /// - 不可编辑已撤回，已删除，超出可编辑时间的消息
    ///
    /// - 操作者必须在消息所属的群中且具备发言权限才可以编辑消息
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/update>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/im-v1/message/update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fim-v1%2Fmessage%2Fupdate>
    pub async fn update_message_edit(
        &self,
        req: UpdateMessageEditReq,
    ) -> Result<(UpdateMessageEditResp, CommonResponse), Error> {
        self.update_message_edit_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_message_edit](#method.update_message_edit) 函数
    pub async fn update_message_edit_with_opt(
        &self,
        req: UpdateMessageEditReq,
        method_option: MethodOption,
    ) -> Result<(UpdateMessageEditResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_message_edit(&req) {
                tracing::info!("[lark] Message#UpdateMessageEdit **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Message#UpdateMessageEdit call api");

        let req = ApiRequest {
            scope: "Message",
            api: "UpdateMessageEdit",
            method: http::Method::PUT,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/messages/:message_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateMessageEditRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateMessageEditReq {
    /// 待编辑的消息的ID，仅支持文本（text）或富文本（post）消息，详情参见[消息ID说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "om_dc13264520392913993dd051dba21dcf"
    #[api(kind = "path", name = "message_id")]
    pub message_id: String,

    /// 消息的类型，当前仅支持文本（text）和富文本（post）类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "text"
    #[api(kind = "body", name = "msg_type")]
    pub msg_type: String,
    /// 消息内容，JSON结构序列化后的字符串。不同msg_type对应不同内容，具体格式说明参考：[发送消息Content](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json)
    ///
    /// **注意：**
    ///
    /// - JSON字符串需进行转义，如换行符转义后为`\\n`
    ///
    /// - 文本消息请求体最大不能超过150KB
    ///
    /// - 富文本消息请求体最大不能超过30KB
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "{\"text\":\"test content\"}"
    #[api(kind = "body", name = "content")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateMessageEditRespInner {
    #[serde(flatten)]
    data: Option<UpdateMessageEditResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateMessageEditResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: MessageSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MessageSubResp {
    /// 消息id open_message_id
    ///
    /// **示例值**: "om_dc13264520392913993dd051dba21dcf"
    #[serde(
        rename = "message_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub message_id: String,
    /// 根消息id open_message_id
    ///
    /// **示例值**: "om_40eb06e7b84dc71c03e009ad3c754195"
    #[serde(
        rename = "root_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub root_id: String,
    /// 父消息的id open_message_id
    ///
    /// **示例值**: "om_d4be107c616aed9c1da8ed8068570a9f"
    #[serde(
        rename = "parent_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_id: String,
    /// 消息所属的话题 ID（不返回说明该消息非话题消息），说明参见：[话题介绍](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/thread-introduction)
    ///
    /// **示例值**: "omt_d4be107c616a"
    #[serde(
        rename = "thread_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub thread_id: String,
    /// 消息类型 text post card image等等
    ///
    /// **示例值**: "card"
    #[serde(
        rename = "msg_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub msg_type: String,
    /// 消息生成的时间戳(毫秒)
    ///
    /// **示例值**: "1609296809"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 消息更新的时间戳
    ///
    /// **示例值**: "1609336806"
    #[serde(
        rename = "update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_time: String,
    /// 消息是否被撤回
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "deleted",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub deleted: bool,
    /// 消息是否被更新
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "updated",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub updated: bool,
    /// 所属的群
    ///
    /// **示例值**: "oc_5ad11d72b830411d72b836c20"
    #[serde(
        rename = "chat_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_id: String,
    /// 发送者，可以是用户或应用
    ///
    /// **示例值**: "object"
    #[serde(
        rename = "sender",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub sender: SenderSubResp,
    /// 消息内容，JSON结构
    ///
    /// **示例值**: "json结构"
    #[serde(
        rename = "body",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body: MessageBodySubResp,
    /// 被艾特的人或应用的id
    #[serde(
        rename = "mentions",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mentions: Vec<MentionSubResp>,
    /// 合并消息的上一层级消息id open_message_id
    ///
    /// **示例值**: "om_40eb06e7b84dc71c03e00ida3c754892"
    #[serde(
        rename = "upper_message_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub upper_message_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SenderSubResp {
    /// 该字段标识发送者的id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "cli_9f427eec54ae901b"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 该字段标识发送者的id类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "app_id"
    #[serde(
        rename = "id_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id_type: String,
    /// 该字段标识发送者的类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "app"
    #[serde(
        rename = "sender_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub sender_type: String,
    /// tenant key
    ///
    /// **示例值**: "736588c9260f175e"
    #[serde(
        rename = "tenant_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tenant_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MessageBodySubResp {
    /// 消息内容，JSON字符串格式
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "{\"text\":\"@_user_1 test content\"}"
    #[serde(
        rename = "content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MentionSubResp {
    /// mention key
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "@_user_1"
    #[serde(
        rename = "key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key: String,
    /// 用户或机器人的 open_id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_155184d1e73cbfb8973e5a9e698e74f2"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 被@的用户或机器人 id 类型，目前仅支持 `open_id` ([什么是 Open ID？](https://open.feishu.cn/document/home/user-identity-introduction/open-id))
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "open_id"
    #[serde(
        rename = "id_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id_type: String,
    /// 被at用户的姓名
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Tom"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// tenant key
    ///
    /// **示例值**: "736588c9260f175e"
    #[serde(
        rename = "tenant_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tenant_key: String,
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
        Fn(UpdateMessageEditReq) -> Result<(UpdateMessageEditResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(UpdateMessageEditReq) -> Result<(UpdateMessageEditResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MessageServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_message_edit<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdateMessageEditReq, UpdateMessageEditResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_message_edit(
            &self,
            req: &UpdateMessageEditReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateMessageEditReq, UpdateMessageEditResp, Arc<dyn MockFunc>>(
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
        api::gen::message::update_message_edit::{
            UpdateMessageEditReq, UpdateMessageEditResp, UpdateMessageEditRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .message()
            .mock()
            .mock_update_message_edit(|_| {
                Ok((UpdateMessageEditResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .message()
            .update_message_edit(UpdateMessageEditReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .message()
            .update_message_edit(UpdateMessageEditReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "msg_type": "text",
    "content": "{\"text\":\"test content\"}"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateMessageEditReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "message_id": "om_dc13264520392913993dd051dba21dcf",
        "root_id": "om_40eb06e7b84dc71c03e009ad3c754195",
        "parent_id": "om_d4be107c616aed9c1da8ed8068570a9f",
        "thread_id": "omt_d4be107c616a",
        "msg_type": "card",
        "create_time": "1609296809",
        "update_time": "1609336806",
        "deleted": false,
        "updated": true,
        "chat_id": "oc_5ad11d72b830411d72b836c20",
        "sender": {
            "id": "cli_9f427eec54ae901b",
            "id_type": "app_id",
            "sender_type": "app",
            "tenant_key": "736588c9260f175e"
        },
        "body": {
            "content": "{\"text\":\"@_user_1 test content\"}"
        },
        "mentions": [
            {
                "key": "@_user_1",
                "id": "ou_155184d1e73cbfb8973e5a9e698e74f2",
                "id_type": "open_id",
                "name": "Tom",
                "tenant_key": "736588c9260f175e"
            }
        ],
        "upper_message_id": "om_40eb06e7b84dc71c03e00ida3c754892"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateMessageEditRespInner>(RESP);
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