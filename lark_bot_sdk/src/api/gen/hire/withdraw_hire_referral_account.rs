//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/withdraw>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::hire::HireService;

impl<'c, IStore: Store, IClient: HttpClient> HireService<'c, IStore, IClient> {
    /// **api 版本: 2024-08-02T03:52:47+00:00**
    ///
    /// ## 全额提取内推账户余额
    ///
    /// 通过账号 ID 全额提取内推账号下的积分。全额提现后，内推人在飞书招聘系统中的积分余额会变为 0，对应的积分奖励状态也会变为「已发放」。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/withdraw>
    ///
    /// new doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/withdraw>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FukTMukTMukTM%2FuMzM1YjLzMTN24yMzUjN%2Fhire-v1%2Freferral_account%2Fwithdraw>
    pub async fn withdraw_hire_referral_account(
        &self,
        req: WithdrawHireReferralAccountReq,
    ) -> Result<(WithdrawHireReferralAccountResp, CommonResponse), Error> {
        self.withdraw_hire_referral_account_with_opt(req, Default::default())
            .await
    }

    /// 参见 [withdraw_hire_referral_account](#method.withdraw_hire_referral_account) 函数
    pub async fn withdraw_hire_referral_account_with_opt(
        &self,
        req: WithdrawHireReferralAccountReq,
        method_option: MethodOption,
    ) -> Result<(WithdrawHireReferralAccountResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_withdraw_hire_referral_account(&req) {
                tracing::info!("[lark] Hire#WithdrawHireReferralAccount **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#WithdrawHireReferralAccount call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "WithdrawHireReferralAccount",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/referral_account/:referral_account_id/withdraw",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (WithdrawHireReferralAccountRespInner, _) =
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
pub struct WithdrawHireReferralAccountReq {
    /// 账户 ID，通过[注册内推账户](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/create)生成
    ///
    /// **示例值**: "6942778198054125570"
    #[api(kind = "path", name = "referral_account_id")]
    pub referral_account_id: String,

    /// 提取的奖励类型，当前仅支持积分（1）提取
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "withdraw_bonus_type")]
    pub withdraw_bonus_type: Vec<Option<i64>>,
    /// 外部提取单 ID，由请求方提供，用于保证接口的幂等性，需要保证唯一。传入重复 ID 会返回原 ID 对应的提取详情
    ///
    /// **示例值**: "6942778198054125570"
    #[api(kind = "body", name = "external_order_id")]
    pub external_order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct WithdrawHireReferralAccountRespInner {
    #[serde(flatten)]
    data: Option<WithdrawHireReferralAccountResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WithdrawHireReferralAccountResp {
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
    /// 提取单 ID
    ///
    /// **示例值**: "6942778198054125570"
    #[serde(
        rename = "external_order_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub external_order_id: String,
    /// 交易时间，毫秒时间戳
    ///
    /// **示例值**: "1683634459543"
    #[serde(
        rename = "trans_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub trans_time: String,
    /// 提取详情
    #[serde(
        rename = "withdrawal_details",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub withdrawal_details: BonusAmountSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct BonusAmountSubResp {
    /// 提取的积分数量
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "point_bonus",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub point_bonus: i64,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::hire::HireServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            WithdrawHireReferralAccountReq,
        ) -> Result<(WithdrawHireReferralAccountResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    WithdrawHireReferralAccountReq,
                )
                    -> Result<(WithdrawHireReferralAccountResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_withdraw_hire_referral_account<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            WithdrawHireReferralAccountReq,
            WithdrawHireReferralAccountResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_withdraw_hire_referral_account(
            &self,
            req: &WithdrawHireReferralAccountReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                WithdrawHireReferralAccountReq,
                WithdrawHireReferralAccountResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::withdraw_hire_referral_account::{
            WithdrawHireReferralAccountReq, WithdrawHireReferralAccountResp,
            WithdrawHireReferralAccountRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_withdraw_hire_referral_account(|_| {
                Ok((
                    WithdrawHireReferralAccountResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .withdraw_hire_referral_account(WithdrawHireReferralAccountReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .withdraw_hire_referral_account(WithdrawHireReferralAccountReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "withdraw_bonus_type": [
        1
    ],
    "external_order_id": "6942778198054125570"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::WithdrawHireReferralAccountReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "SUCCESS",
    "data": {
        "external_order_id": "6942778198054125570",
        "trans_time": "1683634459543",
        "withdrawal_details": {
            "point_bonus": 100
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<WithdrawHireReferralAccountRespInner>(RESP);
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
