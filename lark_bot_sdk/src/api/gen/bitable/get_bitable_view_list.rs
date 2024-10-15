//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/list>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::bitable::BitableService;

impl<'c, IStore: Store, IClient: HttpClient> BitableService<'c, IStore, IClient> {
    /// **api 版本: 2023-07-28T08:21:51+00:00**
    ///
    /// ## 列出视图
    ///
    /// 根据 app_token 和 table_id，获取数据表的所有视图
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/bitable-v1/app-table-view/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fbitable-v1%2Fapp-table-view%2Flist>
    pub async fn get_bitable_view_list(
        &self,
        req: GetBitableViewListReq,
    ) -> Result<(GetBitableViewListResp, CommonResponse), Error> {
        self.get_bitable_view_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_bitable_view_list](#method.get_bitable_view_list) 函数
    pub async fn get_bitable_view_list_with_opt(
        &self,
        req: GetBitableViewListReq,
        method_option: MethodOption,
    ) -> Result<(GetBitableViewListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_bitable_view_list(&req) {
                tracing::info!("[lark] Bitable#GetBitableViewList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Bitable#GetBitableViewList call api");

        let req = ApiRequest {
            scope: "Bitable",
            api: "GetBitableViewList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetBitableViewListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetBitableViewListReq {
    /// base app token
    ///
    /// **示例值**: "appbcbWCzen6D8dezhoCH2RpMAh"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[api(kind = "path", name = "app_token")]
    pub app_token: String,
    /// table id
    ///
    /// **示例值**: "tblsRc9GRRXKqhvW"
    #[api(kind = "path", name = "table_id")]
    pub table_id: String,
    /// 分页大小
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "vewTpR1urY"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
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
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetBitableViewListRespInner {
    #[serde(flatten)]
    data: Option<GetBitableViewListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetBitableViewListResp {
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
    /// 视图信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<AppTableViewSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 总数
    ///
    /// **示例值**: "20"
    #[serde(
        rename = "total",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppTableViewSubResp {
    /// 视图Id
    ///
    /// **示例值**: "vewieWxfON"
    #[serde(
        rename = "view_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub view_id: String,
    /// 视图名字
    ///
    /// **示例值**: "表格 1"
    #[serde(
        rename = "view_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub view_name: String,
    /// 视图类型
    ///
    /// **示例值**: "grid"
    #[serde(
        rename = "view_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub view_type: String,
    /// 视图公共等级 Public、Locked、Private
    ///
    /// **示例值**: "Public"
    ///
    /// **可选值**:
    ///
    /// `Public`: 公共视图
    ///
    /// `Locked`: 锁定视图
    ///
    /// `Private`: 个人视图
    #[serde(
        rename = "view_public_level",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub view_public_level: String,
    /// 个人视图的owner_id，id类型和 user_id_type 参数保持一致
    ///
    /// **示例值**: "ou_2910013f1e6456f16a0ce75ede950a0a"
    #[serde(
        rename = "view_private_owner_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub view_private_owner_id: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::bitable::BitableServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetBitableViewListReq) -> Result<(GetBitableViewListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetBitableViewListReq) -> Result<(GetBitableViewListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> BitableServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_bitable_view_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetBitableViewListReq, GetBitableViewListResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_bitable_view_list(
            &self,
            req: &GetBitableViewListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetBitableViewListReq, GetBitableViewListResp, Arc<dyn MockFunc>>(
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
        api::gen::bitable::get_bitable_view_list::{
            GetBitableViewListReq, GetBitableViewListResp, GetBitableViewListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .bitable()
            .mock()
            .mock_get_bitable_view_list(|_| {
                Ok((GetBitableViewListResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .bitable()
            .get_bitable_view_list(GetBitableViewListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .bitable()
            .get_bitable_view_list(GetBitableViewListReq::default())
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
		"has_more": false,
		"items": [{
				"view_id": "vewqtI3f2u",
				"view_name": "公共表格视图",
				"view_public_level": "Public",
				"view_type": "grid"
			},
			{
				"view_id": "vew5Ys1Y1B",
				"view_name": "个人表格视图",
				"view_private_owner_id": "ou_fe4e2a0c10f41fb85620eb4b71d12082",
				"view_public_level": "Private",
				"view_type": "grid"
			}
		],
		"page_token": "vew5Ys1Y1B",
		"total": 2
	}
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetBitableViewListRespInner>(RESP);
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