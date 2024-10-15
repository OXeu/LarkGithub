//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/query>
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
    /// **api 版本: 2024-07-16T09:26:23+00:00**
    ///
    /// ## 批量查询用户人脸识别信息
    ///
    /// 批量查询授权内员工的用户设置信息，包括人脸照片文件 ID、人脸照片更新时间。对应页面假勤设置-[人脸识别](https://example.feishu.cn/people/workforce-management/setting/group/security)。根据返回的face_key可以下载人脸信息[下载用户人脸识别照片
    ///
    /// ](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/file/download)
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/query>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/attendance-v1/user_setting/query>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fattendance-v1%2Fuser_setting%2Fquery>
    pub async fn get_attendance_user_setting_list(
        &self,
        req: GetAttendanceUserSettingListReq,
    ) -> Result<(GetAttendanceUserSettingListResp, CommonResponse), Error> {
        self.get_attendance_user_setting_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_attendance_user_setting_list](#method.get_attendance_user_setting_list) 函数
    pub async fn get_attendance_user_setting_list_with_opt(
        &self,
        req: GetAttendanceUserSettingListReq,
        method_option: MethodOption,
    ) -> Result<(GetAttendanceUserSettingListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_attendance_user_setting_list(&req) {
                tracing::info!("[lark] Attendance#GetAttendanceUserSettingList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Attendance#GetAttendanceUserSettingList call api");

        let req = ApiRequest {
            scope: "Attendance",
            api: "GetAttendanceUserSettingList",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/attendance/v1/user_settings/query",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetAttendanceUserSettingListRespInner, _) =
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
pub struct GetAttendanceUserSettingListReq {
    /// 请求体中的 user_ids 和响应体中的 user_id 的员工ID类型。如果没有后台管理权限，可使用[通过手机号或邮箱获取用户 ID](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/batch_get_id)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "employee_id"
    ///
    /// **可选值**:
    ///
    /// `EmployeeId`: 员工 employee ID，即[飞书管理后台](https://example.feishu.cn/admin/contacts/departmentanduser) > 组织架构 > 成员与部门 > 成员详情中的用户 ID
    ///
    /// `EmployeeNo`: 员工工号，即[飞书管理后台](https://example.feishu.cn/admin/contacts/departmentanduser) > 组织架构 > 成员与部门 > 成员详情中的工号
    #[api(
        kind = "query",
        name = "employee_type",
        v_type = "var",
        option = "false"
    )]
    pub employee_type: String,
    /// employee_no 或 employee_id 列表，对应employee_type
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "["abd754f7"]"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `100` 字符
    #[api(kind = "body", name = "user_ids")]
    pub user_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetAttendanceUserSettingListRespInner {
    #[serde(flatten)]
    data: Option<GetAttendanceUserSettingListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetAttendanceUserSettingListResp {
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
    /// 用户设置信息列表
    #[serde(
        rename = "user_settings",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_settings: Vec<UserSettingSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserSettingSubResp {
    /// 用户 ID，对应employee_type
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "abd754f7"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 人脸照片文件 ID，可用于：[下载用户人脸识别照片](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/file/download)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "xxxxxb306842b1c189bc5212eefxxxxx"
    #[serde(
        rename = "face_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub face_key: String,
    /// 人脸照片更新时间，精确到秒的时间戳
    ///
    /// **示例值**: "1625681917"
    #[serde(
        rename = "face_key_update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub face_key_update_time: String,
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
            GetAttendanceUserSettingListReq,
        ) -> Result<(GetAttendanceUserSettingListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetAttendanceUserSettingListReq,
                )
                    -> Result<(GetAttendanceUserSettingListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AttendanceServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_attendance_user_setting_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetAttendanceUserSettingListReq,
            GetAttendanceUserSettingListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_attendance_user_setting_list(
            &self,
            req: &GetAttendanceUserSettingListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetAttendanceUserSettingListReq,
                GetAttendanceUserSettingListResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::attendance::get_attendance_user_setting_list::{
            GetAttendanceUserSettingListReq, GetAttendanceUserSettingListResp,
            GetAttendanceUserSettingListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .attendance()
            .mock()
            .mock_get_attendance_user_setting_list(|_| {
                Ok((
                    GetAttendanceUserSettingListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .attendance()
            .get_attendance_user_setting_list(GetAttendanceUserSettingListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .attendance()
            .get_attendance_user_setting_list(GetAttendanceUserSettingListReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "user_ids": [
        "abd754f7"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::GetAttendanceUserSettingListReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "user_settings": [
            {
                "user_id": "abd754f7",
                "face_key": "xxxxxb306842b1c189bc5212eefxxxxx",
                "face_key_update_time": "1625681917"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetAttendanceUserSettingListRespInner>(RESP);
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