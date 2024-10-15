//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/answer_user_query>
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
    /// **api 版本: 2023-08-15T07:34:21+00:00**
    ///
    /// ## 回复用户在工单里的提问
    ///
    /// 该接口用于回复用户提问结果至工单，需要工单仍处于进行中且未接入人工状态。仅支持自建应用。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/answer_user_query>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/helpdesk-v1/ticket-management/ticket/answer_user_query>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhelpdesk-v1%2Fticket-management%2Fticket%2Fanswer_user_query>
    pub async fn answer_helpdesk_ticket_user_query(
        &self,
        req: AnswerHelpdeskTicketUserQueryReq,
    ) -> Result<(AnswerHelpdeskTicketUserQueryResp, CommonResponse), Error> {
        self.answer_helpdesk_ticket_user_query_with_opt(req, Default::default())
            .await
    }

    /// 参见 [answer_helpdesk_ticket_user_query](#method.answer_helpdesk_ticket_user_query) 函数
    pub async fn answer_helpdesk_ticket_user_query_with_opt(
        &self,
        req: AnswerHelpdeskTicketUserQueryReq,
        method_option: MethodOption,
    ) -> Result<(AnswerHelpdeskTicketUserQueryResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_answer_helpdesk_ticket_user_query(&req) {
                tracing::info!("[lark] Helpdesk#AnswerHelpdeskTicketUserQuery **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Helpdesk#AnswerHelpdeskTicketUserQuery call api");

        let req = ApiRequest {
            scope: "Helpdesk",
            api: "AnswerHelpdeskTicketUserQuery",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/helpdesk/v1/tickets/:ticket_id/answer_user_query",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (AnswerHelpdeskTicketUserQueryRespInner, _) =
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
pub struct AnswerHelpdeskTicketUserQueryReq {
    /// 工单ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6945345902185807891"
    #[api(kind = "path", name = "ticket_id")]
    pub ticket_id: String,

    /// 事件ID,可从订阅事件中提取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "abcd"
    #[api(kind = "body", name = "event_id")]
    pub event_id: String,
    /// faq结果列表
    #[api(kind = "body", name = "faqs")]
    pub faqs: Vec<Option<UserQueryFaqInfoSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserQueryFaqInfoSubReq {
    /// faq服务台内唯一标识
    ///
    /// **示例值**: "12345"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: Option<String>,
    /// faq匹配得分
    ///
    /// **示例值**: "0.9"
    #[serde(
        rename = "score",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub score: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct AnswerHelpdeskTicketUserQueryRespInner {
    #[serde(flatten)]
    data: Option<AnswerHelpdeskTicketUserQueryResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AnswerHelpdeskTicketUserQueryResp {
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
            AnswerHelpdeskTicketUserQueryReq,
        ) -> Result<(AnswerHelpdeskTicketUserQueryResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    AnswerHelpdeskTicketUserQueryReq,
                )
                    -> Result<(AnswerHelpdeskTicketUserQueryResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HelpdeskServiceMocker<'c, IStore, IClient> {
        pub fn mock_answer_helpdesk_ticket_user_query<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            AnswerHelpdeskTicketUserQueryReq,
            AnswerHelpdeskTicketUserQueryResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_answer_helpdesk_ticket_user_query(
            &self,
            req: &AnswerHelpdeskTicketUserQueryReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                AnswerHelpdeskTicketUserQueryReq,
                AnswerHelpdeskTicketUserQueryResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::helpdesk::answer_helpdesk_ticket_user_query::{
            AnswerHelpdeskTicketUserQueryReq, AnswerHelpdeskTicketUserQueryResp,
            AnswerHelpdeskTicketUserQueryRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .helpdesk()
            .mock()
            .mock_answer_helpdesk_ticket_user_query(|_| {
                Ok((
                    AnswerHelpdeskTicketUserQueryResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .helpdesk()
            .answer_helpdesk_ticket_user_query(AnswerHelpdeskTicketUserQueryReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .helpdesk()
            .answer_helpdesk_ticket_user_query(AnswerHelpdeskTicketUserQueryReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "event_id": "abcd",
    "faqs": [
        {
            "id": "12345",
            "score": 0.9
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::AnswerHelpdeskTicketUserQueryReqBody>(REQ) {
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
        let res = serde_json::from_str::<AnswerHelpdeskTicketUserQueryRespInner>(RESP);
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
