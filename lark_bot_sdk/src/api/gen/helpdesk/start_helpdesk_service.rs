//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/start_service>
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
    /// **api 版本: 2023-08-15T07:34:20+00:00**
    ///
    /// ## 创建服务台对话
    ///
    /// 该接口用于创建服务台对话。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/start_service>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/helpdesk-v1/ticket-management/ticket/start_service>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhelpdesk-v1%2Fticket-management%2Fticket%2Fstart_service>
    pub async fn start_helpdesk_service(
        &self,
        req: StartHelpdeskServiceReq,
    ) -> Result<(StartHelpdeskServiceResp, CommonResponse), Error> {
        self.start_helpdesk_service_with_opt(req, Default::default())
            .await
    }

    /// 参见 [start_helpdesk_service](#method.start_helpdesk_service) 函数
    pub async fn start_helpdesk_service_with_opt(
        &self,
        req: StartHelpdeskServiceReq,
        method_option: MethodOption,
    ) -> Result<(StartHelpdeskServiceResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_start_helpdesk_service(&req) {
                tracing::info!("[lark] Helpdesk#StartHelpdeskService **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Helpdesk#StartHelpdeskService call api");

        let req = ApiRequest {
            scope: "Helpdesk",
            api: "StartHelpdeskService",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/helpdesk/v1/start_service",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (StartHelpdeskServiceRespInner, _) = self.cli.do_req(req).await?;
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
pub struct StartHelpdeskServiceReq {
    /// 是否直接进入人工(若appointed_agents填写了，该值为必填)
    ///
    /// **示例值**: "false"
    #[api(kind = "body", name = "human_service")]
    pub human_service: Option<bool>,
    /// 客服 open ids (获取方式参考[获取单个用户信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/get))，human_service需要为true
    ///
    /// **示例值**: "[ou_7dab8a3d3cdcc9da365777c7ad535d62]"
    #[api(kind = "body", name = "appointed_agents")]
    pub appointed_agents: Vec<Option<String>>,
    /// 用户 open id,(获取方式参考[获取单个用户信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/get))
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ou_7dab8a3d3cdcc9da365777c7ad535d62"
    #[api(kind = "body", name = "open_id")]
    pub open_id: String,
    /// 工单来源自定义信息，长度限制1024字符，如设置，[获取工单详情](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/get)会返回此信息
    ///
    /// **示例值**: "测试自定义字段信息"
    #[api(kind = "body", name = "customized_info")]
    pub customized_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct StartHelpdeskServiceRespInner {
    #[serde(flatten)]
    data: Option<StartHelpdeskServiceResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StartHelpdeskServiceResp {
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
    /// 客服群open ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "创建的 chat-id"
    #[serde(
        rename = "chat_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_id: String,
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
        Fn(StartHelpdeskServiceReq) -> Result<(StartHelpdeskServiceResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    StartHelpdeskServiceReq,
                ) -> Result<(StartHelpdeskServiceResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HelpdeskServiceMocker<'c, IStore, IClient> {
        pub fn mock_start_helpdesk_service<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            StartHelpdeskServiceReq,
            StartHelpdeskServiceResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_start_helpdesk_service(
            &self,
            req: &StartHelpdeskServiceReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, StartHelpdeskServiceReq, StartHelpdeskServiceResp, Arc<dyn MockFunc>>(
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
        api::gen::helpdesk::start_helpdesk_service::{
            StartHelpdeskServiceReq, StartHelpdeskServiceResp, StartHelpdeskServiceRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .helpdesk()
            .mock()
            .mock_start_helpdesk_service(|_| {
                Ok((
                    StartHelpdeskServiceResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .helpdesk()
            .start_helpdesk_service(StartHelpdeskServiceReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .helpdesk()
            .start_helpdesk_service(StartHelpdeskServiceReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "human_service": false,
    "appointed_agents": [
        "ou_7dab8a3d3cdcc9da365777c7ad535d62"
    ],
    "open_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62",
    "customized_info": "测试自定义字段信息"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::StartHelpdeskServiceReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "chat_id": "创建的 chat-id"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<StartHelpdeskServiceRespInner>(RESP);
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