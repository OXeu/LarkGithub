//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/get>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::approval::ApprovalService;

impl<'c, IStore: Store, IClient: HttpClient> ApprovalService<'c, IStore, IClient> {
    /// **api 版本: 2024-03-01T07:48:45+00:00**
    ///
    /// ## 查看指定审批定义
    ///
    /// 根据 Approval Code 获取某个审批定义的详情，用于构造创建审批实例的请求。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/approval-v4/approval/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fapproval-v4%2Fapproval%2Fget>
    pub async fn get_approval(
        &self,
        req: GetApprovalReq,
    ) -> Result<(GetApprovalResp, CommonResponse), Error> {
        self.get_approval_with_opt(req, Default::default()).await
    }

    /// 参见 [get_approval](#method.get_approval) 函数
    pub async fn get_approval_with_opt(
        &self,
        req: GetApprovalReq,
        method_option: MethodOption,
    ) -> Result<(GetApprovalResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_approval(&req) {
                tracing::info!("[lark] Approval#GetApproval **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Approval#GetApproval call api");

        let req = ApiRequest {
            scope: "Approval",
            api: "GetApproval",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/approval/v4/approvals/:approval_code",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetApprovalRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetApprovalReq {
    /// 审批定义 Code
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7C468A54-8745-2245-9675-08B7C63E7A85"
    #[api(kind = "path", name = "approval_code")]
    pub approval_code: String,
    /// 语言可选值
    ///
    /// **示例值**: "zh-CN"
    ///
    /// **可选值**:
    ///
    /// `Zhcn`: 中文
    ///
    /// `Enus`: 英文
    ///
    /// `Jajp`: 日文
    #[api(kind = "query", name = "locale", v_type = "var", option = "false")]
    pub locale: String,
    /// 可选是否返回有数据权限审批流程管理员ID列表
    ///
    /// **示例值**: "false"
    #[api(
        kind = "query",
        name = "with_admin_id",
        v_type = "var",
        option = "false"
    )]
    pub with_admin_id: bool,
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
struct GetApprovalRespInner {
    #[serde(flatten)]
    data: Option<GetApprovalResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetApprovalResp {
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
    /// 审批名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Payment"
    #[serde(
        rename = "approval_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_name: String,
    /// 审批定义状态
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "ACTIVE"
    ///
    /// **可选值**:
    ///
    /// `ACTIVE`: 已启用
    ///
    /// `INACTIVE`: 已停用
    ///
    /// `DELETED`: 已删除
    ///
    /// `UNKNOWN`: 未知
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: String,
    /// 控件信息，见下方form字段说明
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "[{\"id\": \"widget1\", \"custom_id\": \"user_name\",\"name\": \"Item application\",\"type\": \"textarea\",\"printable\": true,\"required\": true}]"
    #[serde(
        rename = "form",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub form: String,
    /// 节点信息
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "node_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_list: Vec<ApprovalNodeInfoSubResp>,
    /// 可见人列表
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "viewers",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub viewers: Vec<ApprovalViewerInfoSubResp>,
    /// 有数据管理权限的审批流程管理员ID，由参数“with_admin_id”控制是否返回
    #[serde(
        rename = "approval_admin_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_admin_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApprovalNodeInfoSubResp {
    /// 节点名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Approval"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 是否发起人自选节点 true - 发起审批时需要提交审批人
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "need_approver",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub need_approver: bool,
    /// 节点 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "46e6d96cfa756980907209209ec03b64"
    #[serde(
        rename = "node_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_id: String,
    /// 节点自定义 ID，如果没有设置则不返回
    ///
    /// **示例值**: "46e6d96cfa756980907209209ec03b64"
    #[serde(
        rename = "custom_node_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_node_id: String,
    /// 审批方式
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "AND"
    ///
    /// **可选值**:
    ///
    /// `And`: 会签
    ///
    /// `Or`: 或签
    ///
    /// `Sequential`: 依次审批
    ///
    /// `CcNode`: 抄送节点
    #[serde(
        rename = "node_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub node_type: String,
    /// 是否支持多选：true-支持，发起、结束节点该值无意义
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "approver_chosen_multi",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approver_chosen_multi: bool,
    /// 自选范围
    #[serde(
        rename = "approver_chosen_range",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approver_chosen_range: Vec<ApproverChosenRangeSubResp>,
    /// 是否签名
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "require_signature",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub require_signature: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApprovalViewerInfoSubResp {
    /// 可见人类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "TENANT"
    ///
    /// **可选值**:
    ///
    /// `Tenant`: 租户内可见
    ///
    /// `Department`: 指定部门
    ///
    /// `User`: 指定用户
    ///
    /// `Role`: 指定角色
    ///
    /// `UserGroup`: 指定用户组
    ///
    /// `None`: 任何人都不可见
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: String,
    /// 在可见人类型为DEPARTMENT时，id为部门的id ；在可见人类型为USER时，id为用户的id ；在可见人类型为ROLE时，id为角色的id ；在可见人类型为USER_GROUP时，id为用户组的id
    ///
    /// **示例值**: "ou_e03053f0541cecc3269d7a9dc34a0b21"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 在可见人类型为USER时，表示可见人用户id
    ///
    /// **示例值**: "f7cb567e"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApproverChosenRangeSubResp {
    /// 指定范围：0-all，1-指定角色，2-指定人员
    ///
    /// **示例值**: "2"
    ///
    /// **可选值**:
    ///
    /// `ALL`: 全公司范围
    ///
    /// `ROLE`: 指定角色范围
    ///
    /// `USER`: 指定用户范围
    #[serde(
        rename = "approver_range_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approver_range_type: i64,
    /// 根据上面的type，分别存放角色id与人员open_id，type为0时本字段为空列表
    #[serde(
        rename = "approver_range_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approver_range_ids: Vec<String>,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::approval::ApprovalServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetApprovalReq) -> Result<(GetApprovalResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetApprovalReq) -> Result<(GetApprovalResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ApprovalServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_approval<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetApprovalReq, GetApprovalResp, Arc<dyn MockFunc>> {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_approval(
            &self,
            req: &GetApprovalReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetApprovalReq, GetApprovalResp, Arc<dyn MockFunc>>(
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
        api::gen::approval::get_approval::{GetApprovalReq, GetApprovalResp, GetApprovalRespInner},
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .approval()
            .mock()
            .mock_get_approval(|_| Ok((GetApprovalResp::default(), CommonResponse::default())))
            .build();
        let res = lark
            .approval()
            .get_approval(GetApprovalReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .approval()
            .get_approval(GetApprovalReq::default())
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
        "approval_name": "Payment",
        "status": "ACTIVE",
        "form": "[{\"id\": \"widget1\", \"custom_id\": \"user_name\",\"name\": \"Item application\",\"type\": \"textarea\",\"printable\": true,\"required\": true}]",
        "node_list": [
            {
                "name": "Approval",
                "need_approver": true,
                "node_id": "46e6d96cfa756980907209209ec03b64",
                "custom_node_id": "46e6d96cfa756980907209209ec03b64",
                "node_type": "AND",
                "approver_chosen_multi": true,
                "approver_chosen_range": [
                    {
                        "approver_range_type": 2,
                        "approver_range_ids": [
                            "ou_e03053f0541cecc3269d7a9dc34a0b21"
                        ]
                    }
                ],
                "require_signature": false
            }
        ],
        "viewers": [
            {
                "type": "TENANT",
                "id": "ou_e03053f0541cecc3269d7a9dc34a0b21",
                "user_id": "f7cb567e"
            }
        ],
        "approval_admin_ids": [
            "ou_3cda9c969f737aaa05e6915dce306cb9"
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetApprovalRespInner>(RESP);
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