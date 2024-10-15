//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user/query>
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
    /// **api 版本: 2024-07-25T02:11:26+00:00**
    ///
    /// ## 查询邮箱地址状态
    ///
    /// 使用邮箱状态查询接口，可以输入邮箱地址，查询出该邮箱地址对应的类型以及状态。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user/query>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/mail-v1/user/query>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fuser%2Fquery>
    pub async fn get_mail_user(
        &self,
        req: GetMailUserReq,
    ) -> Result<(GetMailUserResp, CommonResponse), Error> {
        self.get_mail_user_with_opt(req, Default::default()).await
    }

    /// 参见 [get_mail_user](#method.get_mail_user) 函数
    pub async fn get_mail_user_with_opt(
        &self,
        req: GetMailUserReq,
        method_option: MethodOption,
    ) -> Result<(GetMailUserResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_mail_user(&req) {
                tracing::info!("[lark] Mail#GetMailUser **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#GetMailUser call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "GetMailUser",
            method: http::Method::POST,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/mail/v1/users/query",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetMailUserRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetMailUserReq {
    /// 需要查询的邮箱地址列表
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "["aaa@lark.com","bbb@lark.com"]"
    #[api(kind = "body", name = "email_list")]
    pub email_list: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetMailUserRespInner {
    #[serde(flatten)]
    data: Option<GetMailUserResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetMailUserResp {
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
    /// 邮箱地址以及其对应的类型类型和状态
    #[serde(
        rename = "user_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_list: Vec<UserSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserSubResp {
    /// 邮箱地址
    ///
    /// **示例值**: "aaa@lark.com"
    #[serde(
        rename = "email",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email: String,
    /// 邮箱地址状态
    ///
    /// **示例值**: "4"
    ///
    /// **可选值**:
    ///
    /// `Invalid`: 邮箱地址格式错误
    ///
    /// `DomainNotExist`: 邮箱地址域名不存在
    ///
    /// `AddressNotExist`: 邮箱地址不存在
    ///
    /// `Onboard`: 启用
    ///
    /// `Deleted`: 已删除（邮箱回收站中）
    ///
    /// `Forbidden`: 禁用
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 邮箱地址类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `UserMailbox`: 成员邮箱
    ///
    /// `UserMailboxAlias`: 成员邮箱别名
    ///
    /// `PublicMailbox`: 公共邮箱
    ///
    /// `PublicMailboxAlias`: 公共邮箱别名
    ///
    /// `MailGroup`: 邮件组
    ///
    /// `MailGroupAlias`: 邮件组别名
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: i64,
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
        Fn(GetMailUserReq) -> Result<(GetMailUserResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetMailUserReq) -> Result<(GetMailUserResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_mail_user<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetMailUserReq, GetMailUserResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_mail_user(
            &self,
            req: &GetMailUserReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetMailUserReq, GetMailUserResp, Arc<dyn MockFunc>>(
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
        api::gen::mail::get_mail_user::{GetMailUserReq, GetMailUserResp, GetMailUserRespInner},
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_get_mail_user(|_| Ok((GetMailUserResp::default(), CommonResponse::default())))
            .build();
        let res = lark.mail().get_mail_user(GetMailUserReq::default()).await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark.mail().get_mail_user(GetMailUserReq::default()).await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "email_list": [
        "test@a.com"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::GetMailUserReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "user_list": [
            {
                "email": "aaa@lark.com",
                "status": 4,
                "type": 1
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetMailUserRespInner>(RESP);
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