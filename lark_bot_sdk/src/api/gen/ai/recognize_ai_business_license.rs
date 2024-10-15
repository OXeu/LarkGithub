//! doc url: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/business_license/recognize>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{
    ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqData, StreamReqParam,
};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::ai::AiService;

impl<'c, IStore: Store, IClient: HttpClient> AiService<'c, IStore, IClient> {
    /// **api 版本: 2023-12-22T02:50:16+00:00**
    ///
    /// ## 识别文件中的营业执照
    ///
    /// 营业执照识别接口，支持JPG/JPEG/PNG/BMP/PDF五种文件类型的一次性的识别。
    ///
    /// 单租户限流：10QPS，同租户下的应用没有限流，共享本租户的 10QPS 限流
    ///
    /// doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/business_license/recognize>
    ///
    /// new doc: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/business_license/recognize>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Freference%2Fai%2Fdocument_ai-v1%2Fbusiness_license%2Frecognize>
    pub async fn recognize_ai_business_license<Data: StreamReqData>(
        &self,
        req: RecognizeAiBusinessLicenseReq<Data>,
    ) -> Result<(RecognizeAiBusinessLicenseResp, CommonResponse), Error> {
        self.recognize_ai_business_license_with_opt(req, Default::default())
            .await
    }

    /// 参见 [recognize_ai_business_license](#method.recognize_ai_business_license) 函数
    pub async fn recognize_ai_business_license_with_opt<Data: StreamReqData>(
        &self,
        req: RecognizeAiBusinessLicenseReq<Data>,
        method_option: MethodOption,
    ) -> Result<(RecognizeAiBusinessLicenseResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_recognize_ai_business_license(&req) {
                tracing::info!("[lark] Ai#RecognizeAiBusinessLicense **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Ai#RecognizeAiBusinessLicense call api");

        let req = ApiRequest::<()> {
            scope: "Ai",
            api: "RecognizeAiBusinessLicense",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/document_ai/v1/business_license/recognize",
            stream_param_data: req.gen_stream_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (RecognizeAiBusinessLicenseRespInner, _) =
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

#[derive(Debug, lark_bot_sdk_macros::ApiReqParams)]
pub struct RecognizeAiBusinessLicenseReq<Data: StreamReqData> {
    #[api(kind = "stream", name = "#data#", option = "false")]
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct RecognizeAiBusinessLicenseRespInner {
    #[serde(flatten)]
    data: Option<RecognizeAiBusinessLicenseResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecognizeAiBusinessLicenseResp {
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
    /// 营业执照信息
    #[serde(
        rename = "business_license",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub business_license: BusinessLicenseSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct BusinessLicenseSubResp {
    /// 识别出的实体类型
    #[serde(
        rename = "entities",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub entities: Vec<BusinessEntitySubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct BusinessEntitySubResp {
    /// 识别的字段种类
    ///
    /// **示例值**: "legal_representative"
    ///
    /// **可选值**:
    ///
    /// `CertificateType`: 证书类型
    ///
    /// `UnifiedSocialCreditCode`: 统一社会信用代码
    ///
    /// `CompanyName`: 公司名称
    ///
    /// `CompanyType`: 公司类型
    ///
    /// `Domicile`: 住所
    ///
    /// `LegalRepresentative`: 法定代表人
    ///
    /// `RegisteredCapital`: 注册资本
    ///
    /// `EstablishedTime`: 成立日期
    ///
    /// `EstablishedDate`: 营业期限
    ///
    /// `BusinessScope`: 经营范围
    ///
    /// `Website`: 企业信用信息公示系统网址
    ///
    /// `ApprovalDate`: 核准日期
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 识别出字段的文本信息
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

    use self::gen::ai::AiServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc<D: StreamReqData>:
        Fn(
            RecognizeAiBusinessLicenseReq<D>,
        ) -> Result<(RecognizeAiBusinessLicenseResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            D: StreamReqData,
            T: Fn(
                    RecognizeAiBusinessLicenseReq<D>,
                )
                    -> Result<(RecognizeAiBusinessLicenseResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc<D> for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AiServiceMocker<'c, IStore, IClient> {
        pub fn mock_recognize_ai_business_license<T: StreamReqData, F: MockFunc<T>>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            RecognizeAiBusinessLicenseReq<T>,
            RecognizeAiBusinessLicenseResp,
            Arc<dyn MockFunc<T>>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_recognize_ai_business_license<T: StreamReqData>(
            &self,
            req: &RecognizeAiBusinessLicenseReq<T>,
        ) -> Option<Arc<dyn MockFunc<T>>> {
            do_mock::<
                Mocker,
                RecognizeAiBusinessLicenseReq<T>,
                RecognizeAiBusinessLicenseResp,
                Arc<dyn MockFunc<T>>,
            >(self.cli.instance_id, req)
        }
    }
}