//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-member/delete>
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
    /// **api 版本: 2024-07-25T02:11:25+00:00**
    ///
    /// ## 删除公共邮箱单个成员
    ///
    /// 删除公共邮箱单个成员。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-member/delete>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/mail-v1/public-mailbox/public_mailbox-member/delete>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fpublic-mailbox%2Fpublic_mailbox-member%2Fdelete>
    pub async fn delete_public_mailbox_member(
        &self,
        req: DeletePublicMailboxMemberReq,
    ) -> Result<(DeletePublicMailboxMemberResp, CommonResponse), Error> {
        self.delete_public_mailbox_member_with_opt(req, Default::default())
            .await
    }

    /// 参见 [delete_public_mailbox_member](#method.delete_public_mailbox_member) 函数
    pub async fn delete_public_mailbox_member_with_opt(
        &self,
        req: DeletePublicMailboxMemberReq,
        method_option: MethodOption,
    ) -> Result<(DeletePublicMailboxMemberResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_delete_public_mailbox_member(&req) {
                tracing::info!("[lark] Mail#DeletePublicMailboxMember **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#DeletePublicMailboxMember call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "DeletePublicMailboxMember",
            method: http::Method::DELETE,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/:member_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (DeletePublicMailboxMemberRespInner, _) =
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
pub struct DeletePublicMailboxMemberReq {
    /// 公共邮箱唯一标识或公共邮箱地址
    ///
    /// **示例值**: "xxxxxxxxxxxxxxx 或 test_public_mailbox@xxx.xx"
    #[api(kind = "path", name = "public_mailbox_id")]
    pub public_mailbox_id: String,
    /// 公共邮箱内成员唯一标识
    ///
    /// **示例值**: "xxxxxxxxxxxxxxx"
    #[api(kind = "path", name = "member_id")]
    pub member_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct DeletePublicMailboxMemberRespInner {
    #[serde(flatten)]
    data: Option<DeletePublicMailboxMemberResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeletePublicMailboxMemberResp {
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
        Fn(
            DeletePublicMailboxMemberReq,
        ) -> Result<(DeletePublicMailboxMemberResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    DeletePublicMailboxMemberReq,
                )
                    -> Result<(DeletePublicMailboxMemberResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_delete_public_mailbox_member<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            DeletePublicMailboxMemberReq,
            DeletePublicMailboxMemberResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_delete_public_mailbox_member(
            &self,
            req: &DeletePublicMailboxMemberReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                DeletePublicMailboxMemberReq,
                DeletePublicMailboxMemberResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::mail::delete_public_mailbox_member::{
            DeletePublicMailboxMemberReq, DeletePublicMailboxMemberResp,
            DeletePublicMailboxMemberRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_delete_public_mailbox_member(|_| {
                Ok((
                    DeletePublicMailboxMemberResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .mail()
            .delete_public_mailbox_member(DeletePublicMailboxMemberReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .mail()
            .delete_public_mailbox_member(DeletePublicMailboxMemberReq::default())
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
    "data": {}
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<DeletePublicMailboxMemberRespInner>(RESP);
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