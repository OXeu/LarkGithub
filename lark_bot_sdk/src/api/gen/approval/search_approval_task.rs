//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/search>
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
    /// **api 版本: 2024-07-23T07:32:42+00:00**
    ///
    /// ## 查询任务列表
    ///
    /// 该接口通过不同条件查询审批系统中符合条件的审批任务列表。
    ///
    /// 如需了解审批任务各状态的说明，以及审批任务状态变更事件，可参见[审批任务状态变更](https://open.feishu.cn/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/common-event/approval-task-event)。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/search>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/approval-v4/approval-search/search>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fapproval-v4%2Fapproval-search%2Fsearch>
    pub async fn search_approval_task(
        &self,
        req: SearchApprovalTaskReq,
    ) -> Result<(SearchApprovalTaskResp, CommonResponse), Error> {
        self.search_approval_task_with_opt(req, Default::default())
            .await
    }

    /// 参见 [search_approval_task](#method.search_approval_task) 函数
    pub async fn search_approval_task_with_opt(
        &self,
        req: SearchApprovalTaskReq,
        method_option: MethodOption,
    ) -> Result<(SearchApprovalTaskResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_search_approval_task(&req) {
                tracing::info!("[lark] Approval#SearchApprovalTask **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Approval#SearchApprovalTask call api");

        let req = ApiRequest {
            scope: "Approval",
            api: "SearchApprovalTask",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/approval/v4/tasks/search",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (SearchApprovalTaskRespInner, _) = self.cli.do_req(req).await?;
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
pub struct SearchApprovalTaskReq {
    /// 分页大小
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "nF1ZXJ5VGhlbkZldGNoCgAAAAAA6PZwFmUzSldvTC1yU"
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
    /// 根据x_user_type填写审批人id
    ///
    /// **示例值**: "lwiu098wj"
    #[api(kind = "body", name = "user_id")]
    pub user_id: Option<String>,
    /// 审批定义 code
    ///
    /// **示例值**: "EB828003-9FFE-4B3F-AA50-2E199E2ED942"
    #[api(kind = "body", name = "approval_code")]
    pub approval_code: Option<String>,
    /// 审批实例 code
    ///
    /// **示例值**: "EB828003-9FFE-4B3F-AA50-2E199E2ED943"
    #[api(kind = "body", name = "instance_code")]
    pub instance_code: Option<String>,
    /// 审批实例第三方 id 注：和 approval_code 取并集
    ///
    /// **示例值**: "EB828003-9FFE-4B3F-AA50-2E199E2ED976"
    #[api(kind = "body", name = "instance_external_id")]
    pub instance_external_id: Option<String>,
    /// 审批定义分组第三方 id 注：和 instance_code 取并集
    ///
    /// **示例值**: "1234567"
    #[api(kind = "body", name = "group_external_id")]
    pub group_external_id: Option<String>,
    /// 审批任务标题（只有第三方审批有）
    ///
    /// **示例值**: "test"
    #[api(kind = "body", name = "task_title")]
    pub task_title: Option<String>,
    /// 审批任务状态，注：若不设置，查询全部状态 若不在集合中，报错
    ///
    /// **示例值**: "PENDING"
    ///
    /// **可选值**:
    ///
    /// `Pending`: 审批中
    ///
    /// `Reject`: 拒绝
    ///
    /// `Approverd`: 通过
    ///
    /// `TRANSFERRED`: 转交
    ///
    /// `DONE`: 已完成
    ///
    /// `RM_REPEAT`: 去重
    ///
    /// `PROCESSED`: 已处理
    ///
    /// `ALL`: 所有状态
    #[api(kind = "body", name = "task_status")]
    pub task_status: Option<String>,
    /// 任务查询开始时间（unix毫秒时间戳）
    ///
    /// **示例值**: "1547654251506"
    #[api(kind = "body", name = "task_start_time_from")]
    pub task_start_time_from: Option<String>,
    /// 任务查询结束时间 (unix毫秒时间戳)
    ///
    /// **示例值**: "1547654251506"
    #[api(kind = "body", name = "task_start_time_to")]
    pub task_start_time_to: Option<String>,
    /// 地区
    ///
    /// **示例值**: "zh-CN"
    ///
    /// **可选值**:
    ///
    /// `ZhCn`: 中文
    ///
    /// `EnUs`: 英文
    ///
    /// `JaJp`: 日文
    #[api(kind = "body", name = "locale")]
    pub locale: Option<String>,
    /// 可选择task_status中的多个状态，当填写此参数时，task_status失效
    ///
    /// **示例值**: "PENDING"
    #[api(kind = "body", name = "task_status_list")]
    pub task_status_list: Vec<Option<String>>,
    /// 按任务时间排序
    ///
    /// **示例值**: "2"
    ///
    /// **可选值**:
    ///
    /// `UpdateTimeDESC`: 按update_time倒排
    ///
    /// `UpdateTimeASC`: 按update_time正排
    ///
    /// `StartTimeDESC`: 按start_time倒排
    ///
    /// `StartTimeASC`: 按start_time正排
    #[api(kind = "body", name = "order")]
    pub order: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct SearchApprovalTaskRespInner {
    #[serde(flatten)]
    data: Option<SearchApprovalTaskResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchApprovalTaskResp {
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
    /// 查询返回条数
    ///
    /// **示例值**: "10"
    #[serde(
        rename = "count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub count: i64,
    /// 审批任务列表
    #[serde(
        rename = "task_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub task_list: Vec<TaskSearchItemSubResp>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "nF1ZXJ5VGhlbkZldGNoCgAAAAAA6PZwFmUzSldvTC1yU"
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TaskSearchItemSubResp {
    /// 审批定义
    #[serde(
        rename = "approval",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval: InstanceSearchApprovalSubResp,
    /// 审批定义分组
    #[serde(
        rename = "group",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub group: InstanceSearchGroupSubResp,
    /// 审批实例信息
    #[serde(
        rename = "instance",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub instance: InstanceSearchNodeSubResp,
    /// 审批任务
    #[serde(
        rename = "task",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub task: TaskSearchNodeSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InstanceSearchApprovalSubResp {
    /// 审批定义 code
    ///
    /// **示例值**: "EB828003-9FFE-4B3F-AA50-2E199E2ED943"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 审批定义名称
    ///
    /// **示例值**: "approval"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 是否为第三方审批
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_external",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_external: bool,
    /// 第三方审批信息
    #[serde(
        rename = "external",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub external: InstanceSearchApprovalExternalSubResp,
    /// 审批定义Id
    ///
    /// **示例值**: "7090754740375519252"
    #[serde(
        rename = "approval_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_id: String,
    /// 审批定义图标信息
    ///
    /// **示例值**: "https://lf3-ea.bytetos.com/obj/goofy/ee/approval/approval-admin/image/iconLib/v3/person.png"
    #[serde(
        rename = "icon",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InstanceSearchGroupSubResp {
    /// 审批定义分组外部 id
    ///
    /// **示例值**: "0004"
    #[serde(
        rename = "external_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub external_id: String,
    /// 审批定义分组名称
    ///
    /// **示例值**: "groupA"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InstanceSearchNodeSubResp {
    /// 审批实例 code
    ///
    /// **示例值**: "EB828003-9FFE-4B3F-AA50-2E199E2ED943"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 审批实例外部 id
    ///
    /// **示例值**: "0004_3ED52DC1-AA6C"
    #[serde(
        rename = "external_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub external_id: String,
    /// 审批实例发起人 id
    ///
    /// **示例值**: "lwiu098wj"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 审批实例开始时间
    ///
    /// **示例值**: "1547654251506"
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: String,
    /// 审批实例结束时间
    ///
    /// **示例值**: "1547654251506"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: String,
    /// 审批实例状态
    ///
    /// **示例值**: "pending"
    ///
    /// **可选值**:
    ///
    /// `Reject`: 拒绝
    ///
    /// `Pending`: 审批中
    ///
    /// `Recall`: 撤回
    ///
    /// `Deleted`: 已删除
    ///
    /// `Approved`: 通过
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: String,
    /// 审批实例名称（只有第三方审批有）
    ///
    /// **示例值**: "test"
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: String,
    /// 审批实例扩展字段，string型json
    ///
    /// **示例值**: "{}"
    #[serde(
        rename = "extra",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub extra: String,
    /// 审批流水号
    ///
    /// **示例值**: "201902020001"
    #[serde(
        rename = "serial_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub serial_id: String,
    /// 审批实例链接（只有第三方审批有）
    #[serde(
        rename = "link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub link: InstanceSearchLinkSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TaskSearchNodeSubResp {
    /// 审批任务审批人 id
    ///
    /// **示例值**: "lwiu098wj"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 审批任务开始时间
    ///
    /// **示例值**: "1547654251506"
    #[serde(
        rename = "start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub start_time: String,
    /// 审批任务结束时间
    ///
    /// **示例值**: "1547654251506"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: String,
    /// 审批任务状态
    ///
    /// **示例值**: "PENDING"
    ///
    /// **可选值**:
    ///
    /// `Rejected`: 拒绝
    ///
    /// `Pending`: 审批中
    ///
    /// `Approved`: 通过
    ///
    /// `Transferred`: 转交
    ///
    /// `Done`: 已完成
    ///
    /// `RmRepeat`: 去重
    ///
    /// `Processed`: 已处理
    ///
    /// `Hidden`: 隐藏
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: String,
    /// 审批任务名称（只有第三方审批有）
    ///
    /// **示例值**: "test"
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: String,
    /// 审批任务扩展字段，string型json
    ///
    /// **示例值**: "{}"
    #[serde(
        rename = "extra",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub extra: String,
    /// 审批任务链接（只有第三方审批有）
    #[serde(
        rename = "link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub link: InstanceSearchLinkSubResp,
    /// 任务id
    ///
    /// **示例值**: "7110153401253494803"
    #[serde(
        rename = "task_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub task_id: String,
    /// 审批任务更新时间
    ///
    /// **示例值**: "1547654251506"
    #[serde(
        rename = "update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_time: String,
    /// 三方审批扩展 任务ID
    ///
    /// **示例值**: "123123daddf21313"
    #[serde(
        rename = "task_external_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub task_external_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InstanceSearchApprovalExternalSubResp {
    /// 是否支持批量读
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "batch_cc_read",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub batch_cc_read: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InstanceSearchLinkSubResp {
    /// 审批实例 pc 端链接
    ///
    /// **示例值**: "https://www.baidu.com/"
    #[serde(
        rename = "pc_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub pc_link: String,
    /// 审批实例移动端链接
    ///
    /// **示例值**: "https://www.baidu.com/"
    #[serde(
        rename = "mobile_link",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub mobile_link: String,
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
        Fn(SearchApprovalTaskReq) -> Result<(SearchApprovalTaskResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(SearchApprovalTaskReq) -> Result<(SearchApprovalTaskResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ApprovalServiceMocker<'c, IStore, IClient> {
        pub fn mock_search_approval_task<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, SearchApprovalTaskReq, SearchApprovalTaskResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_search_approval_task(
            &self,
            req: &SearchApprovalTaskReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, SearchApprovalTaskReq, SearchApprovalTaskResp, Arc<dyn MockFunc>>(
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
        api::gen::approval::search_approval_task::{
            SearchApprovalTaskReq, SearchApprovalTaskResp, SearchApprovalTaskRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .approval()
            .mock()
            .mock_search_approval_task(|_| {
                Ok((SearchApprovalTaskResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .approval()
            .search_approval_task(SearchApprovalTaskReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .approval()
            .search_approval_task(SearchApprovalTaskReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "user_id": "lwiu098wj",
    "approval_code": "EB828003-9FFE-4B3F-AA50-2E199E2ED942",
    "instance_code": "EB828003-9FFE-4B3F-AA50-2E199E2ED943",
    "instance_external_id": "EB828003-9FFE-4B3F-AA50-2E199E2ED976",
    "group_external_id": "1234567",
    "task_title": "test",
    "task_status": "PENDING",
    "task_start_time_from": "1547654251506",
    "task_start_time_to": "1547654251506",
    "locale": "zh-CN",
    "task_status_list": [
        "PENDING"
    ],
    "order": 2
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::SearchApprovalTaskReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "count": 10,
        "task_list": [
            {
                "approval": {
                    "code": "EB828003-9FFE-4B3F-AA50-2E199E2ED943",
                    "name": "approval",
                    "is_external": true,
                    "external": {
                        "batch_cc_read": false
                    }
                },
                "group": {
                    "external_id": "0004",
                    "name": "groupA"
                },
                "instance": {
                    "code": "EB828003-9FFE-4B3F-AA50-2E199E2ED943",
                    "external_id": "0004_3ED52DC1-AA6C",
                    "user_id": "lwiu098wj",
                    "start_time": "1547654251506",
                    "end_time": "1547654251506",
                    "status": "pending",
                    "title": "test",
                    "extra": "{}",
                    "serial_id": "201902020001",
                    "link": {
                        "pc_link": "https://www.baidu.com/",
                        "mobile_link": "https://www.baidu.com/"
                    }
                },
                "task": {
                    "user_id": "lwiu098wj",
                    "start_time": "1547654251506",
                    "end_time": "1547654251506",
                    "status": "pending",
                    "title": "test",
                    "extra": "{}",
                    "link": {
                        "pc_link": "https://www.baidu.com/",
                        "mobile_link": "https://www.baidu.com/"
                    },
                    "task_id": "7110153401253494803",
                    "update_time": "1547654251506"
                }
            }
        ],
        "page_token": "nF1ZXJ5VGhlbkZldGNoCgAAAAAA6PZwFmUzSldvTC1yU",
        "has_more": false
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<SearchApprovalTaskRespInner>(RESP);
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