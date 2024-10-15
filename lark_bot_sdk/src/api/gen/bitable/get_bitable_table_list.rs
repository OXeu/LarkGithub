//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/list>
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
    /// **api 版本: 2023-08-03T07:18:14+00:00**
    ///
    /// ## 列出数据表
    ///
    /// 根据  app_token，获取多维表格下的所有数据表。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/bitable-v1/app-table/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fbitable-v1%2Fapp-table%2Flist>
    pub async fn get_bitable_table_list(
        &self,
        req: GetBitableTableListReq,
    ) -> Result<(GetBitableTableListResp, CommonResponse), Error> {
        self.get_bitable_table_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_bitable_table_list](#method.get_bitable_table_list) 函数
    pub async fn get_bitable_table_list_with_opt(
        &self,
        req: GetBitableTableListReq,
        method_option: MethodOption,
    ) -> Result<(GetBitableTableListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_bitable_table_list(&req) {
                tracing::info!("[lark] Bitable#GetBitableTableList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Bitable#GetBitableTableList call api");

        let req = ApiRequest {
            scope: "Bitable",
            api: "GetBitableTableList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/bitable/v1/apps/:app_token/tables",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetBitableTableListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetBitableTableListReq {
    /// 多维表格的唯一标识符 [app_token 参数说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bitable/notification#8121eebe)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "appbcbWCzen6D8dezhoCH2RpMAh"
    #[api(kind = "path", name = "app_token")]
    pub app_token: String,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "tblsRc9GRRXKqhvW"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 分页大小
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetBitableTableListRespInner {
    #[serde(flatten)]
    data: Option<GetBitableTableListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetBitableTableListResp {
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
    /// **示例值**: "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 总数
    ///
    /// **示例值**: "\-"
    #[serde(
        rename = "total",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub total: i64,
    /// 数据表信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<AppTableSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppTableSubResp {
    /// 数据表 id
    ///
    /// **示例值**: "\-"
    #[serde(
        rename = "table_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub table_id: String,
    /// 数据表的版本号
    ///
    /// **示例值**: "\-"
    #[serde(
        rename = "revision",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub revision: i64,
    /// 数据表名字
    ///
    /// **示例值**: "\-"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
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
        Fn(GetBitableTableListReq) -> Result<(GetBitableTableListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetBitableTableListReq,
                ) -> Result<(GetBitableTableListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> BitableServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_bitable_table_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetBitableTableListReq, GetBitableTableListResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_bitable_table_list(
            &self,
            req: &GetBitableTableListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetBitableTableListReq, GetBitableTableListResp, Arc<dyn MockFunc>>(
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
        api::gen::bitable::get_bitable_table_list::{
            GetBitableTableListReq, GetBitableTableListResp, GetBitableTableListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .bitable()
            .mock()
            .mock_get_bitable_table_list(|_| {
                Ok((
                    GetBitableTableListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .bitable()
            .get_bitable_table_list(GetBitableTableListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .bitable()
            .get_bitable_table_list(GetBitableTableListReq::default())
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
        "page_token": "tblKz5D60T4JlfcT",
        "total": 1,
        "items": [
            {
                "table_id": "tblKz5D60T4JlfcT",
                "revision": 1,
                "name": "数据表1"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetBitableTableListRespInner>(RESP);
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