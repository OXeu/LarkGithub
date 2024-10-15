//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup/update>
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
    /// **api 版本: 2024-07-25T02:11:09+00:00**
    ///
    /// ## 修改邮件组全部信息
    ///
    /// 更新邮件组所有信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup/update>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/mail-v1/mail-group/mailgroup/update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fmail-group%2Fmailgroup%2Fupdate>
    pub async fn update_mail_group(
        &self,
        req: UpdateMailGroupReq,
    ) -> Result<(UpdateMailGroupResp, CommonResponse), Error> {
        self.update_mail_group_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_mail_group](#method.update_mail_group) 函数
    pub async fn update_mail_group_with_opt(
        &self,
        req: UpdateMailGroupReq,
        method_option: MethodOption,
    ) -> Result<(UpdateMailGroupResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_mail_group(&req) {
                tracing::info!("[lark] Mail#UpdateMailGroup **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#UpdateMailGroup call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "UpdateMailGroup",
            method: http::Method::PUT,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/mail/v1/mailgroups/:mailgroup_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateMailGroupRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateMailGroupReq {
    /// 邮件组ID或者邮件组地址
    ///
    /// **示例值**: "xxxxxxxxxxxxxxx 或 test_mail_group@xxx.xx"
    #[api(kind = "path", name = "mailgroup_id")]
    pub mailgroup_id: String,

    /// 邮件组地址
    ///
    /// **示例值**: "test_mail_group@xxx.xx"
    #[api(kind = "body", name = "email")]
    pub email: Option<String>,
    /// 邮件组名称
    ///
    /// **示例值**: "test mail group"
    #[api(kind = "body", name = "name")]
    pub name: Option<String>,
    /// 邮件组描述
    ///
    /// **示例值**: "mail group for testing"
    #[api(kind = "body", name = "description")]
    pub description: Option<String>,
    /// 谁可发送邮件到此邮件组
    ///
    /// **示例值**: "ALL_INTERNAL_USERS"
    ///
    /// **可选值**:
    ///
    /// `ANYONE`: 任何人
    ///
    /// `ALL_INTERNAL_USERS`: 仅组织内部成员
    ///
    /// `ALL_GROUP_MEMBERS`: 仅邮件组成员
    ///
    /// `CUSTOM_MEMBERS`: 自定义成员
    #[api(kind = "body", name = "who_can_send_mail")]
    pub who_can_send_mail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateMailGroupRespInner {
    #[serde(flatten)]
    data: Option<UpdateMailGroupResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateMailGroupResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: MailgroupSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct MailgroupSubResp {
    /// 邮件组ID
    ///
    /// **示例值**: "xxxxxxxxxxxxxxx"
    #[serde(
        rename = "mailgroup_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mailgroup_id: String,
    /// 邮件组地址
    ///
    /// **示例值**: "test_mail_group@xxx.xx"
    #[serde(
        rename = "email",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email: String,
    /// 邮件组名称
    ///
    /// **示例值**: "test mail group"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 邮件组描述
    ///
    /// **示例值**: "mail group for testing"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 邮件组成员数量
    ///
    /// **示例值**: "10"
    #[serde(
        rename = "direct_members_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub direct_members_count: String,
    /// 是否包含外部成员
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "include_external_member",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub include_external_member: bool,
    /// 是否是全员邮件组
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "include_all_company_member",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub include_all_company_member: bool,
    /// 谁可发送邮件到此邮件组
    ///
    /// **示例值**: "ALL_INTERNAL_USERS"
    ///
    /// **可选值**:
    ///
    /// `ANYONE`: 任何人
    ///
    /// `ALL_INTERNAL_USERS`: 仅组织内部成员
    ///
    /// `ALL_GROUP_MEMBERS`: 仅邮件组成员
    ///
    /// `CUSTOM_MEMBERS`: 自定义成员
    #[serde(
        rename = "who_can_send_mail",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub who_can_send_mail: String,
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
        Fn(UpdateMailGroupReq) -> Result<(UpdateMailGroupResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(UpdateMailGroupReq) -> Result<(UpdateMailGroupResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_mail_group<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdateMailGroupReq, UpdateMailGroupResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_mail_group(
            &self,
            req: &UpdateMailGroupReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateMailGroupReq, UpdateMailGroupResp, Arc<dyn MockFunc>>(
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
        api::gen::mail::update_mail_group::{
            UpdateMailGroupReq, UpdateMailGroupResp, UpdateMailGroupRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_update_mail_group(|_| {
                Ok((UpdateMailGroupResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .mail()
            .update_mail_group(UpdateMailGroupReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .mail()
            .update_mail_group(UpdateMailGroupReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{ 
   "email": "xxx@xxx.com",
    "name": "xxx",
    "description": "xxx",
    "who_can_send_mail": "ANYONE"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateMailGroupReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "mailgroup_id": "xxx",
        "email": "xx@xx.xx",
        "name":"xxx",
        "description":"xxx",
        "direct_members_count":"x",
        "include_external_member": true,
        "include_all_company_member":false,
        "who_can_send_mail":"ANYONE"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateMailGroupRespInner>(RESP);
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
