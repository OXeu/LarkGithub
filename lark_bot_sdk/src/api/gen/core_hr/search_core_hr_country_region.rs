//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-country_region/search>
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
    /// **api 版本: 2024-06-29T10:20:47+00:00**
    ///
    /// ## 查询国家 / 地区信息
    ///
    /// 根据国家/地区 ID、状态，批量查询国家/地区信息
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-country_region/search>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-country_region/search>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Fcorehr-v2%2Fbasic_info-country_region%2Fsearch>
    pub async fn search_core_hr_country_region(
        &self,
        req: SearchCoreHrCountryRegionReq,
    ) -> Result<(SearchCoreHrCountryRegionResp, CommonResponse), Error> {
        self.search_core_hr_country_region_with_opt(req, Default::default())
            .await
    }

    /// 参见 [search_core_hr_country_region](#method.search_core_hr_country_region) 函数
    pub async fn search_core_hr_country_region_with_opt(
        &self,
        req: SearchCoreHrCountryRegionReq,
        method_option: MethodOption,
    ) -> Result<(SearchCoreHrCountryRegionResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_search_core_hr_country_region(&req) {
                tracing::info!("[lark] CoreHr#SearchCoreHrCountryRegion **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#SearchCoreHrCountryRegion call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "SearchCoreHrCountryRegion",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v2/basic_info/country_regions/search",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (SearchCoreHrCountryRegionRespInner, _) =
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
pub struct SearchCoreHrCountryRegionReq {
    /// 分页大小，最大 100
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "100"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "6862995772275688974"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 国家/地区 ID 列表，不填写则返回全部
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `100` 字符
    #[api(kind = "body", name = "country_region_id_list")]
    pub country_region_id_list: Vec<Option<String>>,
    /// 国家/地区数据的状态列表，不填写则返回全部
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `2` 字符
    #[api(kind = "body", name = "status_list")]
    pub status_list: Vec<Option<i64>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct SearchCoreHrCountryRegionRespInner {
    #[serde(flatten)]
    data: Option<SearchCoreHrCountryRegionResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchCoreHrCountryRegionResp {
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
    /// 查询到的国家/地区列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<CountryRegionSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "6862995772275688974"
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
pub struct CountryRegionSubResp {
    /// 国家/地区 ID
    ///
    /// **示例值**: "6862995757234914824"
    #[serde(
        rename = "country_region_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub country_region_id: String,
    /// 国家/地区名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Vec<I18nSubResp>,
    /// 国家/地区全称
    #[serde(
        rename = "full_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub full_name: Vec<I18nSubResp>,
    /// 国家/地区两位字母编码（ISO 3166-1）
    ///
    /// **示例值**: "CN"
    #[serde(
        rename = "alpha_2_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub alpha_2_code: String,
    /// 国家/地区三位字母编码（ISO 3166-1）
    ///
    /// **示例值**: "CHN"
    #[serde(
        rename = "alpha_3_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub alpha_3_code: String,
    /// 国际电话区号
    ///
    /// **示例值**: "+86"
    #[serde(
        rename = "global_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub global_code: String,
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
    /// **示例值**: "中国"
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
        Fn(
            SearchCoreHrCountryRegionReq,
        ) -> Result<(SearchCoreHrCountryRegionResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    SearchCoreHrCountryRegionReq,
                )
                    -> Result<(SearchCoreHrCountryRegionResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_search_core_hr_country_region<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            SearchCoreHrCountryRegionReq,
            SearchCoreHrCountryRegionResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_search_core_hr_country_region(
            &self,
            req: &SearchCoreHrCountryRegionReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                SearchCoreHrCountryRegionReq,
                SearchCoreHrCountryRegionResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::search_core_hr_country_region::{
            SearchCoreHrCountryRegionReq, SearchCoreHrCountryRegionResp,
            SearchCoreHrCountryRegionRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_search_core_hr_country_region(|_| {
                Ok((
                    SearchCoreHrCountryRegionResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .search_core_hr_country_region(SearchCoreHrCountryRegionReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .search_core_hr_country_region(SearchCoreHrCountryRegionReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "country_region_id_list": [
        "6862995791674344967"
    ],
    "status_list": [
        1
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::SearchCoreHrCountryRegionReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            {
                "country_region_id": "6862995757234914824",
                "name": [
                    {
                        "lang": "zh-CN",
                        "value": "中国"
                    }
                ],
                "full_name": [
                    {
                        "lang": "zh-CN",
                        "value": "中华人民共和国"
                    }
                ],
                "alpha_2_code": "CN",
                "alpha_3_code": "CHN",
                "global_code": "+86",
                "status": 1
            }
        ],
        "page_token": "6862995772275688974",
        "has_more": true
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<SearchCoreHrCountryRegionRespInner>(RESP);
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