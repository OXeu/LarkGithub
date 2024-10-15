//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-subscription/patch>
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
    /// **api 版本: 2023-12-29T07:31:09+00:00**
    ///
    /// ## 更新订阅状态
    ///
    /// 根据订阅ID更新订阅状态
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-subscription/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/docs-assistant/file-subscription/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fdocs-assistant%2Ffile-subscription%2Fpatch>
    pub async fn update_drive_file_subscription(
        &self,
        req: UpdateDriveFileSubscriptionReq,
    ) -> Result<(UpdateDriveFileSubscriptionResp, CommonResponse), Error> {
        self.update_drive_file_subscription_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_drive_file_subscription](#method.update_drive_file_subscription) 函数
    pub async fn update_drive_file_subscription_with_opt(
        &self,
        req: UpdateDriveFileSubscriptionReq,
        method_option: MethodOption,
    ) -> Result<(UpdateDriveFileSubscriptionResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_drive_file_subscription(&req) {
                tracing::info!("[lark] Drive#UpdateDriveFileSubscription **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#UpdateDriveFileSubscription call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "UpdateDriveFileSubscription",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/files/:file_token/subscriptions/:subscription_id",
            param_data: req.gen_param(),
            method_option,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateDriveFileSubscriptionRespInner, _) =
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
pub struct UpdateDriveFileSubscriptionReq {
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

    /// 是否订阅
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[api(kind = "body", name = "is_subscribe")]
    pub is_subscribe: bool,
    /// 文档类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "docx"
    ///
    /// **可选值**:
    ///
    /// `doc`: 文档
    ///
    /// `docx`: 新版文档
    ///
    /// `wiki`: 知识库wiki
    #[api(kind = "body", name = "file_type")]
    pub file_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateDriveFileSubscriptionRespInner {
    #[serde(flatten)]
    data: Option<UpdateDriveFileSubscriptionResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDriveFileSubscriptionResp {
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
            UpdateDriveFileSubscriptionReq,
        ) -> Result<(UpdateDriveFileSubscriptionResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateDriveFileSubscriptionReq,
                )
                    -> Result<(UpdateDriveFileSubscriptionResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_drive_file_subscription<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateDriveFileSubscriptionReq,
            UpdateDriveFileSubscriptionResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_drive_file_subscription(
            &self,
            req: &UpdateDriveFileSubscriptionReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateDriveFileSubscriptionReq,
                UpdateDriveFileSubscriptionResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::drive::update_drive_file_subscription::{
            UpdateDriveFileSubscriptionReq, UpdateDriveFileSubscriptionResp,
            UpdateDriveFileSubscriptionRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_update_drive_file_subscription(|_| {
                Ok((
                    UpdateDriveFileSubscriptionResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .update_drive_file_subscription(UpdateDriveFileSubscriptionReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .update_drive_file_subscription(UpdateDriveFileSubscriptionReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "is_subscribe": true,
    "file_type": "docx"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateDriveFileSubscriptionReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "subscription_id": "1234567890987654321",
        "subscription_type": "comment_update",
        "is_subcribe": true,
        "file_type": "docx"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateDriveFileSubscriptionRespInner>(RESP);
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
