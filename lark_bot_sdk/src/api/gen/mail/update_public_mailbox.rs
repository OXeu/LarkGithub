//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/update>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::mail::MailService;

impl<'c, IStore: Store, IClient: HttpClient> MailService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-25T02:11:12+00:00**
    ///
    /// ## 修改公共邮箱全部信息
    ///
    /// 更新公共邮箱所有信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/update>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/mail-v1/public-mailbox/public_mailbox/update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fpublic-mailbox%2Fpublic_mailbox%2Fupdate>
    pub async fn update_public_mailbox(
        &self,
        req: UpdatePublicMailboxReq,
    ) -> Result<(UpdatePublicMailboxResp, CommonResponse), Error> {
        self.update_public_mailbox_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_public_mailbox](#method.update_public_mailbox) 函数
    pub async fn update_public_mailbox_with_opt(
        &self,
        req: UpdatePublicMailboxReq,
        method_option: MethodOption,
    ) -> Result<(UpdatePublicMailboxResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_public_mailbox(&req) {
                tracing::info!("[lark] Mail#UpdatePublicMailbox **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#UpdatePublicMailbox call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "UpdatePublicMailbox",
            method: http::Method::PUT,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/mail/v1/public_mailboxes/:public_mailbox_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdatePublicMailboxRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdatePublicMailboxReq {
    /// 公共邮箱唯一标识或公共邮箱地址
    ///
    /// **示例值**: "xxxxxxxxxxxxxxx 或 test_public_mailbox@xxx.xx"
    #[api(kind = "path", name = "public_mailbox_id")]
    pub public_mailbox_id: String,

    /// 公共邮箱地址
    ///
    /// **示例值**: "test_public_mailbox@xxx.xx"
    #[api(kind = "body", name = "email")]
    pub email: Option<String>,
    /// 公共邮箱名称
    ///
    /// **示例值**: "test public mailbox"
    #[api(kind = "body", name = "name")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdatePublicMailboxRespInner {
    #[serde(flatten)]
    data: Option<UpdatePublicMailboxResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdatePublicMailboxResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: PublicMailboxSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PublicMailboxSubResp {
    /// 公共邮箱唯一标识
    ///
    /// **示例值**: "xxxxxxxxxxxxxxx"
    #[serde(
        rename = "public_mailbox_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub public_mailbox_id: String,
    /// 公共邮箱地址
    ///
    /// **示例值**: "test_public_mailbox@xxx.xx"
    #[serde(
        rename = "email",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email: String,
    /// 公共邮箱名称
    ///
    /// **示例值**: "test public mailbox"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::mail::MailServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(UpdatePublicMailboxReq) -> Result<(UpdatePublicMailboxResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdatePublicMailboxReq,
                ) -> Result<(UpdatePublicMailboxResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_public_mailbox<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdatePublicMailboxReq, UpdatePublicMailboxResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_public_mailbox(
            &self,
            req: &UpdatePublicMailboxReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdatePublicMailboxReq, UpdatePublicMailboxResp, Arc<dyn MockFunc>>(
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
        api::gen::mail::update_public_mailbox::{
            UpdatePublicMailboxReq, UpdatePublicMailboxResp, UpdatePublicMailboxRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_update_public_mailbox(|_| {
                Ok((
                    UpdatePublicMailboxResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .mail()
            .update_public_mailbox(UpdatePublicMailboxReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .mail()
            .update_public_mailbox(UpdatePublicMailboxReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
   "name": "xxx",
   "email": "xxx@xxx.xxx"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdatePublicMailboxReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "public_mailbox_id": "xxx",
        "email": "xx@xx.xx",
        "name":"xxx"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdatePublicMailboxRespInner>(RESP);
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