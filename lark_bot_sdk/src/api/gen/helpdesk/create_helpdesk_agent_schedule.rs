//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_schedule/create>
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
    /// **api 版本: 2024-04-15T06:21:35+00:00**
    ///
    /// ## 创建客服工作日程
    ///
    /// 该接口用于创建客服日程。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_schedule/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/helpdesk-v1/agent-function/agent-schedules/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhelpdesk-v1%2Fagent-function%2Fagent-schedules%2Fcreate>
    pub async fn create_helpdesk_agent_schedule(
        &self,
        req: CreateHelpdeskAgentScheduleReq,
    ) -> Result<(CreateHelpdeskAgentScheduleResp, CommonResponse), Error> {
        self.create_helpdesk_agent_schedule_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_helpdesk_agent_schedule](#method.create_helpdesk_agent_schedule) 函数
    pub async fn create_helpdesk_agent_schedule_with_opt(
        &self,
        req: CreateHelpdeskAgentScheduleReq,
        method_option: MethodOption,
    ) -> Result<(CreateHelpdeskAgentScheduleResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_helpdesk_agent_schedule(&req) {
                tracing::info!("[lark] Helpdesk#CreateHelpdeskAgentSchedule **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Helpdesk#CreateHelpdeskAgentSchedule call api");

        let req = ApiRequest {
            scope: "Helpdesk",
            api: "CreateHelpdeskAgentSchedule",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/helpdesk/v1/agent_schedules",
            param_data: req.gen_param(),
            method_option,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateHelpdeskAgentScheduleRespInner, _) =
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
pub struct CreateHelpdeskAgentScheduleReq {
    /// 新客服日程
    #[api(kind = "body", name = "agent_schedules")]
    pub agent_schedules: Vec<Option<AgentScheduleUpdateInfoSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AgentScheduleUpdateInfoSubReq {
    /// 客服id
    ///
    /// [可以以普通用户身份在服务台发起工单，从工单详情里面获取用户guest.id](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/get)
    ///
    /// **示例值**: "agent-id"
    #[serde(
        rename = "agent_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent_id: Option<String>,
    /// 工作日程列表
    #[serde(
        rename = "schedule",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub schedule: Vec<Option<WeekdayScheduleSubReq>>,
    /// 客服技能 ids
    ///
    /// **示例值**: "[“test-skill-id”]"
    #[serde(
        rename = "agent_skill_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub agent_skill_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct WeekdayScheduleSubReq {
    /// 开始时间, format 00:00 - 23:59
    ///
    /// **示例值**: "00:00"
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: Option<String>,
    /// 结束时间, format 00:00 - 23:59
    ///
    /// **示例值**: "24:00"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: Option<String>,
    /// 星期几, 1 - Monday, 2 - Tuesday, 3 - Wednesday, 4 - Thursday, 5 - Friday, 6 - Saturday, 7 - Sunday, 9 - Everyday, 10 - Weekday, 11 - Weekend
    ///
    /// **示例值**: "9"
    #[serde(
        rename = "weekday",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub weekday: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateHelpdeskAgentScheduleRespInner {
    #[serde(flatten)]
    data: Option<CreateHelpdeskAgentScheduleResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateHelpdeskAgentScheduleResp {
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
            CreateHelpdeskAgentScheduleReq,
        ) -> Result<(CreateHelpdeskAgentScheduleResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    CreateHelpdeskAgentScheduleReq,
                )
                    -> Result<(CreateHelpdeskAgentScheduleResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HelpdeskServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_helpdesk_agent_schedule<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            CreateHelpdeskAgentScheduleReq,
            CreateHelpdeskAgentScheduleResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_helpdesk_agent_schedule(
            &self,
            req: &CreateHelpdeskAgentScheduleReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                CreateHelpdeskAgentScheduleReq,
                CreateHelpdeskAgentScheduleResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::helpdesk::create_helpdesk_agent_schedule::{
            CreateHelpdeskAgentScheduleReq, CreateHelpdeskAgentScheduleResp,
            CreateHelpdeskAgentScheduleRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .helpdesk()
            .mock()
            .mock_create_helpdesk_agent_schedule(|_| {
                Ok((
                    CreateHelpdeskAgentScheduleResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .helpdesk()
            .create_helpdesk_agent_schedule(CreateHelpdeskAgentScheduleReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .helpdesk()
            .create_helpdesk_agent_schedule(CreateHelpdeskAgentScheduleReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "agent_schedules": [
        {
            "agent_id": "agent-id",
            "schedule": [
                {
                    "start_time": "00:00",
                    "end_time": "24:00",
                    "weekday": 9
                }
            ],
            "agent_skill_ids": [
                "test-skill-id"
            ]
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateHelpdeskAgentScheduleReqBody>(REQ) {
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
        let res = serde_json::from_str::<CreateHelpdeskAgentScheduleRespInner>(RESP);
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
