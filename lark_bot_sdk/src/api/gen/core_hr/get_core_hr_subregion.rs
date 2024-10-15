//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/subregion/get>
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
    /// **api 版本: 2023-07-14T06:40:27+00:00**
    ///
    /// ## 查询单条城市/区域信息
    ///
    /// 查询单条城市/区域信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/subregion/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/basic-infomation/location_data/get-3>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Fbasic-infomation%2Flocation_data%2Fget-3>
    pub async fn get_core_hr_subregion(
        &self,
        req: GetCoreHrSubregionReq,
    ) -> Result<(GetCoreHrSubregionResp, CommonResponse), Error> {
        self.get_core_hr_subregion_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_core_hr_subregion](#method.get_core_hr_subregion) 函数
    pub async fn get_core_hr_subregion_with_opt(
        &self,
        req: GetCoreHrSubregionReq,
        method_option: MethodOption,
    ) -> Result<(GetCoreHrSubregionResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_core_hr_subregion(&req) {
                tracing::info!("[lark] CoreHr#GetCoreHrSubregion **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#GetCoreHrSubregion call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "GetCoreHrSubregion",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/subregions/:subregion_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCoreHrSubregionRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetCoreHrSubregionReq {
    /// 城市/区域 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "67489937334909845"
    #[api(kind = "path", name = "subregion_id")]
    pub subregion_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCoreHrSubregionRespInner {
    #[serde(flatten)]
    data: Option<GetCoreHrSubregionResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCoreHrSubregionResp {
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
    /// 城市/区域信息
    #[serde(
        rename = "subregion",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub subregion: SubregionSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SubregionSubResp {
    /// 城市/区域id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 城市/区域名称
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Vec<I18nSubResp>,
    /// 所属省份/行政区id，详细信息可通过【查询省份/行政区信息】接口查询获得
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "subdivision_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub subdivision_id: String,
    /// 上级城市/区域区id
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "superior_subregion_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub superior_subregion_id: String,
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
        Fn(GetCoreHrSubregionReq) -> Result<(GetCoreHrSubregionResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetCoreHrSubregionReq) -> Result<(GetCoreHrSubregionResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_core_hr_subregion<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetCoreHrSubregionReq, GetCoreHrSubregionResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_core_hr_subregion(
            &self,
            req: &GetCoreHrSubregionReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetCoreHrSubregionReq, GetCoreHrSubregionResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::get_core_hr_subregion::{
            GetCoreHrSubregionReq, GetCoreHrSubregionResp, GetCoreHrSubregionRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_get_core_hr_subregion(|_| {
                Ok((GetCoreHrSubregionResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .core_hr()
            .get_core_hr_subregion(GetCoreHrSubregionReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .get_core_hr_subregion(GetCoreHrSubregionReq::default())
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
        "subregion": {
            "id": "12",
            "name": [
                {
                    "lang": "zh-CN",
                    "value": "张三"
                }
            ],
            "subdivision_id": "12",
            "superior_subregion_id": "12"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCoreHrSubregionRespInner>(RESP);
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
