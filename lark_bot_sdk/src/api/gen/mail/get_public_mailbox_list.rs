//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/list>
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
    /// ## 查询所有公共邮箱
    ///
    /// 分页批量获取公共邮箱列表。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/mail-v1/public-mailbox/public_mailbox/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fpublic-mailbox%2Fpublic_mailbox%2Flist>
    pub async fn get_public_mailbox_list(
        &self,
        req: GetPublicMailboxListReq,
    ) -> Result<(GetPublicMailboxListResp, CommonResponse), Error> {
        self.get_public_mailbox_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_public_mailbox_list](#method.get_public_mailbox_list) 函数
    pub async fn get_public_mailbox_list_with_opt(
        &self,
        req: GetPublicMailboxListReq,
        method_option: MethodOption,
    ) -> Result<(GetPublicMailboxListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_public_mailbox_list(&req) {
                tracing::info!("[lark] Mail#GetPublicMailboxList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#GetPublicMailboxList call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "GetPublicMailboxList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/mail/v1/public_mailboxes",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetPublicMailboxListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetPublicMailboxListReq {
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "xxx"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 分页大小
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetPublicMailboxListRespInner {
    #[serde(flatten)]
    data: Option<GetPublicMailboxListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetPublicMailboxListResp {
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
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "xxx"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 公共邮箱列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<PublicMailboxSubResp>,
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
    /// 数据驻留地
    ///
    /// **示例值**: "cn"
    #[serde(
        rename = "geo",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub geo: String,
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
        Fn(GetPublicMailboxListReq) -> Result<(GetPublicMailboxListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetPublicMailboxListReq,
                ) -> Result<(GetPublicMailboxListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_public_mailbox_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetPublicMailboxListReq,
            GetPublicMailboxListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_public_mailbox_list(
            &self,
            req: &GetPublicMailboxListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetPublicMailboxListReq, GetPublicMailboxListResp, Arc<dyn MockFunc>>(
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
        api::gen::mail::get_public_mailbox_list::{
            GetPublicMailboxListReq, GetPublicMailboxListResp, GetPublicMailboxListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_get_public_mailbox_list(|_| {
                Ok((
                    GetPublicMailboxListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .mail()
            .get_public_mailbox_list(GetPublicMailboxListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .mail()
            .get_public_mailbox_list(GetPublicMailboxListReq::default())
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
        "has_more": true,
        "page_token": "xxx",
        "items": [
            {
                "public_mailbox_id": "xxxxxxxxxxxxxxx",
                "email": "test_public_mailbox@xxx.xx",
                "name": "test public mailbox",
                "geo": "cn"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetPublicMailboxListRespInner>(RESP);
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
