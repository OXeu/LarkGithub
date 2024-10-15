//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-acl/create>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::calendar::CalendarService;

impl<'c, IStore: Store, IClient: HttpClient> CalendarService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-16T06:22:17+00:00**
    ///
    /// ## 创建访问控制
    ///
    /// 调用该接口以当前身份（应用或用户）为指定日历添加访问控制，即日历成员权限。
    ///
    /// - 当前身份由 Header Authorization 的 Token 类型决定。tenant_access_token 指应用身份，user_access_token 指用户身份。
    ///
    /// - 如果使用应用身份调用该接口，则需要确保应用开启了[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)。
    ///
    /// - 当前身份需要有日历的 owner 权限，并且日历的类型只能为 primary 或 shared。你可以调用[查询日历信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/get)接口，获取日历类型以及当前身份对该日历的访问权限。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-acl/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/calendar-v4/calendar-acl/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcalendar-v4%2Fcalendar-acl%2Fcreate>
    pub async fn create_calendar_acl(
        &self,
        req: CreateCalendarAclReq,
    ) -> Result<(CreateCalendarAclResp, CommonResponse), Error> {
        self.create_calendar_acl_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_calendar_acl](#method.create_calendar_acl) 函数
    pub async fn create_calendar_acl_with_opt(
        &self,
        req: CreateCalendarAclReq,
        method_option: MethodOption,
    ) -> Result<(CreateCalendarAclResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_calendar_acl(&req) {
                tracing::info!("[lark] Calendar#CreateCalendarAcl **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Calendar#CreateCalendarAcl call api");

        let req = ApiRequest {
            scope: "Calendar",
            api: "CreateCalendarAcl",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/calendar/v4/calendars/:calendar_id/acls",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateCalendarAclRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateCalendarAclReq {
    /// 需要添加访问控制的日历 ID。
    ///
    /// 创建共享日历时会返回日历 ID。你也可以调用以下接口获取某一日历的 ID。
    ///
    /// - [查询主日历信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/primary)
    ///
    /// - [查询日历列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/list)
    ///
    /// - [搜索日历](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/search)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "feishu.cn_xxxxxxxxxx@group.calendar.feishu.cn"
    #[api(kind = "path", name = "calendar_id")]
    pub calendar_id: String,
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
    /// 对日历的访问权限。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "writer"
    ///
    /// **可选值**:
    ///
    /// `Unkonwn`: 未知权限。unknown 是 role 参数枚举值之一，但 role 作为请求参数时，不支持传入 unknown。
    ///
    /// `FreeBusyReader`: 游客，只能看到忙碌、空闲信息。
    ///
    /// `Reader`: 订阅者，可查看所有日程详情。
    ///
    /// `Writer`: 编辑者，可创建及修改日程。
    ///
    /// `Owner`: 管理员，可管理日历及共享设置。
    #[api(kind = "body", name = "role")]
    pub role: String,
    /// 权限的生效范围。
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "scope")]
    pub scope: AclScopeSubReq,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AclScopeSubReq {
    /// 权限生效范围的类型。
    ///
    /// **注意**：目前只支持 `user`，且当 `type=user` 时，user_id 需要传入和 user_id_type 一致的用户 ID 类型。例如，`user_id_type=open_id` 时，user_id 需要传入用户的 open_id。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "user"
    ///
    /// **可选值**:
    ///
    /// `User`: 用户
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 用户 ID。当 `type=user` 时，必须设置该参数值。关于用户 ID 的更多介绍可参见[用户相关的 ID 概念](https://open.feishu.cn/document/home/user-identity-introduction/introduction)。
    ///
    /// **示例值**: "ou_xxxxxx"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateCalendarAclRespInner {
    #[serde(flatten)]
    data: Option<CreateCalendarAclResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateCalendarAclResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: CalendarAclSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CalendarAclSubResp {
    /// 访问控制 ID。该 ID 在单个日历实体内唯一，不同日历实体可能存在重复的访问控制 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "user_xxxxxx"
    #[serde(
        rename = "acl_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub acl_id: String,
    /// 对日历的访问权限。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "writer"
    ///
    /// **可选值**:
    ///
    /// `Unkonwn`: 未知权限。
    ///
    /// `FreeBusyReader`: 游客，只能看到忙碌、空闲信息。
    ///
    /// `Reader`: 订阅者，可查看所有日程详情。
    ///
    /// `Writer`: 编辑者，可创建及修改日程。
    ///
    /// `Owner`: 管理员，可管理日历及共享设置。
    #[serde(
        rename = "role",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub role: String,
    /// 权限生效范围。
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "scope",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub scope: AclScopeSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AclScopeSubResp {
    /// 权限生效范围的类型。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "user"
    ///
    /// **可选值**:
    ///
    /// `User`: 用户
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 用户 ID，更多介绍可参见[用户相关的 ID 概念](https://open.feishu.cn/document/home/user-identity-introduction/introduction)。
    ///
    /// **示例值**: "ou_xxxxxx"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::calendar::CalendarServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(CreateCalendarAclReq) -> Result<(CreateCalendarAclResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(CreateCalendarAclReq) -> Result<(CreateCalendarAclResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CalendarServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_calendar_acl<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CreateCalendarAclReq, CreateCalendarAclResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_calendar_acl(
            &self,
            req: &CreateCalendarAclReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateCalendarAclReq, CreateCalendarAclResp, Arc<dyn MockFunc>>(
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
        api::gen::calendar::create_calendar_acl::{
            CreateCalendarAclReq, CreateCalendarAclResp, CreateCalendarAclRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .calendar()
            .mock()
            .mock_create_calendar_acl(|_| {
                Ok((CreateCalendarAclResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .calendar()
            .create_calendar_acl(CreateCalendarAclReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .calendar()
            .create_calendar_acl(CreateCalendarAclReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "role": "writer",
    "scope": {
        "type": "user",
        "user_id": "ou_xxxxxx"
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateCalendarAclReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "acl_id": "user_xxxxxx",
        "role": "writer",
        "scope": {
            "type": "user",
            "user_id": "ou_xxxxxx"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateCalendarAclRespInner>(RESP);
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
