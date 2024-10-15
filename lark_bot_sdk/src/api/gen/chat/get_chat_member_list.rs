//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/get>
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
    /// **api 版本: 2024-06-21T09:04:28+00:00**
    ///
    /// ## 获取群成员列表
    ///
    /// 获取用户/机器人所在群的群成员列表。
    ///
    /// 注意事项：
    ///
    /// - 应用需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
    ///
    /// - 机器人或授权用户必须在群组中
    ///
    /// - 该接口不会返回群内的机器人成员
    ///
    /// - 由于返回的群成员列表会过滤掉机器人成员，因此返回的群成员个数可能会小于指定的page_size
    ///
    /// - 如果有同一时间加入群的群成员，会一次性返回，这会导致返回的群成员个数可能会大于指定的page_size
    ///
    /// - 获取内部群信息时，操作者须与群组在同一租户下
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/group/chat-member/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fgroup%2Fchat-member%2Fget>
    pub async fn get_chat_member_list(
        &self,
        req: GetChatMemberListReq,
    ) -> Result<(GetChatMemberListResp, CommonResponse), Error> {
        self.get_chat_member_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_chat_member_list](#method.get_chat_member_list) 函数
    pub async fn get_chat_member_list_with_opt(
        &self,
        req: GetChatMemberListReq,
        method_option: MethodOption,
    ) -> Result<(GetChatMemberListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_chat_member_list(&req) {
                tracing::info!("[lark] Chat#GetChatMemberList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Chat#GetChatMemberList call api");

        let req = ApiRequest {
            scope: "Chat",
            api: "GetChatMemberList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/chats/:chat_id/members",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetChatMemberListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetChatMemberListReq {
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
    /// `open_id`: 以 open_id 来识别成员
    ///
    /// **当值为 `user_id`，字段权限要求**：
    ///
    /// (**仅自建应用**) 获取用户 user ID
    ///
    /// `union_id`: 以 union_id 来识别成员
    ///
    /// `user_id`: 以 user_id 来识别成员
    #[api(
        kind = "query",
        name = "member_id_type",
        v_type = "var",
        option = "false"
    )]
    pub member_id_type: String,
    /// 分页大小
    ///
    /// **示例值**: "20
    ///
    /// **默认值**：`20`
    ///
    /// **数据校验规则**：
    ///
    /// - 最大值：`100`"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "WWxHTStrOEs5WHZpNktGbU94bUcvMWlxdDUzTWt1OXNrRmlLaGRNVG0yaz0="
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetChatMemberListRespInner {
    #[serde(flatten)]
    data: Option<GetChatMemberListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetChatMemberListResp {
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
    /// 成员列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<ListMemberSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 成员总数
    ///
    /// **示例值**: "2"
    #[serde(
        rename = "member_total",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub member_total: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ListMemberSubResp {
    /// 成员的用户 ID 类型，与查询参数中的 member_id_type 相同。取值为：`open_id`、`user_id`、`union_id`其中之一。
    ///
    /// **示例值**: "user_id"
    #[serde(
        rename = "member_id_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub member_id_type: String,
    /// 成员的用户ID，ID值与查询参数中的 member_id_type 对应。
    ///
    /// 不同 ID 的说明参见 [用户相关的 ID 概念](https://open.feishu.cn/document/home/user-identity-introduction/introduction)
    ///
    /// **示例值**: "4d7a3c6g"
    #[serde(
        rename = "member_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub member_id: String,
    /// 名字
    ///
    /// **示例值**: "张三"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 租户Key，为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用中的唯一标识
    ///
    /// **示例值**: "736588c9260f175d"
    #[serde(
        rename = "tenant_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tenant_key: String,
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
        Fn(GetChatMemberListReq) -> Result<(GetChatMemberListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetChatMemberListReq) -> Result<(GetChatMemberListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ChatServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_chat_member_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetChatMemberListReq, GetChatMemberListResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_chat_member_list(
            &self,
            req: &GetChatMemberListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetChatMemberListReq, GetChatMemberListResp, Arc<dyn MockFunc>>(
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
        api::gen::chat::get_chat_member_list::{
            GetChatMemberListReq, GetChatMemberListResp, GetChatMemberListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .chat()
            .mock()
            .mock_get_chat_member_list(|_| {
                Ok((GetChatMemberListResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .chat()
            .get_chat_member_list(GetChatMemberListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .chat()
            .get_chat_member_list(GetChatMemberListReq::default())
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
        "items": [
            {
                "member_id_type": "open_id",
                "member_id": "ou_9204a37300b3700d61effaa439f34295",
                "name": "张三",
                "tenant_key": "736588c9260f175d"
            }
        ],
        "page_token": "dmJCRHhpd3JRbGV1VEVNRFFyTitRWDY5ZFkybmYrMEUwMUFYT0VMMWdENEtuYUhsNUxGMDIwemtvdE5ORjBNQQ==",
        "has_more": true,
        "member_total": 2
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetChatMemberListRespInner>(RESP);
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
