//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check_custom_field/batch_delete>
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
    /// **api 版本: 2024-07-30T08:24:25+00:00**
    ///
    /// ## 删除背调自定义字段
    ///
    /// 删除用户在发起背调时展示的表单自定义字段。
    ///
    /// * 删除操作不影响已创建的背调。
    ///
    /// * 该账号下被删除的自定义字段的key（详见[创建背调自定义字段](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check_custom_field/create)）不能被重复使用。
    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check_custom_field/batch_delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/ecological-docking/eco_background_check_custom_field/batch_delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fecological-docking%2Feco_background_check_custom_field%2Fbatch_delete>
    pub async fn batch_delete_hire_eco_background_check_custom_field(
        &self,
        req: BatchDeleteHireEcoBackgroundCheckCustomFieldReq,
    ) -> Result<
        (
            BatchDeleteHireEcoBackgroundCheckCustomFieldResp,
            CommonResponse,
        ),
        Error,
    > {
        self.batch_delete_hire_eco_background_check_custom_field_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_delete_hire_eco_background_check_custom_field](#method.batch_delete_hire_eco_background_check_custom_field) 函数
    pub async fn batch_delete_hire_eco_background_check_custom_field_with_opt(
        &self,
        req: BatchDeleteHireEcoBackgroundCheckCustomFieldReq,
        method_option: MethodOption,
    ) -> Result<
        (
            BatchDeleteHireEcoBackgroundCheckCustomFieldResp,
            CommonResponse,
        ),
        Error,
    > {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_batch_delete_hire_eco_background_check_custom_field(&req)
            {
                tracing::info!(
                    "[lark] Hire#BatchDeleteHireEcoBackgroundCheckCustomField **mocking** api"
                );
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#BatchDeleteHireEcoBackgroundCheckCustomField call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "BatchDeleteHireEcoBackgroundCheckCustomField",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/eco_background_check_custom_fields/batch_delete",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchDeleteHireEcoBackgroundCheckCustomFieldRespInner, _) =
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
pub struct BatchDeleteHireEcoBackgroundCheckCustomFieldReq {
    /// 背调账号 ID，可通过[账号绑定](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_account/events/created)事件获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6995842370159937061"
    #[api(kind = "body", name = "account_id")]
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchDeleteHireEcoBackgroundCheckCustomFieldRespInner {
    #[serde(flatten)]
    data: Option<BatchDeleteHireEcoBackgroundCheckCustomFieldResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteHireEcoBackgroundCheckCustomFieldResp {
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

    use self::gen::hire::HireServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            BatchDeleteHireEcoBackgroundCheckCustomFieldReq,
        ) -> Result<
            (
                BatchDeleteHireEcoBackgroundCheckCustomFieldResp,
                CommonResponse,
            ),
            Error,
        > + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchDeleteHireEcoBackgroundCheckCustomFieldReq,
                ) -> Result<
                    (
                        BatchDeleteHireEcoBackgroundCheckCustomFieldResp,
                        CommonResponse,
                    ),
                    Error,
                > + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_delete_hire_eco_background_check_custom_field<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchDeleteHireEcoBackgroundCheckCustomFieldReq,
            BatchDeleteHireEcoBackgroundCheckCustomFieldResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_delete_hire_eco_background_check_custom_field(
            &self,
            req: &BatchDeleteHireEcoBackgroundCheckCustomFieldReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BatchDeleteHireEcoBackgroundCheckCustomFieldReq,
                BatchDeleteHireEcoBackgroundCheckCustomFieldResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::batch_delete_hire_eco_background_check_custom_field::{
            BatchDeleteHireEcoBackgroundCheckCustomFieldReq,
            BatchDeleteHireEcoBackgroundCheckCustomFieldResp,
            BatchDeleteHireEcoBackgroundCheckCustomFieldRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_batch_delete_hire_eco_background_check_custom_field(|_| {
                Ok((
                    BatchDeleteHireEcoBackgroundCheckCustomFieldResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .batch_delete_hire_eco_background_check_custom_field(
                BatchDeleteHireEcoBackgroundCheckCustomFieldReq::default(),
            )
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .batch_delete_hire_eco_background_check_custom_field(
                BatchDeleteHireEcoBackgroundCheckCustomFieldReq::default(),
            )
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "account_id": "6995842370159937061"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) =
            serde_json::from_str::<super::BatchDeleteHireEcoBackgroundCheckCustomFieldReqBody>(REQ)
        {
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
        let res =
            serde_json::from_str::<BatchDeleteHireEcoBackgroundCheckCustomFieldRespInner>(RESP);
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
