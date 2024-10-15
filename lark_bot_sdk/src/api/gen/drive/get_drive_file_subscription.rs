//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-subscription/get>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::drive::DriveService;

impl<'c, IStore: Store, IClient: HttpClient> DriveService<'c, IStore, IClient> {
    /// **api 版本: 2023-12-29T07:31:08+00:00**
    ///
    /// ## 获取订阅状态
    ///
    /// 根据订阅ID获取该订阅的状态
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-subscription/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/docs-assistant/file-subscription/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fdocs-assistant%2Ffile-subscription%2Fget>
    pub async fn get_drive_file_subscription(
        &self,
        req: GetDriveFileSubscriptionReq,
    ) -> Result<(GetDriveFileSubscriptionResp, CommonResponse), Error> {
        self.get_drive_file_subscription_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_drive_file_subscription](#method.get_drive_file_subscription) 函数
    pub async fn get_drive_file_subscription_with_opt(
        &self,
        req: GetDriveFileSubscriptionReq,
        method_option: MethodOption,
    ) -> Result<(GetDriveFileSubscriptionResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_drive_file_subscription(&req) {
                tracing::info!("[lark] Drive#GetDriveFileSubscription **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#GetDriveFileSubscription call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "GetDriveFileSubscription",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/files/:file_token/subscriptions/:subscription_id",
            param_data: req.gen_param(),
            method_option,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetDriveFileSubscriptionRespInner, _) =
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
pub struct GetDriveFileSubscriptionReq {
    /// 文档token
    ///
    /// **示例值**: "doxcnxxxxxxxxxxxxxxxxxxxxxx"
    #[api(kind = "path", name = "file_token")]
    pub file_token: String,
    /// 订阅关系ID
    ///
    /// **示例值**: "1234567890987654321"
    #[api(kind = "path", name = "subscription_id")]
    pub subscription_id: String,

    /// 文档类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doc"
    ///
    /// **可选值**:
    ///
    /// `Docs`: 旧版文档
    ///
    /// `Upgraded Docs`: 新版文档
    ///
    /// `Wiki`: 云空间
    #[api(kind = "body", name = "file_type")]
    pub file_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetDriveFileSubscriptionRespInner {
    #[serde(flatten)]
    data: Option<GetDriveFileSubscriptionResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDriveFileSubscriptionResp {
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
    /// 订阅关系ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1234567890987654321"
    #[serde(
        rename = "subscription_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub subscription_id: String,
    /// 订阅类型
    ///
    /// **示例值**: "comment_update"
    ///
    /// **可选值**:
    ///
    /// `comment_update`: 评论更新
    #[serde(
        rename = "subscription_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub subscription_type: String,
    /// 是否订阅
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_subcribe",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_subcribe: bool,
    /// 文档类型
    ///
    /// **示例值**: "docx"
    ///
    /// **可选值**:
    ///
    /// `Docs`: 旧版文档
    ///
    /// `Upgraded Docs`: 新版文档
    ///
    /// `Wiki`: 知识库
    #[serde(
        rename = "file_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub file_type: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::drive::DriveServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            GetDriveFileSubscriptionReq,
        ) -> Result<(GetDriveFileSubscriptionResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetDriveFileSubscriptionReq,
                ) -> Result<(GetDriveFileSubscriptionResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_drive_file_subscription<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetDriveFileSubscriptionReq,
            GetDriveFileSubscriptionResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_drive_file_subscription(
            &self,
            req: &GetDriveFileSubscriptionReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetDriveFileSubscriptionReq,
                GetDriveFileSubscriptionResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::drive::get_drive_file_subscription::{
            GetDriveFileSubscriptionReq, GetDriveFileSubscriptionResp,
            GetDriveFileSubscriptionRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_get_drive_file_subscription(|_| {
                Ok((
                    GetDriveFileSubscriptionResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .get_drive_file_subscription(GetDriveFileSubscriptionReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .get_drive_file_subscription(GetDriveFileSubscriptionReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{"file_type":"docx"}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::GetDriveFileSubscriptionReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "data": {
        "file_type": "docx",
        "is_subcribe": false,
        "subscription_id": "xxxxxxxx",
        "subscription_type": "comment_update"
    },
    "msg": "success"
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetDriveFileSubscriptionRespInner>(RESP);
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
