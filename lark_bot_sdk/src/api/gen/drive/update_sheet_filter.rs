//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/update>
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
    /// **api 版本: 2024-07-31T09:17:17+00:00**
    ///
    /// ## 更新筛选
    ///
    /// 在电子表格工作表筛选范围中，更新指定列的筛选条件。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/update>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/update>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fsheets-v3%2Fspreadsheet-sheet-filter%2Fupdate>
    pub async fn update_sheet_filter(
        &self,
        req: UpdateSheetFilterReq,
    ) -> Result<(UpdateSheetFilterResp, CommonResponse), Error> {
        self.update_sheet_filter_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_sheet_filter](#method.update_sheet_filter) 函数
    pub async fn update_sheet_filter_with_opt(
        &self,
        req: UpdateSheetFilterReq,
        method_option: MethodOption,
    ) -> Result<(UpdateSheetFilterResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_sheet_filter(&req) {
                tracing::info!("[lark] Drive#UpdateSheetFilter **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#UpdateSheetFilter call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "UpdateSheetFilter",
            method: http::Method::PUT,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateSheetFilterRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateSheetFilterReq {
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

    /// 指定要更新筛选条件的列。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "G"
    #[api(kind = "body", name = "col")]
    pub col: String,
    /// 设置筛选条件。
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "condition")]
    pub condition: ConditionSubReq,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ConditionSubReq {
    /// 筛选类型，枚举值如下所示。了解更多，参考[筛选指南](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/filter-user-guide)。
    ///
    /// - multiValue ：多值筛选
    ///
    /// - number ：数字筛选
    ///
    /// - text ：文本筛选
    ///
    /// - color ：颜色筛选
    ///
    /// - clear ：清除某列的筛选条件
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "text"
    #[serde(
        rename = "filter_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub filter_type: String,
    /// 比较类型。不同筛选类型的比较类型的枚举值不同，详情参考[筛选指南](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/filter-user-guide)。
    ///
    /// **示例值**: "beginsWith"
    #[serde(
        rename = "compare_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub compare_type: Option<String>,
    /// 筛选参数。不同筛选类型的筛选参数限制不同，详情参考[筛选指南](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/filter-user-guide)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6"
    #[serde(
        rename = "expected",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expected: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateSheetFilterRespInner {
    #[serde(flatten)]
    data: Option<UpdateSheetFilterResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSheetFilterResp {
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

    use self::gen::drive::DriveServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(UpdateSheetFilterReq) -> Result<(UpdateSheetFilterResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(UpdateSheetFilterReq) -> Result<(UpdateSheetFilterResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_sheet_filter<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdateSheetFilterReq, UpdateSheetFilterResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_sheet_filter(
            &self,
            req: &UpdateSheetFilterReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateSheetFilterReq, UpdateSheetFilterResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::update_sheet_filter::{
            UpdateSheetFilterReq, UpdateSheetFilterResp, UpdateSheetFilterRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_update_sheet_filter(|_| {
                Ok((UpdateSheetFilterResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .drive()
            .update_sheet_filter(UpdateSheetFilterReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .update_sheet_filter(UpdateSheetFilterReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "col": "G",
    "condition": {
        "filter_type": "text",
        "compare_type": "beginsWith",
        "expected": [
            "a"
        ]
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateSheetFilterReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateSheetFilterRespInner>(RESP);
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