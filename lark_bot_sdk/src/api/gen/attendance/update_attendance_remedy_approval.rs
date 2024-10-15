//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/approval_info/process>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::attendance::AttendanceService;

impl<'c, IStore: Store, IClient: HttpClient> AttendanceService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-10T11:33:32+00:00**
    ///
    /// ## 通知审批状态更新
    ///
    /// 对于只使用飞书考勤系统而未使用飞书审批系统的企业，可以通过该接口更新写入飞书考勤系统中的三方系统审批状态，例如请假、加班、外出、出差、补卡等审批，状态包括通过、不通过、撤销等。
    ///
    /// 发起状态的审批才可以被更新为通过、不通过，已经通过的审批才可以被更新为撤销。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/approval_info/process>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/attendance-v1/user_approval/process>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fattendance-v1%2Fuser_approval%2Fprocess>
    pub async fn update_attendance_remedy_approval(
        &self,
        req: UpdateAttendanceRemedyApprovalReq,
    ) -> Result<(UpdateAttendanceRemedyApprovalResp, CommonResponse), Error> {
        self.update_attendance_remedy_approval_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_attendance_remedy_approval](#method.update_attendance_remedy_approval) 函数
    pub async fn update_attendance_remedy_approval_with_opt(
        &self,
        req: UpdateAttendanceRemedyApprovalReq,
        method_option: MethodOption,
    ) -> Result<(UpdateAttendanceRemedyApprovalResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_attendance_remedy_approval(&req) {
                tracing::info!("[lark] Attendance#UpdateAttendanceRemedyApproval **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Attendance#UpdateAttendanceRemedyApproval call api");

        let req = ApiRequest {
            scope: "Attendance",
            api: "UpdateAttendanceRemedyApproval",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/attendance/v1/approval_infos/process",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateAttendanceRemedyApprovalRespInner, _) =
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
pub struct UpdateAttendanceRemedyApprovalReq {
    /// 审批实例 ID，获取方式：1）[获取审批通过数据](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_approval/query) 2）[写入审批结果](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_approval/create) 3）[通知补卡审批发起（补卡情况下）](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/create)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6737202939523236113"
    #[api(kind = "body", name = "approval_id")]
    pub approval_id: String,
    /// 审批类型
    ///
    /// - `leave`：请假
    ///
    /// - `out`：外出
    ///
    /// - `overtime`：加班
    ///
    /// - `trip`：出差
    ///
    /// - `remedy`：补卡
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "remedy"
    #[api(kind = "body", name = "approval_type")]
    pub approval_type: String,
    /// 审批状态
    ///
    /// - `1`：不通过
    ///
    /// - `2`：通过
    ///
    /// - `4`：撤销
    ///
    /// **注意**
    ///
    /// - **请假、外出、加班、出差**只支持传**撤销**
    ///
    /// - **补卡**支持传**不通过、通过和撤销**
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "4"
    #[api(kind = "body", name = "status")]
    pub status: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateAttendanceRemedyApprovalRespInner {
    #[serde(flatten)]
    data: Option<UpdateAttendanceRemedyApprovalResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateAttendanceRemedyApprovalResp {
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
    /// 审批信息
    #[serde(
        rename = "approval_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_info: ApprovalInfoSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApprovalInfoSubResp {
    /// 审批实例 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6737202939523236113"
    #[serde(
        rename = "approval_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_id: String,
    /// 审批类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "remedy"
    ///
    /// **可选值**:
    ///
    /// `Leave`: 请假
    ///
    /// `OverTime`: 加班
    ///
    /// `Trip`: 出差
    ///
    /// `Out`: 外出
    ///
    /// `Remedy`: 补卡
    #[serde(
        rename = "approval_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub approval_type: String,
    /// 审批状态
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `Todo`: 待审批
    ///
    /// `Rejected`: 未通过
    ///
    /// `Approved`: 已通过
    ///
    /// `Canceled`: 已取消
    ///
    /// `Reverted`: 已撤回
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::attendance::AttendanceServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            UpdateAttendanceRemedyApprovalReq,
        ) -> Result<(UpdateAttendanceRemedyApprovalResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateAttendanceRemedyApprovalReq,
                )
                    -> Result<(UpdateAttendanceRemedyApprovalResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AttendanceServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_attendance_remedy_approval<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateAttendanceRemedyApprovalReq,
            UpdateAttendanceRemedyApprovalResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_attendance_remedy_approval(
            &self,
            req: &UpdateAttendanceRemedyApprovalReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateAttendanceRemedyApprovalReq,
                UpdateAttendanceRemedyApprovalResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::attendance::update_attendance_remedy_approval::{
            UpdateAttendanceRemedyApprovalReq, UpdateAttendanceRemedyApprovalResp,
            UpdateAttendanceRemedyApprovalRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .attendance()
            .mock()
            .mock_update_attendance_remedy_approval(|_| {
                Ok((
                    UpdateAttendanceRemedyApprovalResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .attendance()
            .update_attendance_remedy_approval(UpdateAttendanceRemedyApprovalReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .attendance()
            .update_attendance_remedy_approval(UpdateAttendanceRemedyApprovalReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "approval_id": "6737202939523236113",
    "approval_type": "remedy",
    "status": 4
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateAttendanceRemedyApprovalReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "approval_info": {
            "approval_id": "6737202939523236113",
            "approval_type": "remedy",
            "status": 0
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateAttendanceRemedyApprovalRespInner>(RESP);
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