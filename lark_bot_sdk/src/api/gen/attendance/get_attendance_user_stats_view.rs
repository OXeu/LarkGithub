//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_view/query>
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
    /// **api 版本: 2024-07-25T08:30:16+00:00**
    ///
    /// ## 查询统计设置
    ///
    /// 查询考勤统计支持的日度统计或月度统计的统计表头。报表的表头信息可以在考勤统计-[报表](https://example.feishu.cn/people/workforce-management/manage/statistics/report)中查询到具体的报表信息，此接口专门用于查询表头数据。注意此接口和[查询统计表头](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_field/query)基本相同，区别点在于在兼容历史统计视图模型（历史统计数据模型可以按用户ID设置，后续统计升级为仅支持租户维度）
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_view/query>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/attendance-v1/user_stats_data/query>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fattendance-v1%2Fuser_stats_data%2Fquery>
    pub async fn get_attendance_user_stats_view(
        &self,
        req: GetAttendanceUserStatsViewReq,
    ) -> Result<(GetAttendanceUserStatsViewResp, CommonResponse), Error> {
        self.get_attendance_user_stats_view_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_attendance_user_stats_view](#method.get_attendance_user_stats_view) 函数
    pub async fn get_attendance_user_stats_view_with_opt(
        &self,
        req: GetAttendanceUserStatsViewReq,
        method_option: MethodOption,
    ) -> Result<(GetAttendanceUserStatsViewResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_attendance_user_stats_view(&req) {
                tracing::info!("[lark] Attendance#GetAttendanceUserStatsView **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Attendance#GetAttendanceUserStatsView call api");

        let req = ApiRequest {
            scope: "Attendance",
            api: "GetAttendanceUserStatsView",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/attendance/v1/user_stats_views/query",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetAttendanceUserStatsViewRespInner, _) =
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
pub struct GetAttendanceUserStatsViewReq {
    /// 响应体中的 user_id 的员工ID类型。如果没有后台管理权限，可使用[通过手机号或邮箱获取用户 ID](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/batch_get_id)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "employee_id"
    ///
    /// **可选值**:
    ///
    /// `employee_id`: 员工 employee ID，即[飞书管理后台](https://example.feishu.cn/admin/contacts/departmentanduser) > 组织架构 > 成员与部门 > 成员详情中的用户 ID
    ///
    /// `employee_no`: 员工工号，即[飞书管理后台](https://example.feishu.cn/admin/contacts/departmentanduser) > 组织架构 > 成员与部门 > 成员详情中的工号
    #[api(
        kind = "query",
        name = "employee_type",
        v_type = "var",
        option = "false"
    )]
    pub employee_type: String,
    /// 语言类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh"
    ///
    /// **可选值**:
    ///
    /// `en`: 英语
    ///
    /// `ja`: 日语
    ///
    /// `zh`: 中文
    #[api(kind = "body", name = "locale")]
    pub locale: String,
    /// 统计类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "daily"
    ///
    /// **可选值**:
    ///
    /// `daily`: 日度统计
    ///
    /// `month`: 月度统计
    #[api(kind = "body", name = "stats_type")]
    pub stats_type: String,
    /// 操作者的用户id，对应employee_type
    ///
    /// * 必填字段(系统升级后，新系统要求必填)
    ///
    /// **示例值**: "dd31248a"
    #[api(kind = "body", name = "user_id")]
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetAttendanceUserStatsViewRespInner {
    #[serde(flatten)]
    data: Option<GetAttendanceUserStatsViewResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetAttendanceUserStatsViewResp {
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
    /// 统计视图
    #[serde(
        rename = "view",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub view: UserStatsViewSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserStatsViewSubResp {
    /// 视图 ID，可用于[更新统计设置](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_view/update)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: ""TmpZNU5qTTJORFF6T1RnNU5UTTNOakV6TWl0dGIyNTBhQT09""
    #[serde(
        rename = "view_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub view_id: String,
    /// 视图类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: ""month""
    ///
    /// **可选值**:
    ///
    /// `daily`: 日度统计
    ///
    /// `month`: 月度统计
    #[serde(
        rename = "stats_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub stats_type: String,
    /// 操作者的用户id，对应employee_type
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: ""ec8ddg56""
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 用户设置字段
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<ItemSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ItemSubResp {
    /// 标题编号
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "501"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 标题名称
    ///
    /// **示例值**: "基本信息"
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: String,
    /// 子标题
    #[serde(
        rename = "child_items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub child_items: Vec<ChildItemSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ChildItemSubResp {
    /// 子标题编号
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "501"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 开关字段，0：关闭，1：开启
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
    /// 子标题名称
    ///
    /// **示例值**: "工号"
    #[serde(
        rename = "title",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub title: String,
    /// 列类型
    ///
    /// * `0`：未知（默认）
    ///
    /// * `1`：复选框
    ///
    /// * `2`：文本
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "column_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub column_type: i64,
    /// 是否只读
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "read_only",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub read_only: bool,
    /// 最小值
    ///
    /// **示例值**: """"
    #[serde(
        rename = "min_value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub min_value: String,
    /// 最大值
    ///
    /// **示例值**: """"
    #[serde(
        rename = "max_value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub max_value: String,
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
            GetAttendanceUserStatsViewReq,
        ) -> Result<(GetAttendanceUserStatsViewResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetAttendanceUserStatsViewReq,
                )
                    -> Result<(GetAttendanceUserStatsViewResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AttendanceServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_attendance_user_stats_view<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetAttendanceUserStatsViewReq,
            GetAttendanceUserStatsViewResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_attendance_user_stats_view(
            &self,
            req: &GetAttendanceUserStatsViewReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                GetAttendanceUserStatsViewReq,
                GetAttendanceUserStatsViewResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::attendance::get_attendance_user_stats_view::{
            GetAttendanceUserStatsViewReq, GetAttendanceUserStatsViewResp,
            GetAttendanceUserStatsViewRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .attendance()
            .mock()
            .mock_get_attendance_user_stats_view(|_| {
                Ok((
                    GetAttendanceUserStatsViewResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .attendance()
            .get_attendance_user_stats_view(GetAttendanceUserStatsViewReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .attendance()
            .get_attendance_user_stats_view(GetAttendanceUserStatsViewReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "locale": "zh",
    "stats_type": "daily",
    "user_id": "dd31248a"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::GetAttendanceUserStatsViewReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "",
    "data": {
        "view": {
            "items": [
                {
                    "child_items": [
                        {
                            "code": "50101",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": true,
                            "title": "姓名",
                            "value": "1"
                        },
                        {
                            "code": "50102",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "部门",
                            "value": "0"
                        },
                        {
                            "code": "50111",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "组织架构",
                            "value": "0"
                        },
                        {
                            "code": "50103",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "工号",
                            "value": "1"
                        },
                        {
                            "code": "50104",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "邮箱",
                            "value": "0"
                        },
                        {
                            "code": "50105",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "雇员类型",
                            "value": "0"
                        },
                        {
                            "code": "50106",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "序列",
                            "value": "0"
                        },
                        {
                            "code": "50107",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "入职时间",
                            "value": "0"
                        },
                        {
                            "code": "50108",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "离职时间",
                            "value": "0"
                        },
                        {
                            "code": "50109",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "状态",
                            "value": "0"
                        },
                        {
                            "code": "50110",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "直属上级",
                            "value": "0"
                        }
                    ],
                    "code": "501",
                    "title": "基本信息"
                },
                {
                    "child_items": [
                        {
                            "code": "52108",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "考勤组名称",
                            "value": "1"
                        },
                        {
                            "code": "52101",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "应出勤天数",
                            "value": "1"
                        },
                        {
                            "code": "52102",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "工作日出勤天数",
                            "value": "1"
                        },
                        {
                            "code": "52103",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "休息日出勤天数",
                            "value": "0"
                        },
                        {
                            "code": "52104",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "应出勤时长",
                            "value": "1"
                        },
                        {
                            "code": "52105",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "实际出勤时长",
                            "value": "1"
                        },
                        {
                            "code": "52106",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "计薪工作时长",
                            "value": "0"
                        },
                        {
                            "code": "52107",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "加班工作时长",
                            "value": "1"
                        },
                        {
                            "code": "52109",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "加班时长(计加班费)\n",
                            "value": "0"
                        },
                        {
                            "code": "52110",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "加班时长(计为调休)\n",
                            "value": "0"
                        }
                    ],
                    "code": "521",
                    "title": "出勤统计"
                },
                {
                    "child_items": [
                        {
                            "code": "52201",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "迟到次数",
                            "value": "1"
                        },
                        {
                            "code": "52202",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "迟到时长",
                            "value": "0"
                        },
                        {
                            "code": "52203",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "早退次数",
                            "value": "1"
                        },
                        {
                            "code": "52204",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "早退时长",
                            "value": "0"
                        },
                        {
                            "code": "52205",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "上班缺卡次数",
                            "value": "0"
                        },
                        {
                            "code": "52206",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "下班缺卡次数",
                            "value": "0"
                        },
                        {
                            "code": "52207",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "缺勤",
                            "value": "1"
                        },
                        {
                            "code": "52208",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "请假时长",
                            "value": "0"
                        },
                        {
                            "code": "52209",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "出差时长",
                            "value": "0"
                        },
                        {
                            "code": "52211",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "换班天数",
                            "value": "0"
                        },
                        {
                            "code": "52212",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "补卡次数",
                            "value": "0"
                        },
                        {
                            "code": "52213",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "外勤次数",
                            "value": "0"
                        },
                        {
                            "code": "52214",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "外出时长\t",
                            "value": "0"
                        }
                    ],
                    "code": "522",
                    "title": "异常统计"
                },
                {
                    "child_items": [
                        {
                            "code": "52401",
                            "column_type": 0,
                            "max_value": "",
                            "min_value": "",
                            "read_only": false,
                            "title": "每日考勤结果",
                            "value": "1"
                        }
                    ],
                    "code": "524",
                    "title": "每日统计"
                }
            ],
            "stats_type": "month",
            "user_id": "ec8ddg56",
            "view_id": "TmpZNU5qTTJORFF6T1RnNU5UTTNOakV6TWl0dGIyNTBhQT09"
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetAttendanceUserStatsViewRespInner>(RESP);
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
