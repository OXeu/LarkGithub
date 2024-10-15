//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/list>
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
    /// **api 版本: 2024-08-02T07:13:25+00:00**
    ///
    /// ## 获取部门信息列表
    ///
    /// 该接口用于获取当前部门子部门列表。[常见问题答疑](https://open.feishu.cn/document/ugTN1YjL4UTN24CO1UjN/uQzN1YjL0cTN24CN3UjN)。
    ///
    /// - 使用 user_access_token 时，返回该用户组织架构可见性范围（[登陆企业管理后台进行权限配置](https://www.feishu.cn/admin/security/permission/visibility)）内的所有可见部门。当进行递归查询时，只筛查最多1000个部门的可见性。
    ///
    /// - 使用
    ///
    /// tenant_access_token 则基于应用的通讯录权限范围进行权限校验与过滤。由于
    ///
    /// parent_department_id 是非必填参数，填与不填存在<b>两种数据权限校验与返回</b>情况：
    ///
    /// <br> <br>1、请求设置了
    ///
    /// parent_department_id 为A（根部门0），会检验A是否在通讯录权限内，若在( parent_department_id=0 时会校验是否为全员权限），则返回部门下子部门列表（根据fetch_child决定是否递归），否则返回无部门通讯录权限错误码。
    ///
    /// <br> <br>2、请求未带
    ///
    /// parent_department_id 参数，如通讯录范围为全员权限，只返回根部门ID(部门ID为0)，否则返回根据通讯录范围配置的部门ID及子部门(根据
    ///
    /// fetch_child 决定是否递归)。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/historic-version//department/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhistoric-version%2F%2Fdepartment%2Flist>
    pub async fn get_department_list_old(
        &self,
        req: GetDepartmentListOldReq,
    ) -> Result<(GetDepartmentListOldResp, CommonResponse), Error> {
        self.get_department_list_old_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_department_list_old](#method.get_department_list_old) 函数
    pub async fn get_department_list_old_with_opt(
        &self,
        req: GetDepartmentListOldReq,
        method_option: MethodOption,
    ) -> Result<(GetDepartmentListOldResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_department_list_old(&req) {
                tracing::info!("[lark] Contact#GetDepartmentListOld **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Contact#GetDepartmentListOld call api");

        let req = ApiRequest {
            scope: "Contact",
            api: "GetDepartmentListOld",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/contact/v3/departments",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetDepartmentListOldRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetDepartmentListOldReq {
    /// 此次调用中使用的用户ID的类型
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
    /// 此次调用中使用的部门ID的类型
    ///
    /// **示例值**: "open_department_id"
    ///
    /// **可选值**:
    ///
    /// `department_id`: 以自定义department_id来标识部门
    ///
    /// `open_department_id`: 以open_department_id来标识部门
    #[api(
        kind = "query",
        name = "department_id_type",
        v_type = "var",
        option = "false"
    )]
    pub department_id_type: String,
    /// 父部门的ID，填上获取部门下所有子部门，此处填写的 ID 必须是 department_id_type 指定的 ID。
    ///
    /// **示例值**: "od-4e6ac4d14bcd5071a37a39de902c7141"
    #[api(
        kind = "query",
        name = "parent_department_id",
        v_type = "var",
        option = "false"
    )]
    pub parent_department_id: String,
    /// 是否递归获取子部门
    ///
    /// **示例值**: "是否递归获取子部门，默认值：false"
    #[api(kind = "query", name = "fetch_child", v_type = "var", option = "false")]
    pub fetch_child: bool,
    /// **示例值**: "AQD9/Rn9eij9Pm39ED40/RD/cIFmu77WxpxPB/2oHfQLZ%2BG8JG6tK7%2BZnHiT7COhD2hMSICh/eBl7cpzU6JEC3J7COKNe4jrQ8ExwBCR"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetDepartmentListOldRespInner {
    #[serde(flatten)]
    data: Option<GetDepartmentListOldResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDepartmentListOldResp {
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: DataSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DataSubResp {
    /// 是否有下一页数据
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 下一页分页的token
    ///
    /// **示例值**: "AQD9/Rn9eij9Pm39ED40/RD/cIFmu77WxpxPB/2oHfQLZ%2BG8JG6tK7%2BZnHiT7COhD2hMSICh/eBl7cpzU6JEC3J7COKNe4jrQ8ExwBCR"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,

    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<DepartmentSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DepartmentSubResp {
    /// 部门名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "DemoName"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 国际化的部门名称
    #[serde(
        rename = "i18n_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_name: DepartmentI18nNameSubResp,
    /// 父部门的ID
    ///
    /// * 在根部门下创建新部门，该参数值为 “0”
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "D067"
    #[serde(
        rename = "parent_department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_department_id: String,
    /// 本部门的自定义部门ID
    ///
    /// 注意：除需要满足正则规则外，同时不能以`od-`开头
    ///
    /// **示例值**: "D096"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `64` 字符
    #[serde(
        rename = "department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_id: String,
    /// 部门的open_id，类型与通过请求的查询参数传入的department_id_type相同
    ///
    /// **示例值**: "od-4e6ac4d14bcd5071a37a39de902c7141"
    #[serde(
        rename = "open_department_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub open_department_id: String,
    /// 部门主管用户ID
    ///
    /// **示例值**: "ou_7dab8a3d3cdcc9da365777c7ad535d62"
    #[serde(
        rename = "leader_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub leader_user_id: String,
    /// 部门群ID
    ///
    /// **示例值**: "oc_5ad11d72b830411d72b836c20"
    #[serde(
        rename = "chat_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_id: String,
    /// 部门的排序，即部门在其同级部门的展示顺序
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub order: String,
    /// 部门单位自定义ID列表，当前只支持一个
    #[serde(
        rename = "unit_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub unit_ids: Vec<String>,
    /// 部门下用户的个数
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "member_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub member_count: i64,
    /// 部门状态
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: DepartmentStatusSubResp,
    /// 部门负责人
    #[serde(
        rename = "leaders",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub leaders: Vec<DepartmentLeaderSubResp>,
    /// 部门群雇员类型限制
    #[serde(
        rename = "group_chat_employee_types",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group_chat_employee_types: Vec<i64>,
    /// 部门HRBP
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `500` 字符
    #[serde(
        rename = "department_hrbps",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub department_hrbps: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DepartmentI18nNameSubResp {
    /// 部门的中文名
    ///
    /// **示例值**: "Demo名称"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 部门的日文名
    ///
    /// **示例值**: "デモ名"
    #[serde(
        rename = "ja_jp",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub ja_jp: String,
    /// 部门的英文名
    ///
    /// **示例值**: "Demo Name"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DepartmentStatusSubResp {
    /// 是否被删除
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "is_deleted",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DepartmentLeaderSubResp {
    /// 负责人类型
    ///
    /// **是否必填**: 是
    ///
    /// **可选值**:
    ///
    /// `main`: 主负责人
    ///
    /// `deputy`: 副负责人
    #[serde(
        rename = "leaderType",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub leader_type: i64,
    /// 负责人ID
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "leaderID",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub leader_id: String,
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
        Fn(GetDepartmentListOldReq) -> Result<(GetDepartmentListOldResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetDepartmentListOldReq,
                ) -> Result<(GetDepartmentListOldResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ContactServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_department_list_old<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetDepartmentListOldReq,
            GetDepartmentListOldResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_department_list_old(
            &self,
            req: &GetDepartmentListOldReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetDepartmentListOldReq, GetDepartmentListOldResp, Arc<dyn MockFunc>>(
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
        api::gen::contact::get_department_list_old::{
            GetDepartmentListOldReq, GetDepartmentListOldResp, GetDepartmentListOldRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .contact()
            .mock()
            .mock_get_department_list_old(|_| {
                Ok((
                    GetDepartmentListOldResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .contact()
            .get_department_list_old(GetDepartmentListOldReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .contact()
            .get_department_list_old(GetDepartmentListOldReq::default())
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

    const RESP: &str = "{}";
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetDepartmentListOldRespInner>(RESP);
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