//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-bank/search>
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
    /// **api 版本: 2024-06-29T10:22:43+00:00**
    ///
    /// ## 查询银行信息
    ///
    /// 根据银行 ID 、银行名称，查询银行信息
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-bank/search>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-bank/search>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Fcorehr-v2%2Fbasic_info-bank%2Fsearch>
    pub async fn search_core_hr_bank(
        &self,
        req: SearchCoreHrBankReq,
    ) -> Result<(SearchCoreHrBankResp, CommonResponse), Error> {
        self.search_core_hr_bank_with_opt(req, Default::default())
            .await
    }

    /// 参见 [search_core_hr_bank](#method.search_core_hr_bank) 函数
    pub async fn search_core_hr_bank_with_opt(
        &self,
        req: SearchCoreHrBankReq,
        method_option: MethodOption,
    ) -> Result<(SearchCoreHrBankResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_search_core_hr_bank(&req) {
                tracing::info!("[lark] CoreHr#SearchCoreHrBank **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#SearchCoreHrBank call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "SearchCoreHrBank",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v2/basic_info/banks/search",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (SearchCoreHrBankRespInner, _) = self.cli.do_req(req).await?;
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
pub struct SearchCoreHrBankReq {
    /// 分页大小，最大 100
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "100"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "MDBH00000100"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 银行 ID 列表，可通过[批量查询员工信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/batch_get)等接口返回的 `person_info.bank_account_list.bank_id_v2` 字段获取
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `100` 字符
    #[api(kind = "body", name = "bank_id_list")]
    pub bank_id_list: Vec<Option<String>>,
    /// 银行名称列表，支持对银行名称精确搜索
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `100` 字符
    #[api(kind = "body", name = "bank_name_list")]
    pub bank_name_list: Vec<Option<String>>,
    /// 状态列表
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `2` 字符
    #[api(kind = "body", name = "status_list")]
    pub status_list: Vec<Option<i64>>,
    /// 最早更新时间
    ///
    /// **示例值**: "2020-01-01 00:00:00"
    #[api(kind = "body", name = "update_start_time")]
    pub update_start_time: Option<String>,
    /// 最晚更新时间
    ///
    /// **示例值**: "2024-01-01 00:00:00"
    #[api(kind = "body", name = "update_end_time")]
    pub update_end_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct SearchCoreHrBankRespInner {
    #[serde(flatten)]
    data: Option<SearchCoreHrBankResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchCoreHrBankResp {
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
    /// 查询到的银行列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<BankSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="
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
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct BankSubResp {
    /// 银行 ID
    ///
    /// **示例值**: "MDBH00000080"
    #[serde(
        rename = "bank_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub bank_id: String,
    /// 银行名称
    #[serde(
        rename = "bank_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub bank_name: Vec<I18nSubResp>,
    /// 总行代码
    ///
    /// **示例值**: "CMB"
    #[serde(
        rename = "bank_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub bank_code: String,
    /// 国家 / 地区 ID ，可通过[查询国家 / 地区信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-country_region/search) 接口查询
    ///
    /// **示例值**: "6862995757234914824"
    #[serde(
        rename = "country_region_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub country_region_id: String,
    /// 状态
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `active`: 生效
    ///
    /// `inactive`: 失效
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 创建时间
    ///
    /// **示例值**: "2020-01-01 00:00:00"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 更新时间
    ///
    /// **示例值**: "2024-01-01 00:00:00"
    #[serde(
        rename = "update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 语言编码（IETF BCP 47）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh-CN"
    #[serde(
        rename = "lang",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub lang: String,
    /// 文本内容
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "中文示例"
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
        Fn(SearchCoreHrBankReq) -> Result<(SearchCoreHrBankResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(SearchCoreHrBankReq) -> Result<(SearchCoreHrBankResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_search_core_hr_bank<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, SearchCoreHrBankReq, SearchCoreHrBankResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_search_core_hr_bank(
            &self,
            req: &SearchCoreHrBankReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, SearchCoreHrBankReq, SearchCoreHrBankResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::search_core_hr_bank::{
            SearchCoreHrBankReq, SearchCoreHrBankResp, SearchCoreHrBankRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_search_core_hr_bank(|_| {
                Ok((SearchCoreHrBankResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .core_hr()
            .search_core_hr_bank(SearchCoreHrBankReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .search_core_hr_bank(SearchCoreHrBankReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "bank_id_list": [
        "MDBH00000080"
    ],
    "bank_name_list": [
        "招商银行"
    ],
    "status_list": [
        1
    ],
    "update_start_time": "2020-01-01 00:00:00",
    "update_end_time": "2024-01-01 00:00:00"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::SearchCoreHrBankReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            {
                "bank_id": "MDBH00000080",
                "bank_name": [
                    {
                        "lang": "zh-CN",
                        "value": "中文示例"
                    }
                ],
                "bank_code": "CMB",
                "country_region_id": "6862995757234914824",
                "status": 1,
                "create_time": "2020-01-01 00:00:00",
                "update_time": "2024-01-01 00:00:00"
            }
        ],
        "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
        "has_more": true
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<SearchCoreHrBankRespInner>(RESP);
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