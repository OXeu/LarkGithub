//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::drive::DriveService;

impl<'c, IStore: Store, IClient: HttpClient> DriveService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-31T09:17:12+00:00**
    ///
    /// ## 创建电子表格
    ///
    /// 在云空间指定目录下创建电子表格。可自定义表格标题。不支持带内容创建表格。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/sheets-v3/spreadsheet/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fsheets-v3%2Fspreadsheet%2Fcreate>
    pub async fn create_spreadsheet(
        &self,
        req: CreateSpreadsheetReq,
    ) -> Result<(CreateSpreadsheetResp, CommonResponse), Error> {
        self.create_spreadsheet_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_spreadsheet](#method.create_spreadsheet) 函数
    pub async fn create_spreadsheet_with_opt(
        &self,
        req: CreateSpreadsheetReq,
        method_option: MethodOption,
    ) -> Result<(CreateSpreadsheetResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_spreadsheet(&req) {
                tracing::info!("[lark] Drive#CreateSpreadsheet **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#CreateSpreadsheet call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "CreateSpreadsheet",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/sheets/v3/spreadsheets",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateSpreadsheetRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateSpreadsheetReq {
    /// 表格标题
    ///
    /// **示例值**: "Sales sheet"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `0` 字符- `255` 字符
    #[api(kind = "body", name = "title")]
    pub title: Option<String>,
    /// 文件夹 token。你可通过以下两种方式获取文件夹的 token：
    ///
    /// - 文件夹的 URL：https://sample.feishu.cn/drive/folder/==fldbcO1UuPz8VwnpPx5a92abcef==
    ///
    /// - 调用开放平台接口获取：
    ///
    /// - 调用[获取我的空间（root folder）元数据](https://open.feishu.cn/document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/get-root-folder-meta)接口获取根目录（即根文件夹）的 token。
    ///
    /// - 继续调用[获取文件夹中的文件清单](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/list)接口，获取根目录下文件夹的 token。
    ///
    /// **示例值**: "fldbcO1UuPz8VwnpPx5a92abcef"
    #[api(kind = "body", name = "folder_token")]
    pub folder_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateSpreadsheetRespInner {
    #[serde(flatten)]
    data: Option<CreateSpreadsheetResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateSpreadsheetResp {
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
    /// 电子表格的基础信息
    #[serde(
        rename = "spreadsheet",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub spreadsheet: SpreadsheetSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SpreadsheetSubResp {
    /// 电子表格标题
    ///
    /// **示例值**: "Sales sheet"
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `0` 字符- `255` 字符
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: String,
    /// 文件夹 token
    ///
    /// **示例值**: "fldbcO1UuPz8VwnpPx5a92abcef"
    #[serde(
        rename = "folder_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub folder_token: String,
    /// 电子表格的 URL 链接
    ///
    /// **示例值**: "https://example.feishu.cn/sheets/Iow7sNNEphp3WbtnbCscPqabcef"
    #[serde(
        rename = "url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub url: String,
    /// 电子表格 token
    ///
    /// **示例值**: "Iow7sNNEphp3WbtnbCscPqabcef"
    #[serde(
        rename = "spreadsheet_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub spreadsheet_token: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::drive::DriveServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(CreateSpreadsheetReq) -> Result<(CreateSpreadsheetResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(CreateSpreadsheetReq) -> Result<(CreateSpreadsheetResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_spreadsheet<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, CreateSpreadsheetReq, CreateSpreadsheetResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_spreadsheet(
            &self,
            req: &CreateSpreadsheetReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateSpreadsheetReq, CreateSpreadsheetResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::create_spreadsheet::{
            CreateSpreadsheetReq, CreateSpreadsheetResp, CreateSpreadsheetRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_create_spreadsheet(|_| {
                Ok((CreateSpreadsheetResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .drive()
            .create_spreadsheet(CreateSpreadsheetReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .create_spreadsheet(CreateSpreadsheetReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "title": "Sales sheet",
    "folder_token": "fldbcO1UuPz8VwnpPx5a92abcef"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateSpreadsheetReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "spreadsheet": {
            "title": "Sales sheet",
            "folder_token": "fldbcO1UuPz8VwnpPx5a92abcef",
            "url": "https://example.feishu.cn/sheets/Iow7sNNEphp3WbtnbCscPqabcef",
            "spreadsheet_token": "Iow7sNNEphp3WbtnbCscPqabcef"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateSpreadsheetRespInner>(RESP);
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
