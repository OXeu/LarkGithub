//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/patch>
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
    /// **api 版本: 2024-01-11T02:48:42+00:00**
    ///
    /// ## 更新视图
    ///
    /// 该接口用于增量修改视图信息
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/bitable-v1/app-table-view/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fbitable-v1%2Fapp-table-view%2Fpatch>
    pub async fn update_bitable_view(
        &self,
        req: UpdateBitableViewReq,
    ) -> Result<(UpdateBitableViewResp, CommonResponse), Error> {
        self.update_bitable_view_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_bitable_view](#method.update_bitable_view) 函数
    pub async fn update_bitable_view_with_opt(
        &self,
        req: UpdateBitableViewReq,
        method_option: MethodOption,
    ) -> Result<(UpdateBitableViewResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_bitable_view(&req) {
                tracing::info!("[lark] Bitable#UpdateBitableView **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Bitable#UpdateBitableView call api");

        let req = ApiRequest {
            scope: "Bitable",
            api: "UpdateBitableView",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateBitableViewRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateBitableViewReq {
    /// base app token
    ///
    /// **示例值**: "bascng7vrxcxpig7geggXiCtadY"
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
    /// 视图 ID
    ///
    /// **示例值**: "vewTpR1urY"
    #[api(kind = "path", name = "view_id")]
    pub view_id: String,

    /// 视图名称
    ///
    /// **示例值**: "grid"
    #[api(kind = "body", name = "view_name")]
    pub view_name: Option<String>,
    /// 视图属性
    #[api(kind = "body", name = "property")]
    pub property: Option<AppTableViewPropertySubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppTableViewPropertySubReq {
    /// 过滤条件
    #[serde(
        rename = "filter_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub filter_info: Option<AppTableViewPropertyFilterInfoSubReq>,
    /// 隐藏字段ID列表
    ///
    /// **示例值**: "["fldCGzANXx", "fldCGzANXx"]"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `300` 字符
    #[serde(
        rename = "hidden_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hidden_fields: Vec<Option<String>>,
    /// 表格视图层级结构设置
    #[serde(
        rename = "hierarchy_config",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hierarchy_config: Option<AppTableViewPropertyHierarchyConfigSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppTableViewPropertyFilterInfoSubReq {
    /// 多个筛选条件的关系
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "and"
    ///
    /// **可选值**:
    ///
    /// `And`: 与
    ///
    /// `Or`: 或
    #[serde(
        rename = "conjunction",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub conjunction: String,
    /// 筛选条件
    ///
    /// **是否必填**: 是
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `50` 字符
    #[serde(
        rename = "conditions",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub conditions: Vec<Option<AppTableViewPropertyFilterInfoConditionSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppTableViewPropertyHierarchyConfigSubReq {
    /// 层级结构的关联列id
    ///
    /// **示例值**: "fldTca**hb"
    #[serde(
        rename = "field_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub field_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppTableViewPropertyFilterInfoConditionSubReq {
    /// 用于过滤的字段唯一ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "fldVioU**1"
    #[serde(
        rename = "field_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub field_id: String,
    /// 过滤操作的类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "is"
    ///
    /// **可选值**:
    ///
    /// `Is`: 等于
    ///
    /// `IsNot`: 不等于
    ///
    /// `Contains`: 包含
    ///
    /// `DoesNotContain`: 不包含
    ///
    /// `IsEmpty`: 为空
    ///
    /// `IsNotEmpty`: 不为空
    ///
    /// `IsGreater`: 大于
    ///
    /// `IsGreaterEqual`: 大于等于
    ///
    /// `IsLess`: 小于
    ///
    /// `IsLessEqual`: 小于等于
    #[serde(
        rename = "operator",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator: String,
    /// 筛选值
    ///
    /// **示例值**: "["optbdVH***", "optrpd3***"]"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateBitableViewRespInner {
    #[serde(flatten)]
    data: Option<UpdateBitableViewResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateBitableViewResp {
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
        rename = "view",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub view: AppTableViewSubResp,
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
    /// 视图属性
    #[serde(
        rename = "property",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub property: AppTableViewPropertySubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppTableViewPropertySubResp {
    /// 过滤条件
    #[serde(
        rename = "filter_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub filter_info: AppTableViewPropertyFilterInfoSubResp,
    /// 隐藏字段ID列表
    ///
    /// **示例值**: "["fldCGzANXx", "fldCGzANXx"]"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `300` 字符
    #[serde(
        rename = "hidden_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hidden_fields: Vec<String>,
    /// 表格视图层级结构设置
    #[serde(
        rename = "hierarchy_config",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hierarchy_config: AppTableViewPropertyHierarchyConfigSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppTableViewPropertyFilterInfoSubResp {
    /// 多个筛选条件的关系
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "and"
    ///
    /// **可选值**:
    ///
    /// `And`: 与
    ///
    /// `Or`: 或
    #[serde(
        rename = "conjunction",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub conjunction: String,
    /// 筛选条件
    ///
    /// **是否必填**: 是
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `50` 字符
    #[serde(
        rename = "conditions",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub conditions: Vec<AppTableViewPropertyFilterInfoConditionSubResp>,
    /// 筛选条件是否缺省
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "condition_omitted",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub condition_omitted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppTableViewPropertyHierarchyConfigSubResp {
    /// 层级结构的关联列id
    ///
    /// **示例值**: "fldTca**hb"
    #[serde(
        rename = "field_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub field_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppTableViewPropertyFilterInfoConditionSubResp {
    /// 用于过滤的字段唯一ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "fldVioU**1"
    #[serde(
        rename = "field_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub field_id: String,
    /// 过滤操作的类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "is"
    ///
    /// **可选值**:
    ///
    /// `Is`: 等于
    ///
    /// `IsNot`: 不等于
    ///
    /// `Contains`: 包含
    ///
    /// `DoesNotContain`: 不包含
    ///
    /// `IsEmpty`: 为空
    ///
    /// `IsNotEmpty`: 不为空
    ///
    /// `IsGreater`: 大于
    ///
    /// `IsGreaterEqual`: 大于等于
    ///
    /// `IsLess`: 小于
    ///
    /// `IsLessEqual`: 小于等于
    #[serde(
        rename = "operator",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator: String,
    /// 筛选值
    ///
    /// **示例值**: "["optbdVH***", "optrpd3***"]"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
    /// 过滤条件的唯一ID
    ///
    /// **示例值**: "conNaOEK6O"
    #[serde(
        rename = "condition_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub condition_id: String,
    /// 用于过滤的字段类型<br>1：多行文本<br>2：数字<br>3：单选<br>4：多选<br>5：日期<br>7：复选框<br>11：人员<br>13：电话号码<br>15：超链接<br>17：附件<br>18：单向关联<br>19：查找引用<br>20：公式<br>21：双向关联<br>22：地理位置<br>23：群组<br>1001：创建时间<br>1002：最后更新时间<br>1003：创建人<br>1004：修改人<br>1005：自动编号
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "field_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub field_type: i64,
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
        Fn(UpdateBitableViewReq) -> Result<(UpdateBitableViewResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(UpdateBitableViewReq) -> Result<(UpdateBitableViewResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> BitableServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_bitable_view<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdateBitableViewReq, UpdateBitableViewResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_bitable_view(
            &self,
            req: &UpdateBitableViewReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateBitableViewReq, UpdateBitableViewResp, Arc<dyn MockFunc>>(
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
        api::gen::bitable::update_bitable_view::{
            UpdateBitableViewReq, UpdateBitableViewResp, UpdateBitableViewRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .bitable()
            .mock()
            .mock_update_bitable_view(|_| {
                Ok((UpdateBitableViewResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .bitable()
            .update_bitable_view(UpdateBitableViewReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .bitable()
            .update_bitable_view(UpdateBitableViewReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "view_name": "grid",
    "property": {
        "filter_info": {
            "conditions": [
                {
                    "field_id": "fldVioU**1",
                    "operator": "is",
                    "value": "[\"text content\"]"
                }
            ],
            "conjunction": "and"
        },
        "hidden_fields": null
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateBitableViewReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "view": {
            "view_id": "vewsOleexJ",
            "view_name": "grid",
            "view_type": "grid",
            "property": {
                "filter_info": {
                    "condition_omitted": null,
                    "conditions": [
                        {
                            "condition_id": "conuKMQNNg",
                            "field_id": "fldVioU**1",
                            "field_type": 1,
                            "operator": "is",
                            "value": "[\"text content\"]"
                        }
                    ],
                    "conjunction": "and"
                },
                "hidden_fields": null
            }
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateBitableViewRespInner>(RESP);
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
