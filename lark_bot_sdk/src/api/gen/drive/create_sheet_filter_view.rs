//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/create>
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
    /// **api 版本: 2024-07-31T09:17:18+00:00**
    ///
    /// ## 创建筛选视图
    ///
    /// 指定电子表格工作表的筛选范围，创建一个筛选视图。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/create>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/create>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fsheets-v3%2Fspreadsheet-sheet-filter_view%2Fcreate>
    pub async fn create_sheet_filter_view(
        &self,
        req: CreateSheetFilterViewReq,
    ) -> Result<(CreateSheetFilterViewResp, CommonResponse), Error> {
        self.create_sheet_filter_view_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_sheet_filter_view](#method.create_sheet_filter_view) 函数
    pub async fn create_sheet_filter_view_with_opt(
        &self,
        req: CreateSheetFilterViewReq,
        method_option: MethodOption,
    ) -> Result<(CreateSheetFilterViewResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_sheet_filter_view(&req) {
                tracing::info!("[lark] Drive#CreateSheetFilterView **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#CreateSheetFilterView call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "CreateSheetFilterView",
            method: http::Method::POST,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateSheetFilterViewRespInner, _) = self.cli.do_req(req).await?;
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
pub struct CreateSheetFilterViewReq {
    /// 电子表格的 token。可通过以下两种方式获取。了解更多，参考[电子表格概述](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/overview)。
    ///
    /// - 电子表格的 URL：https://sample.feishu.cn/sheets/==Iow7sNNEphp3WbtnbCscPqabcef==
    ///
    /// - 调用[获取文件夹中的文件清单](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/list)
    ///
    /// **示例值**: "Iow7sNNEphp3WbtnbCscPqabcef"
    #[api(kind = "path", name = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表 ID，通过[获取工作表](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/query) 获取。
    ///
    /// **示例值**: "8fe9d6"
    #[api(kind = "path", name = "sheet_id")]
    pub sheet_id: String,

    /// 自定义筛选视图 ID。不填将由系统生成。长度需为 10 字符。字符可以是 0-9 数字、a-z、A-Z 字母或其组合。
    ///
    /// **示例值**: "pH9hbVcCXA"
    #[api(kind = "body", name = "filter_view_id")]
    pub filter_view_id: Option<String>,
    /// 自定义筛选视图名称。不填将由系统生成。长度不超过100 个字符。
    ///
    /// **示例值**: "筛选视图 1"
    #[api(kind = "body", name = "filter_view_name")]
    pub filter_view_name: Option<String>,
    /// 筛选视图的筛选范围。该参数必填，请忽略必填列的“否”。支持以下五种写法，了解更多，参考[筛选指南](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/filter-user-guide)。
    ///
    /// - `sheetId`：填写实际的工作表 ID，表示将筛选应用于整表
    ///
    /// - `sheetId!{开始行索引}:{结束行索引}`：填写工作表 ID 和行数区间，表示将筛选应用于整行
    ///
    /// - `sheetId!{开始列索引}:{结束列索引}`：填写工作表 ID 和列的区间，表示将筛选应用于整列
    ///
    /// - `sheetId!{开始单元格}:{结束单元格}`：填写工作表 ID 和单元格区间，表示将筛选应用于单元格选定的区域中
    ///
    /// - `sheetId!{开始单元格}:{结束列索引}`：填写工作表 ID、起始单元格和结束列，表示省略结束行，使用表格的最后行作为结束行
    ///
    /// **示例值**: "8fe9d6!C1:H14"
    #[api(kind = "body", name = "range")]
    pub range: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateSheetFilterViewRespInner {
    #[serde(flatten)]
    data: Option<CreateSheetFilterViewResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateSheetFilterViewResp {
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
    /// 创建的筛选视图的信息
    #[serde(
        rename = "filter_view",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub filter_view: FilterViewSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FilterViewSubResp {
    /// 筛选视图 ID
    ///
    /// **示例值**: "pH9hbVcCXA"
    #[serde(
        rename = "filter_view_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub filter_view_id: String,
    /// 筛选视图名称
    ///
    /// **示例值**: "筛选视图 1"
    #[serde(
        rename = "filter_view_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub filter_view_name: String,
    /// 筛选视图的筛选范围
    ///
    /// **示例值**: "8fe9d6!C1:H14"
    #[serde(
        rename = "range",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub range: String,
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
        Fn(CreateSheetFilterViewReq) -> Result<(CreateSheetFilterViewResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    CreateSheetFilterViewReq,
                ) -> Result<(CreateSheetFilterViewResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_sheet_filter_view<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            CreateSheetFilterViewReq,
            CreateSheetFilterViewResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_sheet_filter_view(
            &self,
            req: &CreateSheetFilterViewReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, CreateSheetFilterViewReq, CreateSheetFilterViewResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::create_sheet_filter_view::{
            CreateSheetFilterViewReq, CreateSheetFilterViewResp, CreateSheetFilterViewRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_create_sheet_filter_view(|_| {
                Ok((
                    CreateSheetFilterViewResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .create_sheet_filter_view(CreateSheetFilterViewReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .create_sheet_filter_view(CreateSheetFilterViewReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "filter_view_id": "pH9hbVcCXA",
    "filter_view_name": "筛选视图 1",
    "range": "8fe9d6!C1:H14"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateSheetFilterViewReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "filter_view": {
            "filter_view_id": "pH9hbVcCXA",
            "filter_view_name": "筛选视图 1",
            "range": "8fe9d6!C1:H14"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateSheetFilterViewRespInner>(RESP);
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