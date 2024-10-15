//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_account_custom_field/batch_update>
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
    /// **api 版本: 2023-11-02T08:52:01+00:00**
    ///
    /// ## 更新帐号自定义字段
    ///
    /// 更新用户在服务商处的身份标示字段（如用户在服务商处的租户 ID），此方法只会更新同一 scope 内 key 一致的自定义字段。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_account_custom_field/batch_update>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/ecological-docking/eco_account_custom_field/batch_update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fecological-docking%2Feco_account_custom_field%2Fbatch_update>
    pub async fn batch_update_hire_eco_account_custom_field(
        &self,
        req: BatchUpdateHireEcoAccountCustomFieldReq,
    ) -> Result<(BatchUpdateHireEcoAccountCustomFieldResp, CommonResponse), Error> {
        self.batch_update_hire_eco_account_custom_field_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_update_hire_eco_account_custom_field](#method.batch_update_hire_eco_account_custom_field) 函数
    pub async fn batch_update_hire_eco_account_custom_field_with_opt(
        &self,
        req: BatchUpdateHireEcoAccountCustomFieldReq,
        method_option: MethodOption,
    ) -> Result<(BatchUpdateHireEcoAccountCustomFieldResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_batch_update_hire_eco_account_custom_field(&req)
            {
                tracing::info!("[lark] Hire#BatchUpdateHireEcoAccountCustomField **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#BatchUpdateHireEcoAccountCustomField call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "BatchUpdateHireEcoAccountCustomField",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/eco_account_custom_fields/batch_update",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchUpdateHireEcoAccountCustomFieldRespInner, _) =
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
pub struct BatchUpdateHireEcoAccountCustomFieldReq {
    /// 适用范围
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `背调`: 背调
    ///
    /// `笔试`: 笔试
    #[api(kind = "body", name = "scope")]
    pub scope: i64,
    /// 自定义字段列表
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "custom_field_list")]
    pub custom_field_list: Vec<Option<EcoAccountCustomFieldDataSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EcoAccountCustomFieldDataSubReq {
    /// 自定义字段的标识，同一 scope 内须唯一
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "org_id"
    #[serde(
        rename = "key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub key: String,
    /// 自定义字段的名称，用户在添加账号表单看到的控件标题
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: I18nSubReq,
    /// 是否必填
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_required",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_required: bool,
    /// 自定义字段的描述，用户在添加账号表单看到的 place holder
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: Option<I18nSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubReq {
    /// 中文
    ///
    /// **示例值**: "测试"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: Option<String>,
    /// 英文
    ///
    /// **示例值**: "test"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchUpdateHireEcoAccountCustomFieldRespInner {
    #[serde(flatten)]
    data: Option<BatchUpdateHireEcoAccountCustomFieldResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateHireEcoAccountCustomFieldResp {
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
            BatchUpdateHireEcoAccountCustomFieldReq,
        ) -> Result<(BatchUpdateHireEcoAccountCustomFieldResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchUpdateHireEcoAccountCustomFieldReq,
                )
                    -> Result<(BatchUpdateHireEcoAccountCustomFieldResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_update_hire_eco_account_custom_field<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchUpdateHireEcoAccountCustomFieldReq,
            BatchUpdateHireEcoAccountCustomFieldResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_update_hire_eco_account_custom_field(
            &self,
            req: &BatchUpdateHireEcoAccountCustomFieldReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BatchUpdateHireEcoAccountCustomFieldReq,
                BatchUpdateHireEcoAccountCustomFieldResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::batch_update_hire_eco_account_custom_field::{
            BatchUpdateHireEcoAccountCustomFieldReq, BatchUpdateHireEcoAccountCustomFieldResp,
            BatchUpdateHireEcoAccountCustomFieldRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_batch_update_hire_eco_account_custom_field(|_| {
                Ok((
                    BatchUpdateHireEcoAccountCustomFieldResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .batch_update_hire_eco_account_custom_field(
                BatchUpdateHireEcoAccountCustomFieldReq::default(),
            )
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .batch_update_hire_eco_account_custom_field(
                BatchUpdateHireEcoAccountCustomFieldReq::default(),
            )
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "scope": 1,
    "custom_field_list": [
        {
            "key": "org_id",
            "name": {
                "zh_cn": "测试",
                "en_us": "test"
            },
            "is_required": true,
            "description": {
                "zh_cn": "测试",
                "en_us": "test"
            }
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) =
            serde_json::from_str::<super::BatchUpdateHireEcoAccountCustomFieldReqBody>(REQ)
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
        let res = serde_json::from_str::<BatchUpdateHireEcoAccountCustomFieldRespInner>(RESP);
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
