//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/get>
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
    /// ## 获取群信息
    ///
    /// 获取群名称、群描述、群头像、群主 ID 等群基本信息。
    ///
    /// 注意事项：
    ///
    /// - 应用需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
    ///
    /// - 机器人或授权用户必须在群里（否则只会返回群名称、群头像等基本信息）
    ///
    /// - 获取内部群信息时，操作者须与群组在同一租户下
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/group/chat/get-2>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fgroup%2Fchat%2Fget-2>
    pub async fn get_chat(&self, req: GetChatReq) -> Result<(GetChatResp, CommonResponse), Error> {
        self.get_chat_with_opt(req, Default::default()).await
    }

    /// 参见 [get_chat](#method.get_chat) 函数
    pub async fn get_chat_with_opt(
        &self,
        req: GetChatReq,
        method_option: MethodOption,
    ) -> Result<(GetChatResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_chat(&req) {
                tracing::info!("[lark] Chat#GetChat **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Chat#GetChat call api");

        let req = ApiRequest {
            scope: "Chat",
            api: "GetChat",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/chats/:chat_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetChatRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetChatReq {
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
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetChatRespInner {
    #[serde(flatten)]
    data: Option<GetChatResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetChatResp {
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
    /// 群头像 URL
    ///
    /// **示例值**: "https://p3-lark-file.byteimg.com/img/lark-avatar-staging/default-avatar_44ae0ca3-e140-494b-956f-78091e348435~100x100.jpg"
    #[serde(
        rename = "avatar",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar: String,
    /// 群名称
    ///
    /// **示例值**: "测试群名称"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 群描述
    ///
    /// **示例值**: "测试群描述"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 群国际化名称
    #[serde(
        rename = "i18n_names",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_names: I18nNamesSubResp,
    /// 群成员添加权限
    ///
    /// **可选值有**：
    ///
    /// - `only_owner`：仅群主和管理员
    ///
    /// - `all_members`：所有成员
    ///
    /// **注意**：单聊不返回该字段
    ///
    /// **示例值**: "all_members"
    #[serde(
        rename = "add_member_permission",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub add_member_permission: String,
    /// 群分享权限
    ///
    /// **可选值有**：
    ///
    /// - `allowed`：允许
    ///
    /// - `not_allowed`：不允许
    ///
    /// **注意**：单聊不返回该字段
    ///
    /// **示例值**: "allowed"
    #[serde(
        rename = "share_card_permission",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub share_card_permission: String,
    /// at 所有人权限
    ///
    /// **可选值有**：
    ///
    /// - `only_owner`：仅群主和管理员
    ///
    /// - `all_members`：所有成员
    ///
    /// **注意**：单聊不返回该字段
    ///
    /// **示例值**: "all_members"
    #[serde(
        rename = "at_all_permission",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub at_all_permission: String,
    /// 群编辑权限
    ///
    /// **可选值有**：
    ///
    /// - `only_owner`：仅群主和管理员
    ///
    /// - `all_members`：所有成员
    ///
    /// **示例值**: "all_members"
    #[serde(
        rename = "edit_permission",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub edit_permission: String,
    /// 群主 ID 对应的ID类型，与查询参数中的 ==user_id_type== 相同。取值为：`open_id`、`user_id`、`union_id`其中之一
    ///
    /// **注意**：
    ///
    /// - 当群主是机器人时不返回该字段
    ///
    /// - 单聊不返回该字段
    ///
    /// **示例值**: "user_id"
    #[serde(
        rename = "owner_id_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub owner_id_type: String,
    /// 群主 ID，ID值与查询参数中的 ==user_id_type== 对应；不同 ID 的说明参见 [用户相关的 ID 概念](https://open.feishu.cn/document/home/user-identity-introduction/introduction)
    ///
    /// **注意**：
    ///
    /// - 当群主是机器人时不返回该字段
    ///
    /// - 单聊不返回该字段
    ///
    /// **示例值**: "4d7a3c6g"
    #[serde(
        rename = "owner_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub owner_id: String,
    /// 用户管理员列表
    #[serde(
        rename = "user_manager_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_manager_id_list: Vec<String>,
    /// 机器人管理员列表
    #[serde(
        rename = "bot_manager_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub bot_manager_id_list: Vec<String>,
    /// 群消息模式
    ///
    /// **可选值有**：
    ///
    /// - `chat`：会话消息
    ///
    /// - ` thread`：话题消息
    ///
    /// **注意**：仅对话群返回该字段
    ///
    /// **示例值**: "chat"
    #[serde(
        rename = "group_message_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_message_type: String,
    /// 群模式
    ///
    /// **可选值有**：
    ///
    /// - `group`：群组
    ///
    /// - `topic`: 话题
    ///
    /// - `p2p`: 单聊
    ///
    /// **示例值**: "group"
    #[serde(
        rename = "chat_mode",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_mode: String,
    /// 群类型
    ///
    /// **可选值有**：
    ///
    /// - `private`：私有群
    ///
    /// - `public`：公开群
    ///
    /// **注意**：单聊不返回该字段
    ///
    /// **示例值**: "private"
    #[serde(
        rename = "chat_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_type: String,
    /// 群标签，如有多个，则按照下列顺序返回第一个
    ///
    /// **可选值有**：
    ///
    /// - `inner`：内部群
    ///
    /// - `tenant`：公司群
    ///
    /// - `department`：部门群
    ///
    /// - `edu`：教育群
    ///
    /// - `meeting`：会议群
    ///
    /// - `customer_service`：客服群
    ///
    /// **注意**：单聊不返回该字段
    ///
    /// **示例值**: "inner"
    #[serde(
        rename = "chat_tag",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_tag: String,
    /// 入群消息可见性
    ///
    /// **可选值有**：
    ///
    /// - `only_owner`：仅群主和管理员可见
    ///
    /// - `all_members`：所有成员可见
    ///
    /// - `not_anyone`：任何人均不可见
    ///
    /// **注意**：单聊不返回该字段
    ///
    /// **示例值**: "only_owner"
    #[serde(
        rename = "join_message_visibility",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub join_message_visibility: String,
    /// 出群消息可见性
    ///
    /// **可选值有**：
    ///
    /// - `only_owner`：仅群主和管理员可见
    ///
    /// - `all_members`：所有成员可见
    ///
    /// - `not_anyone`：任何人均不可见
    ///
    /// **注意**：单聊不返回该字段
    ///
    /// **示例值**: "only_owner"
    #[serde(
        rename = "leave_message_visibility",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub leave_message_visibility: String,
    /// 加群审批
    ///
    /// **可选值有**：
    ///
    /// - `no_approval_required`：无需审批
    ///
    /// - `approval_required`：需要审批
    ///
    /// **注意**：单聊不返回该字段
    ///
    /// **示例值**: "no_approval_required"
    #[serde(
        rename = "membership_approval",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub membership_approval: String,
    /// 发言权限
    ///
    /// **可选值有**：
    ///
    /// - `only_owner`：仅群主和管理员
    ///
    /// - `all_members`：所有成员
    ///
    /// - `moderator_list`：指定群成员
    ///
    /// **示例值**: "all_members"
    #[serde(
        rename = "moderation_permission",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub moderation_permission: String,
    /// 是否是外部群
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "external",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub external: bool,
    /// 租户Key，为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用中的唯一标识
    ///
    /// **示例值**: "736588c9260f175e"
    #[serde(
        rename = "tenant_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tenant_key: String,
    /// 群成员人数
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "user_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_count: String,
    /// 群机器人数
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "bot_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub bot_count: String,
    /// 保密模式设置
    #[serde(
        rename = "restricted_mode_setting",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub restricted_mode_setting: RestrictedModeSettingSubResp,
    /// 谁可以加急
    ///
    /// **示例值**: "all_members"
    ///
    /// **可选值**:
    ///
    /// `only_owner`: 仅群主和管理员
    ///
    /// `all_members`: 所有成员
    #[serde(
        rename = "urgent_setting",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub urgent_setting: String,
    /// 谁可以发起视频会议
    ///
    /// **示例值**: "all_members"
    ///
    /// **可选值**:
    ///
    /// `only_owner`: 仅群主和管理员
    ///
    /// `all_members`: 所有成员
    #[serde(
        rename = "video_conference_setting",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub video_conference_setting: String,
    /// 隐藏群成员人数设置
    ///
    /// **示例值**: "all_members"
    ///
    /// **可选值**:
    ///
    /// `all_members`: 所有群成员可见
    ///
    /// `only_owner`: 仅群主群管理员可见
    #[serde(
        rename = "hide_member_count_setting",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hide_member_count_setting: String,
    /// 群状态
    ///
    /// **示例值**: "normal"
    ///
    /// **可选值**:
    ///
    /// `normal`: 正常
    ///
    /// `dissolved`: 解散
    ///
    /// `dissolved_save`: 解散并保留
    #[serde(
        rename = "chat_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nNamesSubResp {
    /// 中文名
    ///
    /// **示例值**: "群聊"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 英文名
    ///
    /// **示例值**: "group chat"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
    /// 日文名
    ///
    /// **示例值**: "グループチャット"
    #[serde(
        rename = "ja_jp",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ja_jp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RestrictedModeSettingSubResp {
    /// 保密模式是否开启
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: bool,
    /// 允许截屏录屏
    ///
    /// **示例值**: "all_members"
    ///
    /// **可选值**:
    ///
    /// `AllMembers`: 所有成员允许截屏录屏
    ///
    /// `NotAnyone`: 所有成员禁止截屏录屏
    #[serde(
        rename = "screenshot_has_permission_setting",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub screenshot_has_permission_setting: String,
    /// 允许下载消息中图片、视频和文件
    ///
    /// **示例值**: "all_members"
    ///
    /// **可选值**:
    ///
    /// `AllMembers`: 所有成员允许下载资源
    ///
    /// `NotAnyone`: 所有成员禁止下载资源
    #[serde(
        rename = "download_has_permission_setting",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub download_has_permission_setting: String,
    /// 允许复制和转发消息
    ///
    /// **示例值**: "all_members"
    ///
    /// **可选值**:
    ///
    /// `AllMembers`: 所有成员允许复制和转发消息
    ///
    /// `NotAnyone`: 所有成员禁止复制和转发消息
    #[serde(
        rename = "message_has_permission_setting",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub message_has_permission_setting: String,
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
        Fn(GetChatReq) -> Result<(GetChatResp, CommonResponse), Error> + Send + Sync + 'static
    {
    }
    impl<
            T: Fn(GetChatReq) -> Result<(GetChatResp, CommonResponse), Error> + Send + Sync + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ChatServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_chat<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetChatReq, GetChatResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_chat(&self, req: &GetChatReq) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetChatReq, GetChatResp, Arc<dyn MockFunc>>(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::chat::get_chat::{GetChatReq, GetChatResp, GetChatRespInner},
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .chat()
            .mock()
            .mock_get_chat(|_| Ok((GetChatResp::default(), CommonResponse::default())))
            .build();
        let res = lark.chat().get_chat(GetChatReq::default()).await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark.chat().get_chat(GetChatReq::default()).await;
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
        "avatar": "https://p3-lark-file.byteimg.com/img/lark-avatar-staging/default-avatar_44ae0ca3-e140-494b-956f-78091e348435~100x100.jpg",
        "name": "测试群名称",
        "description": "测试群描述",
        "i18n_names": {
            "zh_cn": "群聊",
            "en_us": "group chat",
            "ja_jp": "グループチャット"
        },
        "add_member_permission": "all_members",
        "share_card_permission": "allowed",
        "at_all_permission": "all_members",
        "edit_permission": "all_members",
        "owner_id_type": "user_id",
        "owner_id": "4d7a3c6g",
        "user_manager_id_list": [
            "ou_9204a37300b3700d61effaa439f34295"
        ],
        "bot_manager_id_list": [
            "cli_a3e157960e7294c"
        ],
        "group_message_type": "chat",
        "chat_mode": "group",
        "chat_type": "private",
        "chat_tag": "inner",
        "join_message_visibility": "only_owner",
        "leave_message_visibility": "only_owner",
        "membership_approval": "no_approval_required",
        "moderation_permission": "all_members",
        "external": false,
        "tenant_key": "736588c9260f175e",
        "user_count": "1",
        "bot_count": "3",
        "restricted_mode_setting": {
            "status": false,
            "screenshot_has_permission_setting": "all_members",
            "download_has_permission_setting": "all_members",
            "message_has_permission_setting": "all_members"
        },
        "urgent_setting": "all_members",
        "video_conference_setting": "all_members",
        "hide_member_count_setting": "all_members",
        "chat_status": "normal"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetChatRespInner>(RESP);
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
