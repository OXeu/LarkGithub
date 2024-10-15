//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/attachment/get>
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
    /// ## 获取附件信息
    ///
    /// 根据附件 ID 和附件类型获取招聘系统中附件的信息，比如附件名称、附件创建时间、附件下载地址等。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/attachment/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/attachment/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fattachment%2Fget>
    pub async fn get_hire_attachment(
        &self,
        req: GetHireAttachmentReq,
    ) -> Result<(GetHireAttachmentResp, CommonResponse), Error> {
        self.get_hire_attachment_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_hire_attachment](#method.get_hire_attachment) 函数
    pub async fn get_hire_attachment_with_opt(
        &self,
        req: GetHireAttachmentReq,
        method_option: MethodOption,
    ) -> Result<(GetHireAttachmentResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_hire_attachment(&req) {
                tracing::info!("[lark] Hire#GetHireAttachment **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#GetHireAttachment call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "GetHireAttachment",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/attachments/:attachment_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetHireAttachmentRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetHireAttachmentReq {
    /// 附件 ID，可通过[获取人才信息 V1](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/get)接口获取人才的简历附件 ID/作品附件 ID/通用附件 ID。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6960663240925956555"
    #[api(kind = "path", name = "attachment_id")]
    pub attachment_id: String,
    /// 附件类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `附件简历`: 简历附件，人才上的简历附件。
    ///
    /// `候选人作品`: 作品附件，人才上的作品附件。
    ///
    /// `自定义附件`: 通用附件，通过[创建附件](https://open.feishu.cn/document/ukTMukTMukTM/uIDN1YjLyQTN24iM0UjN/create_attachment)
    ///
    /// 接口创建的附件，或者业务接口中返回的非`简历附件`和`作品附件`类型的附件，如「自定义字段附件」、「Offer 附件」等。
    #[api(kind = "query", name = "type", v_type = "var", option = "false")]
    pub query_type: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetHireAttachmentRespInner {
    #[serde(flatten)]
    data: Option<GetHireAttachmentResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetHireAttachmentResp {
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
    /// 附件信息
    #[serde(
        rename = "attachment",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub attachment: AttachmentSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AttachmentSubResp {
    /// 附件 ID
    ///
    /// **示例值**: "6949805467799537964"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 附件下载地址，有效期为 30 分钟
    ///
    /// **示例值**: "https://hire.feishu.cn/blob/xx/"
    #[serde(
        rename = "url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub url: String,
    /// 附件文件名
    ///
    /// **示例值**: "xx的简历.prd"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 附件媒体类型/MIME
    ///
    /// **示例值**: "application/pdf"
    #[serde(
        rename = "mime",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mime: String,
    /// 附件创建时间戳（单位ms）
    ///
    /// **示例值**: "1618899376480"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: i64,
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
        Fn(GetHireAttachmentReq) -> Result<(GetHireAttachmentResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetHireAttachmentReq) -> Result<(GetHireAttachmentResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_hire_attachment<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetHireAttachmentReq, GetHireAttachmentResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_hire_attachment(
            &self,
            req: &GetHireAttachmentReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetHireAttachmentReq, GetHireAttachmentResp, Arc<dyn MockFunc>>(
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
        api::gen::hire::get_hire_attachment::{
            GetHireAttachmentReq, GetHireAttachmentResp, GetHireAttachmentRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_get_hire_attachment(|_| {
                Ok((GetHireAttachmentResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .hire()
            .get_hire_attachment(GetHireAttachmentReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .get_hire_attachment(GetHireAttachmentReq::default())
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
        "attachment": {
            "id": "6949805467799537964",
            "url": "https://hire.feishu.cn/blob/xx/",
            "name": "xx的简历.prd",
            "mime": "application/pdf",
            "create_time": 1618899376480
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetHireAttachmentRespInner>(RESP);
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