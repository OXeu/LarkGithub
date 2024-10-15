//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/acs-v1/rule_external/device_bind>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::acs::AcsService;

impl<'c, IStore: Store, IClient: HttpClient> AcsService<'c, IStore, IClient> {
    /// **api 版本: 2024-01-09T09:10:22+00:00**
    ///
    /// ## 设备绑定权限组
    ///
    /// 设备绑定权限组
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/acs-v1/rule_external/device_bind>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/acs-v1/rule_external/device_bind>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Facs-v1%2Frule_external%2Fdevice_bind>
    pub async fn device_bind_acs_rule_external(
        &self,
        req: DeviceBindAcsRuleExternalReq,
    ) -> Result<(DeviceBindAcsRuleExternalResp, CommonResponse), Error> {
        self.device_bind_acs_rule_external_with_opt(req, Default::default())
            .await
    }

    /// 参见 [device_bind_acs_rule_external](#method.device_bind_acs_rule_external) 函数
    pub async fn device_bind_acs_rule_external_with_opt(
        &self,
        req: DeviceBindAcsRuleExternalReq,
        method_option: MethodOption,
    ) -> Result<(DeviceBindAcsRuleExternalResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_device_bind_acs_rule_external(&req) {
                tracing::info!("[lark] Acs#DeviceBindAcsRuleExternal **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Acs#DeviceBindAcsRuleExternal call api");

        let req = ApiRequest {
            scope: "Acs",
            api: "DeviceBindAcsRuleExternal",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/acs/v1/rule_external/device_bind",
            param_data: req.gen_param(),
            method_option,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeviceBindAcsRuleExternalRespInner, _) =
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
pub struct DeviceBindAcsRuleExternalReq {
    /// 设备id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6939433228970082593"
    #[api(kind = "body", name = "device_id")]
    pub device_id: String,
    /// 权限组id列表
    ///
    /// **是否必填**: 是
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `0` 字符- `10000` 字符
    #[api(kind = "body", name = "rule_ids")]
    pub rule_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeviceBindAcsRuleExternalRespInner {
    #[serde(flatten)]
    data: Option<DeviceBindAcsRuleExternalResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeviceBindAcsRuleExternalResp {
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

    use self::gen::acs::AcsServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            DeviceBindAcsRuleExternalReq,
        ) -> Result<(DeviceBindAcsRuleExternalResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeviceBindAcsRuleExternalReq,
                )
                    -> Result<(DeviceBindAcsRuleExternalResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AcsServiceMocker<'c, IStore, IClient> {
        pub fn mock_device_bind_acs_rule_external<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            DeviceBindAcsRuleExternalReq,
            DeviceBindAcsRuleExternalResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_device_bind_acs_rule_external(
            &self,
            req: &DeviceBindAcsRuleExternalReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                DeviceBindAcsRuleExternalReq,
                DeviceBindAcsRuleExternalResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::acs::device_bind_acs_rule_external::{
            DeviceBindAcsRuleExternalReq, DeviceBindAcsRuleExternalResp,
            DeviceBindAcsRuleExternalRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .acs()
            .mock()
            .mock_device_bind_acs_rule_external(|_| {
                Ok((
                    DeviceBindAcsRuleExternalResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .acs()
            .device_bind_acs_rule_external(DeviceBindAcsRuleExternalReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .acs()
            .device_bind_acs_rule_external(DeviceBindAcsRuleExternalReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "device_id": "6939433228970082593",
    "rule_ids": [
        "7298933941867135276"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::DeviceBindAcsRuleExternalReqBody>(REQ) {
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
        let res = serde_json::from_str::<DeviceBindAcsRuleExternalRespInner>(RESP);
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