//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/notification/patch>
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
    /// **api 版本: 2023-08-15T07:34:23+00:00**
    ///
    /// ## 更新推送
    ///
    /// 更新推送信息，只有在草稿状态下才可以调用此接口进行更新。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/notification/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/helpdesk-v1/notification/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhelpdesk-v1%2Fnotification%2Fpatch>
    pub async fn update_helpdesk_notification(
        &self,
        req: UpdateHelpdeskNotificationReq,
    ) -> Result<(UpdateHelpdeskNotificationResp, CommonResponse), Error> {
        self.update_helpdesk_notification_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_helpdesk_notification](#method.update_helpdesk_notification) 函数
    pub async fn update_helpdesk_notification_with_opt(
        &self,
        req: UpdateHelpdeskNotificationReq,
        method_option: MethodOption,
    ) -> Result<(UpdateHelpdeskNotificationResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_helpdesk_notification(&req) {
                tracing::info!("[lark] Helpdesk#UpdateHelpdeskNotification **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Helpdesk#UpdateHelpdeskNotification call api");

        let req = ApiRequest {
            scope: "Helpdesk",
            api: "UpdateHelpdeskNotification",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/helpdesk/v1/notifications/:notification_id",
            param_data: req.gen_param(),
            method_option,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateHelpdeskNotificationRespInner, _) =
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
pub struct UpdateHelpdeskNotificationReq {
    /// push任务唯一id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6985032626234982420"
    #[api(kind = "path", name = "notification_id")]
    pub notification_id: String,
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
    /// 非必填，创建成功后返回
    ///
    /// **示例值**: "6981801914270744596"
    #[api(kind = "body", name = "id")]
    pub id: Option<String>,
    /// 必填，任务名称
    ///
    /// **示例值**: "测试推送任务"
    #[api(kind = "body", name = "job_name")]
    pub job_name: Option<String>,
    /// 非必填，创建成功后返回
    ///
    /// **示例值**: "0"
    #[api(kind = "body", name = "status")]
    pub status: Option<i64>,
    /// 非必填，创建人
    ///
    /// **示例值**: "{"avatar_url":"","name":"","user_id":"ou_7277fd1262bfafc363d5b2a1f9c2ac90"}"
    #[api(kind = "body", name = "create_user")]
    pub create_user: Option<NotificationUserSubReq>,
    /// 非必填，创建时间（毫秒时间戳）
    ///
    /// **示例值**: "1626332244719"
    #[api(kind = "body", name = "created_at")]
    pub created_at: Option<String>,
    /// 非必填，更新用户
    ///
    /// **示例值**: "{"avatar_url":"","name":"","user_id":"ou_7277fd1262bfafc363d5b2a1f9c2ac90"}"
    #[api(kind = "body", name = "update_user")]
    pub update_user: Option<NotificationUserSubReq>,
    /// 非必填，更新时间（毫秒时间戳）
    ///
    /// **示例值**: "1626332244719"
    #[api(kind = "body", name = "updated_at")]
    pub updated_at: Option<String>,
    /// 非必填，目标推送用户总数
    ///
    /// **示例值**: "1"
    #[api(kind = "body", name = "target_user_count")]
    pub target_user_count: Option<i64>,
    /// 非必填，已推送用户总数
    ///
    /// **示例值**: "1"
    #[api(kind = "body", name = "sent_user_count")]
    pub sent_user_count: Option<i64>,
    /// 非必填，已读用户总数
    ///
    /// **示例值**: "1"
    #[api(kind = "body", name = "read_user_count")]
    pub read_user_count: Option<i64>,
    /// 非必填，推送任务触发时间（毫秒时间戳）
    ///
    /// **示例值**: "1626332244719"
    #[api(kind = "body", name = "send_at")]
    pub send_at: Option<String>,
    /// 必填，推送内容，详见：https://open.feishu.cn/tool/cardbuilder?from=howtoguide
    ///
    /// **示例值**: "{   \"config\": {     \"wide_screen_mode\": true   },   \"elements\": [     {       \"tag\": \"div\",       \"text\": {         \"tag\": \"lark_md\",         \"content\": \"[飞书](https://www.feishu.cn)整合即时沟通、日历、音视频会议、云文档、云盘、工作台等功能于一体，成就组织和个人，更高效、更愉悦。\"       }     }   ] }"
    #[api(kind = "body", name = "push_content")]
    pub push_content: Option<String>,
    /// 必填，
    ///
    /// 0（定时推送：push_scope不能等于3） 1（新人入职推送：push_scope必须等于1或者3；new_staff_scope_type不能为空）
    ///
    /// **示例值**: "0"
    #[api(kind = "body", name = "push_type")]
    pub push_type: Option<i64>,
    /// 必填，
    ///
    /// 推送范围（服务台私信） 0：组织内全部成员（user_list和department_list必须为空） 1：不推送任何成员（user_list和department_list必须为空，chat_list不可为空） 2：推送到部分成员（user_list或department_list不能为空） 3：入职新人 以上四种状态，chat_list都相对独立，只有在推送范围为1时，必须需要设置chat_list
    ///
    /// **示例值**: "0"
    #[api(kind = "body", name = "push_scope_type")]
    pub push_scope_type: Option<i64>,
    /// 非必填，
    ///
    /// 新人入职范围类型（push_type为1时生效） 0：组织内所有新人 1：组织内特定的部门（new_staff_scope_department_list 字段不能为空）
    ///
    /// **示例值**: "0"
    #[api(kind = "body", name = "new_staff_scope_type")]
    pub new_staff_scope_type: Option<i64>,
    /// 非必填，新人入职生效部门列表
    ///
    /// **示例值**: "[{"department_id":"od_7c1a2815c9846b5e518b950de0e62de8"}]"
    #[api(kind = "body", name = "new_staff_scope_department_list")]
    pub new_staff_scope_department_list: Vec<Option<NotificationDepartmentSubReq>>,
    /// 非必填，push推送到成员列表
    ///
    /// **示例值**: "[{"user_id":"ou_7277fd1262bfafc363d5b2a1f9c2ac90"}]"
    #[api(kind = "body", name = "user_list")]
    pub user_list: Vec<Option<NotificationUserSubReq>>,
    /// 非必填，push推送到的部门信息列表
    ///
    /// **示例值**: "[{"department_id":"od_7c1a2815c9846b5e518b950de0e62de8"}]"
    #[api(kind = "body", name = "department_list")]
    pub department_list: Vec<Option<NotificationDepartmentSubReq>>,
    /// 非必填，push推送到的会话列表(群)
    ///
    /// **示例值**: "[{"chat_id":"oc_7c1a2815c9846b5e518b950de0e62de8"}]"
    #[api(kind = "body", name = "chat_list")]
    pub chat_list: Vec<Option<NotificationChatSubReq>>,
    /// 非必填，预留扩展字段
    ///
    /// **示例值**: "{}"
    #[api(kind = "body", name = "ext")]
    pub ext: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct NotificationUserSubReq {
    /// 非必填，用户id
    ///
    /// **示例值**: "ou_7277fd1262bfafc363d5b2a1f9c2ac90"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: Option<String>,
    /// 非必填，头像地址
    ///
    /// **示例值**: "http://*.com/*.png"
    #[serde(
        rename = "avatar_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar_url: Option<String>,
    /// 非必填，用户名称
    ///
    /// **示例值**: "test"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct NotificationDepartmentSubReq {
    /// 部门ID
    ///
    /// **示例值**: "od_7277fd1262bfafc363d5b2a1f9c2ac90"
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: Option<String>,
    /// 非必填，部门名称
    ///
    /// **示例值**: "测试部门"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct NotificationChatSubReq {
    /// 非必填，会话ID
    ///
    /// **示例值**: "oc_7277fd1262bfafc363d5b2a1f9c2ac90"
    #[serde(
        rename = "chat_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_id: Option<String>,
    /// 非必填，会话名称
    ///
    /// **示例值**: "测试群聊"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateHelpdeskNotificationRespInner {
    #[serde(flatten)]
    data: Option<UpdateHelpdeskNotificationResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateHelpdeskNotificationResp {
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
            UpdateHelpdeskNotificationReq,
        ) -> Result<(UpdateHelpdeskNotificationResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateHelpdeskNotificationReq,
                )
                    -> Result<(UpdateHelpdeskNotificationResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HelpdeskServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_helpdesk_notification<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateHelpdeskNotificationReq,
            UpdateHelpdeskNotificationResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_helpdesk_notification(
            &self,
            req: &UpdateHelpdeskNotificationReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateHelpdeskNotificationReq,
                UpdateHelpdeskNotificationResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::helpdesk::update_helpdesk_notification::{
            UpdateHelpdeskNotificationReq, UpdateHelpdeskNotificationResp,
            UpdateHelpdeskNotificationRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .helpdesk()
            .mock()
            .mock_update_helpdesk_notification(|_| {
                Ok((
                    UpdateHelpdeskNotificationResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .helpdesk()
            .update_helpdesk_notification(UpdateHelpdeskNotificationReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .helpdesk()
            .update_helpdesk_notification(UpdateHelpdeskNotificationReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "id": "6981801914270744596",
    "job_name": "测试推送任务",
    "status": 0,
    "create_user": {
        "user_id": "ou_7277fd1262bfafc363d5b2a1f9c2ac90",
        "avatar_url": "http://*.com/*.png",
        "name": "test"
    },
    "created_at": "1626332244719",
    "update_user": {
        "user_id": "ou_7277fd1262bfafc363d5b2a1f9c2ac90",
        "avatar_url": "http://*.com/*.png",
        "name": "test"
    },
    "updated_at": "1626332244719",
    "target_user_count": 1,
    "sent_user_count": 1,
    "read_user_count": 1,
    "send_at": "1626332244719",
    "push_content": "{   \"config\": {     \"wide_screen_mode\": true   },   \"elements\": [     {       \"tag\": \"div\",       \"text\": {         \"tag\": \"lark_md\",         \"content\": \"[飞书](https://www.feishu.cn)整合即时沟通、日历、音视频会议、云文档、云盘、工作台等功能于一体，成就组织和个人，更高效、更愉悦。\"       }     }   ] }",
    "push_type": 0,
    "push_scope_type": 0,
    "new_staff_scope_type": 0,
    "new_staff_scope_department_list": [
        {
            "department_id": "od_7277fd1262bfafc363d5b2a1f9c2ac90",
            "name": "测试部门"
        }
    ],
    "user_list": [
        {
            "user_id": "ou_7277fd1262bfafc363d5b2a1f9c2ac90",
            "avatar_url": "http://*.com/*.png",
            "name": "test"
        }
    ],
    "department_list": [
        {
            "department_id": "od_7277fd1262bfafc363d5b2a1f9c2ac90",
            "name": "测试部门"
        }
    ],
    "chat_list": [
        {
            "chat_id": "oc_7277fd1262bfafc363d5b2a1f9c2ac90",
            "name": "测试群聊"
        }
    ],
    "ext": "{}"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateHelpdeskNotificationReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateHelpdeskNotificationRespInner>(RESP);
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