//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-member/clear>
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
    /// ## 删除公共邮箱所有成员
    ///
    /// 删除公共邮箱所有成员。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-member/clear>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/mail-v1/public-mailbox/public_mailbox-member/clear>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fpublic-mailbox%2Fpublic_mailbox-member%2Fclear>
    pub async fn clear_public_mailbox_member(
        &self,
        req: ClearPublicMailboxMemberReq,
    ) -> Result<(ClearPublicMailboxMemberResp, CommonResponse), Error> {
        self.clear_public_mailbox_member_with_opt(req, Default::default())
            .await
    }

    /// 参见 [clear_public_mailbox_member](#method.clear_public_mailbox_member) 函数
    pub async fn clear_public_mailbox_member_with_opt(
        &self,
        req: ClearPublicMailboxMemberReq,
        method_option: MethodOption,
    ) -> Result<(ClearPublicMailboxMemberResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_clear_public_mailbox_member(&req) {
                tracing::info!("[lark] Mail#ClearPublicMailboxMember **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#ClearPublicMailboxMember call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "ClearPublicMailboxMember",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/clear",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (ClearPublicMailboxMemberRespInner, _) =
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
pub struct ClearPublicMailboxMemberReq {
    /// 公共邮箱唯一标识或公共邮箱地址
    ///
    /// **示例值**: "xxxxxxxxxxxxxxx 或 test_public_mailbox@xxx.xx"
    #[api(kind = "path", name = "public_mailbox_id")]
    pub public_mailbox_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct ClearPublicMailboxMemberRespInner {
    #[serde(flatten)]
    data: Option<ClearPublicMailboxMemberResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClearPublicMailboxMemberResp {
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
            ClearPublicMailboxMemberReq,
        ) -> Result<(ClearPublicMailboxMemberResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    ClearPublicMailboxMemberReq,
                ) -> Result<(ClearPublicMailboxMemberResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_clear_public_mailbox_member<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            ClearPublicMailboxMemberReq,
            ClearPublicMailboxMemberResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_clear_public_mailbox_member(
            &self,
            req: &ClearPublicMailboxMemberReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                ClearPublicMailboxMemberReq,
                ClearPublicMailboxMemberResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::mail::clear_public_mailbox_member::{
            ClearPublicMailboxMemberReq, ClearPublicMailboxMemberResp,
            ClearPublicMailboxMemberRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_clear_public_mailbox_member(|_| {
                Ok((
                    ClearPublicMailboxMemberResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .mail()
            .clear_public_mailbox_member(ClearPublicMailboxMemberReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .mail()
            .clear_public_mailbox_member(ClearPublicMailboxMemberReq::default())
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
        let res = serde_json::from_str::<ClearPublicMailboxMemberRespInner>(RESP);
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
