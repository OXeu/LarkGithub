//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/member_belong>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::contact::ContactService;

impl<'c, IStore: Store, IClient: HttpClient> ContactService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-05T08:08:03+00:00**
    ///
    /// ## 查询用户所属用户组
    ///
    /// 调用该接口查询指定用户所属的用户组列表。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/member_belong>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/contact-v3/group/member_belong>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcontact-v3%2Fgroup%2Fmember_belong>
    pub async fn get_contact_member_group_list(
        &self,
        req: GetContactMemberGroupListReq,
    ) -> Result<(GetContactMemberGroupListResp, CommonResponse), Error> {
        self.get_contact_member_group_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_contact_member_group_list](#method.get_contact_member_group_list) 函数
    pub async fn get_contact_member_group_list_with_opt(
        &self,
        req: GetContactMemberGroupListReq,
        method_option: MethodOption,
    ) -> Result<(GetContactMemberGroupListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_contact_member_group_list(&req) {
                tracing::info!("[lark] Contact#GetContactMemberGroupList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#GetContactMemberGroupList call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "GetContactMemberGroupList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/group/member_belong",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetContactMemberGroupListRespInner, _) =
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
pub struct GetContactMemberGroupListReq {
    /// 成员 ID。ID 类型与 member_id_type 取值保持一致。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "u287xj12"
    #[api(kind = "query", name = "member_id", v_type = "var", option = "false")]
    pub member_id: String,
    /// 成员 ID 类型。
    ///
    /// **示例值**: "open_id"
    ///
    /// **可选值**:
    ///
    /// `OpenID`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)。
    ///
    /// `UnionID`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)。
    ///
    /// `UserID`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)。
    #[api(
        kind = "query",
        name = "member_id_type",
        v_type = "var",
        option = "false"
    )]
    pub member_id_type: String,
    /// 用户组类型。
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Assign`: 普通用户组
    ///
    /// `Dynamic`: 动态用户组
    #[api(kind = "query", name = "group_type", v_type = "var", option = "false")]
    pub group_type: i64,
    /// 分页大小，用于限制一次请求所返回的数据条目数。
    ///
    /// **示例值**: "500"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "AQD9/Rn9eij9Pm39ED40/dk53s4Ebp882DYfFaPFbz00L4CMZJrqGdzNyc8BcZtDbwVUvRmQTvyMYicnGWrde9X56TgdBuS+JKiSIkdexPw="
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetContactMemberGroupListRespInner {
    #[serde(flatten)]
    data: Option<GetContactMemberGroupListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetContactMemberGroupListResp {
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
    /// 用户组 ID 列表。
    ///
    /// **说明**：你可以调用[查询指定用户组](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/get)接口，通过用户组 ID 获取用户组的详细信息。
    #[serde(
        rename = "group_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_list: Vec<String>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "AQD9/Rn9eij9Pm39ED40/dk53s4Ebp882DYfFaPFbz00L4CMZJrqGdzNyc8BcZtDbwVUvRmQTvyMYicnGWrde9X56TgdBuS+JKiSIkdexPw="
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 是否还有更多项
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::contact::ContactServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            GetContactMemberGroupListReq,
        ) -> Result<(GetContactMemberGroupListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetContactMemberGroupListReq,
                )
                    -> Result<(GetContactMemberGroupListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_contact_member_group_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetContactMemberGroupListReq,
            GetContactMemberGroupListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_contact_member_group_list(
            &self,
            req: &GetContactMemberGroupListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetContactMemberGroupListReq,
                GetContactMemberGroupListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::contact::get_contact_member_group_list::{
            GetContactMemberGroupListReq, GetContactMemberGroupListResp,
            GetContactMemberGroupListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_get_contact_member_group_list(|_| {
                Ok((
                    GetContactMemberGroupListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .contact()
            .get_contact_member_group_list(GetContactMemberGroupListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .get_contact_member_group_list(GetContactMemberGroupListReq::default())
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
        "group_list": [
            "og-1455998e138698e1386"
        ],
        "page_token": "AQD9/Rn9eij9Pm39ED40/dk53s4Ebp882DYfFaPFbz00L4CMZJrqGdzNyc8BcZtDbwVUvRmQTvyMYicnGWrde9X56TgdBuS+JKiSIkdexPw=",
        "has_more": false
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetContactMemberGroupListRespInner>(RESP);
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