//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/period_rule/list>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::okr::OkrService;

impl<'c, IStore: Store, IClient: HttpClient> OkrService<'c, IStore, IClient> {
    /// **api 版本: 2024-01-09T02:16:17+00:00**
    ///
    /// ## 获取 OKR 周期规则
    ///
    /// 获取租户的周期规则列表。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/period_rule/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/okr-v1/period_rule/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fokr-v1%2Fperiod_rule%2Flist>
    pub async fn get_okr_period_rule_list(
        &self,
        req: GetOkrPeriodRuleListReq,
    ) -> Result<(GetOkrPeriodRuleListResp, CommonResponse), Error> {
        self.get_okr_period_rule_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_okr_period_rule_list](#method.get_okr_period_rule_list) 函数
    pub async fn get_okr_period_rule_list_with_opt(
        &self,
        req: GetOkrPeriodRuleListReq,
        method_option: MethodOption,
    ) -> Result<(GetOkrPeriodRuleListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_okr_period_rule_list(&req) {
                tracing::info!("[lark] Okr#GetOkrPeriodRuleList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Okr#GetOkrPeriodRuleList call api");

        let req = ApiRequest {
            scope: "Okr",
            api: "GetOkrPeriodRuleList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/okr/v1/period_rules",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetOkrPeriodRuleListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetOkrPeriodRuleListReq {}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetOkrPeriodRuleListRespInner {
    #[serde(flatten)]
    data: Option<GetOkrPeriodRuleListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetOkrPeriodRuleListResp {
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
    /// 周期规则列表
    #[serde(
        rename = "period_rules",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub period_rules: Vec<PeriodRuleSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PeriodRuleSubResp {
    /// 周期规则ID
    ///
    /// **示例值**: "134"
    #[serde(
        rename = "period_rule_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub period_rule_id: String,
    /// 周期类型
    ///
    /// - year: 年度周期
    ///
    /// - month: 月度周期
    ///
    /// **示例值**: "year"
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 周期长度（月)
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "length",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub length: i64,
    /// 每年首个开始月份
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "first_month",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub first_month: i64,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::okr::OkrServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetOkrPeriodRuleListReq) -> Result<(GetOkrPeriodRuleListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetOkrPeriodRuleListReq,
                ) -> Result<(GetOkrPeriodRuleListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> OkrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_okr_period_rule_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetOkrPeriodRuleListReq,
            GetOkrPeriodRuleListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_okr_period_rule_list(
            &self,
            req: &GetOkrPeriodRuleListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetOkrPeriodRuleListReq, GetOkrPeriodRuleListResp, Arc<dyn MockFunc>>(
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
        api::gen::okr::get_okr_period_rule_list::{
            GetOkrPeriodRuleListReq, GetOkrPeriodRuleListResp, GetOkrPeriodRuleListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .okr()
            .mock()
            .mock_get_okr_period_rule_list(|_| {
                Ok((
                    GetOkrPeriodRuleListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .okr()
            .get_okr_period_rule_list(GetOkrPeriodRuleListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .okr()
            .get_okr_period_rule_list(GetOkrPeriodRuleListReq::default())
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
        "period_rules": [
            {
                "period_rule_id": "134",
                "type": "year",
                "length": 12,
                "first_month": 12
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetOkrPeriodRuleListRespInner>(RESP);
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