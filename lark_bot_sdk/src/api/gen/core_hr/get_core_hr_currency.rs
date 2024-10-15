//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/currency/get>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::core_hr::CoreHrService;

impl<'c, IStore: Store, IClient: HttpClient> CoreHrService<'c, IStore, IClient> {
    /// **api 版本: 2023-07-14T06:40:30+00:00**
    ///
    /// ## 查询单个货币信息
    ///
    /// 查询单个货币信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/currency/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/basic-infomation/currency/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fbasic-infomation%2Fcurrency%2Fget>
    pub async fn get_core_hr_currency(
        &self,
        req: GetCoreHrCurrencyReq,
    ) -> Result<(GetCoreHrCurrencyResp, CommonResponse), Error> {
        self.get_core_hr_currency_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_core_hr_currency](#method.get_core_hr_currency) 函数
    pub async fn get_core_hr_currency_with_opt(
        &self,
        req: GetCoreHrCurrencyReq,
        method_option: MethodOption,
    ) -> Result<(GetCoreHrCurrencyResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_core_hr_currency(&req) {
                tracing::info!("[lark] CoreHr#GetCoreHrCurrency **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#GetCoreHrCurrency call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "GetCoreHrCurrency",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/currencies/:currency_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCoreHrCurrencyRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetCoreHrCurrencyReq {
    /// 货币 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "67489937334909845"
    #[api(kind = "path", name = "currency_id")]
    pub currency_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCoreHrCurrencyRespInner {
    #[serde(flatten)]
    data: Option<GetCoreHrCurrencyResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCoreHrCurrencyResp {
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
    /// 货币信息
    #[serde(
        rename = "currency",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency: CurrencySubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CurrencySubResp {
    /// 货币id
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 货币所属国家/地区id，详细信息可通过【查询国家/地区信息】接口查询获得
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "country_region_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub country_region_id: String,
    /// 货币名称
    #[serde(
        rename = "currency_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency_name: Vec<I18nSubResp>,
    /// 数字代码
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "numeric_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub numeric_code: i64,
    /// 三位字母代码
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "currency_alpha_3_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency_alpha_3_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 名称信息的语言
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh-CN"
    #[serde(
        rename = "lang",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub lang: String,
    /// 名称信息的内容
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "张三"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::core_hr::CoreHrServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetCoreHrCurrencyReq) -> Result<(GetCoreHrCurrencyResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetCoreHrCurrencyReq) -> Result<(GetCoreHrCurrencyResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_core_hr_currency<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetCoreHrCurrencyReq, GetCoreHrCurrencyResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_core_hr_currency(
            &self,
            req: &GetCoreHrCurrencyReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetCoreHrCurrencyReq, GetCoreHrCurrencyResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::get_core_hr_currency::{
            GetCoreHrCurrencyReq, GetCoreHrCurrencyResp, GetCoreHrCurrencyRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_get_core_hr_currency(|_| {
                Ok((GetCoreHrCurrencyResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .core_hr()
            .get_core_hr_currency(GetCoreHrCurrencyReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .get_core_hr_currency(GetCoreHrCurrencyReq::default())
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
        "currency": {
            "id": "1",
            "country_region_id": "12",
            "currency_name": [
                {
                    "lang": "zh-CN",
                    "value": "张三"
                }
            ],
            "numeric_code": 12,
            "currency_alpha_3_code": "12"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCoreHrCurrencyRespInner>(RESP);
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