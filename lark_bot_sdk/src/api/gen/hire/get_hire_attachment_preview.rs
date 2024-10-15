//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/attachment/preview>
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
    /// **api 版本: 2024-07-11T04:13:51+00:00**
    ///
    /// ## 获取人才简历附件 PDF 格式下载链接
    ///
    /// 根据人才简历附件 ID 获取该简历附件对应的 PDF 文件的下载地址。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/attachment/preview>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/attachment/preview>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fattachment%2Fpreview>
    pub async fn get_hire_attachment_preview(
        &self,
        req: GetHireAttachmentPreviewReq,
    ) -> Result<(GetHireAttachmentPreviewResp, CommonResponse), Error> {
        self.get_hire_attachment_preview_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_hire_attachment_preview](#method.get_hire_attachment_preview) 函数
    pub async fn get_hire_attachment_preview_with_opt(
        &self,
        req: GetHireAttachmentPreviewReq,
        method_option: MethodOption,
    ) -> Result<(GetHireAttachmentPreviewResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_hire_attachment_preview(&req) {
                tracing::info!("[lark] Hire#GetHireAttachmentPreview **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#GetHireAttachmentPreview call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "GetHireAttachmentPreview",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/attachments/:attachment_id/preview",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetHireAttachmentPreviewRespInner, _) =
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
pub struct GetHireAttachmentPreviewReq {
    /// 附件 ID，可通过[获取人才信息](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/get)接口返回数据中获取人才简历附件 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "64352523512563462"
    #[api(kind = "path", name = "attachment_id")]
    pub attachment_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetHireAttachmentPreviewRespInner {
    #[serde(flatten)]
    data: Option<GetHireAttachmentPreviewResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetHireAttachmentPreviewResp {
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
    /// PDF 文件下载链接，有效期为 30 分钟
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "https://xxx.feishu.cn/hire/file/blob/ZXlKaGJHY2lPaUpJVXpJMU5pS3==/"
    #[serde(
        rename = "url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub url: String,
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
            GetHireAttachmentPreviewReq,
        ) -> Result<(GetHireAttachmentPreviewResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetHireAttachmentPreviewReq,
                ) -> Result<(GetHireAttachmentPreviewResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_hire_attachment_preview<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetHireAttachmentPreviewReq,
            GetHireAttachmentPreviewResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_hire_attachment_preview(
            &self,
            req: &GetHireAttachmentPreviewReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetHireAttachmentPreviewReq,
                GetHireAttachmentPreviewResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::hire::get_hire_attachment_preview::{
            GetHireAttachmentPreviewReq, GetHireAttachmentPreviewResp,
            GetHireAttachmentPreviewRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_get_hire_attachment_preview(|_| {
                Ok((
                    GetHireAttachmentPreviewResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .get_hire_attachment_preview(GetHireAttachmentPreviewReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .get_hire_attachment_preview(GetHireAttachmentPreviewReq::default())
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
        "url": "https://xxx.feishu.cn/hire/file/blob/ZXlKaGJHY2lPaUpJVXpJMU5pS3==/"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetHireAttachmentPreviewRespInner>(RESP);
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
