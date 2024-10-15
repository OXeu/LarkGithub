//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/delete>
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
    /// **api 版本: 2024-07-25T02:11:24+00:00**
    ///
    /// ## 永久删除公共邮箱
    ///
    /// 该接口会永久删除公共邮箱地址。可用于释放邮箱回收站的公共邮箱地址，一旦删除，该邮箱地址将无法恢复。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/mail-v1/public-mailbox/public_mailbox/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fpublic-mailbox%2Fpublic_mailbox%2Fdelete>
    pub async fn delete_public_mailbox(
        &self,
        req: DeletePublicMailboxReq,
    ) -> Result<(DeletePublicMailboxResp, CommonResponse), Error> {
        self.delete_public_mailbox_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_public_mailbox](#method.delete_public_mailbox) 函数
    pub async fn delete_public_mailbox_with_opt(
        &self,
        req: DeletePublicMailboxReq,
        method_option: MethodOption,
    ) -> Result<(DeletePublicMailboxResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_public_mailbox(&req) {
                tracing::info!("[lark] Mail#DeletePublicMailbox **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#DeletePublicMailbox call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "DeletePublicMailbox",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/mail/v1/public_mailboxes/:public_mailbox_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeletePublicMailboxRespInner, _) = self.cli.do_req(req).await?;
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
pub struct DeletePublicMailboxReq {
    /// 要释放的公共邮箱地址
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "xxxxxx@abc.com"
    #[api(kind = "path", name = "public_mailbox_id")]
    pub public_mailbox_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeletePublicMailboxRespInner {
    #[serde(flatten)]
    data: Option<DeletePublicMailboxResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeletePublicMailboxResp {
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

    use self::gen::mail::MailServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(DeletePublicMailboxReq) -> Result<(DeletePublicMailboxResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeletePublicMailboxReq,
                ) -> Result<(DeletePublicMailboxResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_public_mailbox<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, DeletePublicMailboxReq, DeletePublicMailboxResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_public_mailbox(
            &self,
            req: &DeletePublicMailboxReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, DeletePublicMailboxReq, DeletePublicMailboxResp, Arc<dyn MockFunc>>(
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
        api::gen::mail::delete_public_mailbox::{
            DeletePublicMailboxReq, DeletePublicMailboxResp, DeletePublicMailboxRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_delete_public_mailbox(|_| {
                Ok((
                    DeletePublicMailboxResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .mail()
            .delete_public_mailbox(DeletePublicMailboxReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .mail()
            .delete_public_mailbox(DeletePublicMailboxReq::default())
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
    "data": {},
    "msg": "release mail address success"
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<DeletePublicMailboxRespInner>(RESP);
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
