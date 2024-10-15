//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup/list>
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
    /// ## 批量获取邮件组
    ///
    /// 分页批量获取邮件组
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/mail-v1/mail-group/mailgroup/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fmail-v1%2Fmail-group%2Fmailgroup%2Flist>
    pub async fn get_mail_group_list(
        &self,
        req: GetMailGroupListReq,
    ) -> Result<(GetMailGroupListResp, CommonResponse), Error> {
        self.get_mail_group_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_mail_group_list](#method.get_mail_group_list) 函数
    pub async fn get_mail_group_list_with_opt(
        &self,
        req: GetMailGroupListReq,
        method_option: MethodOption,
    ) -> Result<(GetMailGroupListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_mail_group_list(&req) {
                tracing::info!("[lark] Mail#GetMailGroupList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Mail#GetMailGroupList call api");

        let req = ApiRequest {
            scope: "Mail",
            api: "GetMailGroupList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/mail/v1/mailgroups",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetMailGroupListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetMailGroupListReq {
    /// 邮件组管理员用户ID，用于获取该用户有管理权限的邮件组
    ///
    /// **示例值**: "ou_xxxxxx"
    #[api(
        kind = "query",
        name = "manager_user_id",
        v_type = "var",
        option = "false"
    )]
    pub manager_user_id: String,
    /// 用户 ID 类型
    ///
    /// **示例值**: "open_id"
    ///
    /// **可选值**:
    ///
    /// `open_id`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// `union_id`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)
    ///
    /// `user_id`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
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
struct GetMailGroupListRespInner {
    #[serde(flatten)]
    data: Option<GetMailGroupListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetMailGroupListResp {
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
    /// 邮件组列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<MailgroupSubResp>,
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
        Fn(GetMailGroupListReq) -> Result<(GetMailGroupListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetMailGroupListReq) -> Result<(GetMailGroupListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MailServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_mail_group_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetMailGroupListReq, GetMailGroupListResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_mail_group_list(
            &self,
            req: &GetMailGroupListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetMailGroupListReq, GetMailGroupListResp, Arc<dyn MockFunc>>(
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
        api::gen::mail::get_mail_group_list::{
            GetMailGroupListReq, GetMailGroupListResp, GetMailGroupListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .mail()
            .mock()
            .mock_get_mail_group_list(|_| {
                Ok((GetMailGroupListResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .mail()
            .get_mail_group_list(GetMailGroupListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .mail()
            .get_mail_group_list(GetMailGroupListReq::default())
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
                "mailgroup_id": "xxxxxxxxxxxxxxx",
                "email": "test_mail_group@xxx.xx",
                "name": "test mail group",
                "description": "mail group for testing",
                "direct_members_count": "10",
                "include_external_member": true,
                "include_all_company_member": false,
                "who_can_send_mail": "ALL_INTERNAL_USERS"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetMailGroupListRespInner>(RESP);
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