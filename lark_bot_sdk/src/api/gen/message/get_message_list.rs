//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list>
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
    /// **api 版本: 2024-06-21T09:04:23+00:00**
    ///
    /// ## 获取会话历史消息
    ///
    /// 获取会话（包括单聊、群组）的历史消息（聊天记录）。
    ///
    /// 接口级别权限默认只能获取单聊（p2p）消息，如果需要获取群组（group）消息，应用还必须拥有 **==获取群组中所有消息==** 权限
    ///
    /// - 需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
    ///
    /// - 获取消息时，机器人必须在群组中
    ///
    /// - 对于**普通对话群**中的话题消息，通过 "chat" 容器类型仅能获取到话题的根消息，可通过指定容器类型为 "thread" 获取话题回复中的所有消息
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/im-v1/message/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fim-v1%2Fmessage%2Flist>
    pub async fn get_message_list(
        &self,
        req: GetMessageListReq,
    ) -> Result<(GetMessageListResp, CommonResponse), Error> {
        self.get_message_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_message_list](#method.get_message_list) 函数
    pub async fn get_message_list_with_opt(
        &self,
        req: GetMessageListReq,
        method_option: MethodOption,
    ) -> Result<(GetMessageListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_message_list(&req) {
                tracing::info!("[lark] Message#GetMessageList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Message#GetMessageList call api");

        let req = ApiRequest {
            scope: "Message",
            api: "GetMessageList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/im/v1/messages",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetMessageListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetMessageListReq {
    /// 容器类型
    ///
    /// **可选值有**：
    ///
    /// - `chat`：包含单聊（p2p）和群聊（group）
    ///
    /// - `thread`: 话题
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "chat"
    #[api(
        kind = "query",
        name = "container_id_type",
        v_type = "var",
        option = "false"
    )]
    pub container_id_type: String,
    /// 容器的ID，可填写：
    ///
    /// - chat_id，群聊或单聊的 ID，参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// - thread_id，话题 ID，参见[话题介绍](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/thread-introduction)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "oc_234jsi43d3ssi993d43545f"
    #[api(
        kind = "query",
        name = "container_id",
        v_type = "var",
        option = "false"
    )]
    pub container_id: String,
    /// 历史信息的起始时间（秒级时间戳）
    ///
    /// **注意：**
    ///
    /// - thread 容器类型暂不支持获取指定时间范围内的消息
    ///
    /// **示例值**: "1608594809"
    #[api(kind = "query", name = "start_time", v_type = "var", option = "false")]
    pub start_time: String,
    /// 历史信息的结束时间（秒级时间戳）
    ///
    /// **注意：**
    ///
    /// - thread 容器类型暂不支持获取指定时间范围内的消息
    ///
    /// **示例值**: "1609296809"
    #[api(kind = "query", name = "end_time", v_type = "var", option = "false")]
    pub end_time: String,
    /// 消息排序方式
    ///
    /// **示例值**: "ByCreateTimeAsc"
    ///
    /// **可选值**:
    ///
    /// `ByCreateTimeAsc`: 按消息创建时间升序排列
    ///
    /// `ByCreateTimeDesc`: 按消息创建时间降序排列
    #[api(kind = "query", name = "sort_type", v_type = "var", option = "false")]
    pub sort_type: String,
    /// 分页大小
    ///
    /// **示例值**: "20"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "GxmvlNRvP0NdQZpa7yIqf_Lv_QuBwTQ8tXkX7w-irAghVD_TvuYd1aoJ1LQph86O-XImC4X9j9FhUPhXQDvtrQ=="
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetMessageListRespInner {
    #[serde(flatten)]
    data: Option<GetMessageListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetMessageListResp {
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
    /// 是否还有更多项
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "GxmvlNRvP0NdQZpa7yIqf_Lv_QuBwTQ8tXkX7w-irAghVD_TvuYd1aoJ1LQph86O-XImC4X9j9FhUPhXQDvtrQ=="
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// message[]
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<MessageSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MessageSubResp {
    /// 消息id，说明参见：[消息ID说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
    ///
    /// **示例值**: "om_dc13264520392913993dd051dba21dcf"
    #[serde(
        rename = "message_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub message_id: String,
    /// 根消息id，用于回复消息场景，说明参见：[消息ID说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
    ///
    /// **示例值**: "om_40eb06e7b84dc71c03e009ad3c754195"
    #[serde(
        rename = "root_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub root_id: String,
    /// 父消息的id，用于回复消息场景，说明参见：[消息ID说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
    ///
    /// **示例值**: "om_d4be107c616aed9c1da8ed8068570a9f"
    #[serde(
        rename = "parent_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_id: String,
    /// 消息所属的话题 ID
    ///
    /// **示例值**: "omt_d4be107c616a"
    #[serde(
        rename = "thread_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub thread_id: String,
    /// 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、share_user等，类型定义请参考[接收消息内容](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/events/message_content)
    ///
    /// **示例值**: "interactive"
    #[serde(
        rename = "msg_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub msg_type: String,
    /// 消息生成的时间戳（毫秒）
    ///
    /// **示例值**: "1615380573411"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 消息更新的时间戳（毫秒）
    ///
    /// **示例值**: "1615380573411"
    #[serde(
        rename = "update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_time: String,
    /// 消息是否被撤回或删除
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "deleted",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub deleted: bool,
    /// 消息是否被更新
    ///
    /// **示例值**: "false"
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
    /// 消息内容
    ///
    /// **示例值**: "json结构"
    #[serde(
        rename = "body",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body: MessageBodySubResp,
    /// 被@的用户或机器人的id列表
    #[serde(
        rename = "mentions",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mentions: Vec<MentionSubResp>,
    /// 合并转发消息中，上一层级的消息id message_id，说明参见：[消息ID说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
    ///
    /// **示例值**: "om_40eb06e7b84dc71c03e009ad3c754195"
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
    /// **可选值有：**
    ///
    /// - `open_id`
    ///
    /// - `app_id`
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
    /// **可选值有：**
    ///
    /// - `user`: 用户
    ///
    /// - `app`: 应用
    ///
    /// - `anonymous`: 匿名
    ///
    /// - `unknown`: 未知
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "app"
    #[serde(
        rename = "sender_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub sender_type: String,
    /// 为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用里面的唯一标识
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
    /// 消息内容，json结构序列化后的字符串。不同msg_type对应不同内容。消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、share_user等，类型定义请参考：[接收消息内容](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/events/message_content)
    ///
    /// <br>
    ///
    /// **注意**：
    ///
    /// - 卡片消息内容与在卡片搭建工具中获取的卡片 JSON 不一致，暂不支持返回原始卡片 JSON
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "{\"text\":\"test content\"}"
    #[serde(
        rename = "content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MentionSubResp {
    /// 被@的用户或机器人的序号。例如，第3个被@到的成员，值为“@_user_3”
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "@_user_1"
    #[serde(
        rename = "key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key: String,
    /// 被@的用户或者机器人的open_id
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
    /// 被@的用户或机器人的姓名
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Tom"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用里面的唯一标识
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
        Fn(GetMessageListReq) -> Result<(GetMessageListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetMessageListReq) -> Result<(GetMessageListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MessageServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_message_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetMessageListReq, GetMessageListResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_message_list(
            &self,
            req: &GetMessageListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetMessageListReq, GetMessageListResp, Arc<dyn MockFunc>>(
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
        api::gen::message::get_message_list::{
            GetMessageListReq, GetMessageListResp, GetMessageListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .message()
            .mock()
            .mock_get_message_list(|_| {
                Ok((GetMessageListResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .message()
            .get_message_list(GetMessageListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .message()
            .get_message_list(GetMessageListReq::default())
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
    "data": {
        "has_more": false,
        "page_token": "GxmvlNRvP0NdQZpa7yIqf_Lv_QuBwTQ8tXkX7w-irAghVD_TvuYd1aoJ1LQph86O-XImC4X9j9FhUPhXQDvtrQ==",
        "items": [
            {
                "message_id": "om_dc13264520392913993dd051dba21dcf",
                "root_id": "om_40eb06e7b84dc71c03e009ad3c754195",
                "parent_id": "om_d4be107c616aed9c1da8ed8068570a9f",
                "thread_id": "omt_d4be107c616a",
                "msg_type": "interactive",
                "create_time": "1615380573411",
                "update_time": "1615380573411",
                "deleted": false,
                "updated": false,
                "chat_id": "oc_5ad11d72b830411d72b836c20",
                "sender": {
                    "id": "cli_9f427eec54ae901b",
                    "id_type": "app_id",
                    "sender_type": "app",
                    "tenant_key": "736588c9260f175e"
                },
                "body": {
                    "content": "{\"text\":\"test content\"}"
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
                "upper_message_id": "om_40eb06e7b84dc71c03e009ad3c754195"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetMessageListRespInner>(RESP);
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