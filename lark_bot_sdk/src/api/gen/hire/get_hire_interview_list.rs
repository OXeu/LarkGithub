//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview/list>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::hire::HireService;

impl<'c, IStore: Store, IClient: HttpClient> HireService<'c, IStore, IClient> {
    /// **api 版本: 2024-07-30T12:24:14+00:00**
    ///
    /// ## 获取面试信息
    ///
    /// 获取面试信息。可通过「投递 ID」、「面试 ID」以及「面试开始时间」进行条件筛选，筛选条件不能同时为空。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/candidate-management/delivery-process-management/interview/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fcandidate-management%2Fdelivery-process-management%2Finterview%2Flist>
    pub async fn get_hire_interview_list(
        &self,
        req: GetHireInterviewListReq,
    ) -> Result<(GetHireInterviewListResp, CommonResponse), Error> {
        self.get_hire_interview_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_hire_interview_list](#method.get_hire_interview_list) 函数
    pub async fn get_hire_interview_list_with_opt(
        &self,
        req: GetHireInterviewListReq,
        method_option: MethodOption,
    ) -> Result<(GetHireInterviewListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_hire_interview_list(&req) {
                tracing::info!("[lark] Hire#GetHireInterviewList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#GetHireInterviewList call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "GetHireInterviewList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/hire/v1/interviews",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetHireInterviewListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetHireInterviewListReq {
    /// 分页大小
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "eyJiaXpfdGltZSI6MTcxMDAzNjAwMDAwMCwiaWQiOiI3MzQzMDI3OTMyODE4NjcxOTE2In0"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 投递 ID，可通过[获取投递列表](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/list)接口获取（不允许 application_id、interview_id、start_time、end_time 同时为空）
    ///
    /// **示例值**: "6134134355464633"
    #[api(
        kind = "query",
        name = "application_id",
        v_type = "var",
        option = "false"
    )]
    pub application_id: String,
    /// 面试 ID，可通过[获取面试信息](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview/list)接口获取，（不允许 application_id、interview_id、start_time、end_time 同时为空）
    ///
    /// **示例值**: "6888217964693309704"
    #[api(
        kind = "query",
        name = "interview_id",
        v_type = "var",
        option = "false"
    )]
    pub interview_id: String,
    /// 面试最早开始时间，毫秒时间戳，必须大于 0（不允许 application_id、interview_id、start_time、end_time 同时为空）
    ///
    /// **示例值**: "1609489908000"
    #[api(kind = "query", name = "start_time", v_type = "var", option = "false")]
    pub start_time: String,
    /// 面试最晚开始时间，毫秒时间戳，必须大于 0（不允许 application_id、interview_id、start_time、end_time 同时为空）
    ///
    /// **示例值**: "1610489908000"
    #[api(kind = "query", name = "end_time", v_type = "var", option = "false")]
    pub end_time: String,
    /// 此次调用中使用的「职级 ID」的类型
    ///
    /// **示例值**: "people_admin_hob_level_id"
    ///
    /// **可选值**:
    ///
    /// `people_admin_job_level_id`: 「人力系统管理后台」适用的职级 ID。人力系统管理后台逐步下线中，建议不继续使用此 ID。
    ///
    /// `job_level_id`: 「飞书管理后台」适用的职级 ID，通过[获取租户职级列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_level/list)
    ///
    /// 接口获取
    #[api(
        kind = "query",
        name = "job_level_id_type",
        v_type = "var",
        option = "false"
    )]
    pub job_level_id_type: String,
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
struct GetHireInterviewListRespInner {
    #[serde(flatten)]
    data: Option<GetHireInterviewListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetHireInterviewListResp {
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
    /// 面试列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<InterviewExtendSubResp>,
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "1234452132"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InterviewExtendSubResp {
    /// 面试 ID
    ///
    /// **示例值**: "6949805467799537964"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 面试开始时间，毫秒时间戳
    ///
    /// **示例值**: "1618899376474"
    #[serde(
        rename = "begin_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub begin_time: i64,
    /// 面试结束时间，毫秒时间戳
    ///
    /// **示例值**: "1618999376474"
    #[serde(
        rename = "end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub end_time: i64,
    /// 面试轮次
    ///
    /// **示例值**: "0"
    #[serde(
        rename = "round",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub round: i64,
    /// 面试记录信息
    #[serde(
        rename = "interview_record_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_record_list: Vec<InterviewRecordSubResp>,
    /// 面试评价提交时间，毫秒时间戳
    ///
    /// **示例值**: "1659318415000"
    #[serde(
        rename = "feedback_submit_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub feedback_submit_time: i64,
    /// 面试关联的投递阶段，详情请查看：[获取招聘流程信息](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_process/list)
    ///
    /// **示例值**: "634324253532232"
    #[serde(
        rename = "stage_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub stage_id: String,
    /// 投递 ID，详情参考[获取投递信息](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_application/create)
    ///
    /// **示例值**: "634324253532232"
    #[serde(
        rename = "application_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub application_id: String,
    /// 阶段信息
    #[serde(
        rename = "stage",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub stage: IdNameObjectSubResp,
    /// 创建人信息
    #[serde(
        rename = "creator",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub creator: IdNameObjectSubResp,
    /// 创建时间，毫秒时间戳
    ///
    /// **示例值**: "1618999376474"
    #[serde(
        rename = "biz_create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub biz_create_time: i64,
    /// 最近更新时间，毫秒时间戳
    ///
    /// **示例值**: "1618999376474"
    #[serde(
        rename = "biz_modify_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub biz_modify_time: i64,
    /// 面试状态
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `未开始`: 未开始
    ///
    /// `全部未评价`: 全部未评价
    ///
    /// `全部通过`: 全部通过
    ///
    /// `全部淘汰`: 全部淘汰
    ///
    /// `爽约`: 爽约
    ///
    /// `部分评价且均评价通过`: 部分评价且均评价通过
    ///
    /// `部分评价且评价中有通过有淘汰的`: 部分评价且评价中有通过有淘汰的
    ///
    /// `部分评价且均评价淘汰`: 部分评价且均评价淘汰
    ///
    /// `所有面试官都提交评价且评价中有通过有淘汰的`: 所有面试官都提交评价且评价中有通过有淘汰的
    #[serde(
        rename = "interview_round_summary",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_round_summary: i64,
    /// 面试安排 ID
    ///
    /// **示例值**: "1111111"
    #[serde(
        rename = "interview_arrangement_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_arrangement_id: String,
    /// 面试类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `现场面试`: 现场面试
    ///
    /// `电话面试`: 电话面试
    ///
    /// `视频面试`: 视频面试
    #[serde(
        rename = "interview_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_type: i64,
    /// 候选人时区
    #[serde(
        rename = "talent_time_zone",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub talent_time_zone: CodeNameObjectSubResp,
    /// 面试联系人
    #[serde(
        rename = "contact_user",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contact_user: IdNameObjectSubResp,
    /// 面试联系人电话
    ///
    /// **示例值**: "13333333333"
    #[serde(
        rename = "contact_mobile",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contact_mobile: String,
    /// 备注
    ///
    /// **示例值**: "这是一个备注"
    #[serde(
        rename = "remark",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub remark: String,
    /// 面试地点
    #[serde(
        rename = "address",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub address: InterviewAddressSubResp,
    /// 视频面试工具
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Zoom`: Zoom
    ///
    /// `牛客技术类型`: 牛客技术类型
    ///
    /// `牛客非技术类型`: 牛客非技术类型
    ///
    /// `赛码`: 赛码
    ///
    /// `Lark`: 飞书
    ///
    /// `Hackerrank`: Hackerrank
    ///
    /// `飞书（含代码考核）`: 飞书（含代码考核）
    ///
    /// `不使用系统工具`: 不使用系统工具
    #[serde(
        rename = "video_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub video_type: i64,
    /// 当安排类型为集中面试时，此值表示集中面试的安排状态。非集中面试该字段无含义。
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `未开始`: 未开始
    ///
    /// `进行中`: 进行中
    ///
    /// `已结束`: 已结束
    #[serde(
        rename = "arrangement_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub arrangement_status: i64,
    /// 安排类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `社招单面`: 社招单面
    ///
    /// `集中面试`: 集中面试
    #[serde(
        rename = "arrangement_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub arrangement_type: i64,
    /// 安排方式（是否使用自助约面）
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `直接安排`: 直接安排
    ///
    /// `自助约面`: 自助约面
    #[serde(
        rename = "arrangement_appointment_kind",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub arrangement_appointment_kind: i64,
    /// 面试会议室
    #[serde(
        rename = "meeting_room_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub meeting_room_list: Vec<InterviewMeetingRoomSubResp>,
    /// 面试轮次类型
    #[serde(
        rename = "interview_round_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_round_type: IdNameObjectSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct IdNameObjectSubResp {
    /// 投递阶段 ID，详情请查看：[获取招聘流程信息](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_process/list)
    ///
    /// **示例值**: "1213213123123"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 阶段信息名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: I18nSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CodeNameObjectSubResp {
    /// 编码
    ///
    /// **示例值**: "UTC+08:00"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 候选人时区名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: I18nSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InterviewAddressSubResp {
    /// 地址 ID，可通过[获取地址列表](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/location/list)获取
    ///
    /// **示例值**: "6949805467799537964"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 地址名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: I18nSubResp,
    /// 地址区域
    #[serde(
        rename = "district",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub district: CodeNameObjectSubResp,
    /// 城市
    #[serde(
        rename = "city",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub city: CodeNameObjectSubResp,
    /// 省
    #[serde(
        rename = "state",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub state: CodeNameObjectSubResp,
    /// 国家
    #[serde(
        rename = "country",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub country: CodeNameObjectSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InterviewRecordSubResp {
    /// 面试记录 ID
    ///
    /// **示例值**: "6949805467799537964"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 面试官用户 ID，与入参`user_id_type`类型一致
    ///
    /// **示例值**: "1618899376474"
    #[serde(
        rename = "user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_id: String,
    /// 系统预设题目内容，来自面试评价表中预设字段「记录」，详情参考[获取面试评价表信息](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview_record_template/list)
    ///
    /// **示例值**: "符合要求，推荐录用"
    #[serde(
        rename = "content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub content: String,
    /// 面试评价的提交状态
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Submited`: 已提交
    ///
    /// `UnSubmited`: 未提交
    #[serde(
        rename = "commit_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub commit_status: i64,
    /// 面试结论
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Pass`: 通过
    ///
    /// `Reject`: 未通过
    ///
    /// `NoStart`: 未开始
    ///
    /// `UnCommited`: 未提交
    ///
    /// `NoShow`: 未到场
    #[serde(
        rename = "conclusion",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub conclusion: i64,
    /// 面试评分
    #[serde(
        rename = "interview_score",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_score: InterviewScoreSubResp,
    /// 面试官信息
    #[serde(
        rename = "interviewer",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interviewer: IdNameObjectSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 阶段信息中文名称
    ///
    /// **示例值**: "笔试"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 阶段信息英文名称
    ///
    /// **示例值**: "writing"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InterviewMeetingRoomSubResp {
    /// 会议室 ID
    ///
    /// **示例值**: "6949805467799537964"
    #[serde(
        rename = "room_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub room_id: String,
    /// 会议室名称
    ///
    /// **示例值**: "OCG111"
    #[serde(
        rename = "room_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub room_name: String,
    /// 建筑名称
    ///
    /// **示例值**: "OCG"
    #[serde(
        rename = "building_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub building_name: String,
    /// 会议室预定状态
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `预约中`: 预约中
    ///
    /// `预约成功`: 预约成功
    ///
    /// `预约失败`: 预约失败
    #[serde(
        rename = "reserved_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub reserved_status: i64,
    /// 楼层
    ///
    /// **示例值**: "17"
    #[serde(
        rename = "floor_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub floor_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InterviewScoreSubResp {
    /// 面试评分 ID，对应[获取面试评价表列表](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview_feedback_form/list)接口返回数据中的 `data.items.modules.dimensions.option_items.id` 字段
    ///
    /// **示例值**: "6949805467799537964"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 分数级别
    ///
    /// **示例值**: "3"
    #[serde(
        rename = "level",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub level: i64,
    /// 面试记录中文名称
    ///
    /// **示例值**: "第一轮面试"
    #[serde(
        rename = "zh_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_name: String,
    /// 面试记录中文描述
    ///
    /// **示例值**: "通过，能力达到要求， 建议录用"
    #[serde(
        rename = "zh_description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_description: String,
    /// 面试记录英文名称
    ///
    /// **示例值**: "English name"
    #[serde(
        rename = "en_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_name: String,
    /// 面试记录英文描述
    ///
    /// **示例值**: "Pass, ability to meet the requirements, suggest to hire"
    #[serde(
        rename = "en_description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_description: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::hire::HireServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetHireInterviewListReq) -> Result<(GetHireInterviewListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetHireInterviewListReq,
                ) -> Result<(GetHireInterviewListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_hire_interview_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetHireInterviewListReq,
            GetHireInterviewListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_hire_interview_list(
            &self,
            req: &GetHireInterviewListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetHireInterviewListReq, GetHireInterviewListResp, Arc<dyn MockFunc>>(
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
        api::gen::hire::get_hire_interview_list::{
            GetHireInterviewListReq, GetHireInterviewListResp, GetHireInterviewListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_get_hire_interview_list(|_| {
                Ok((
                    GetHireInterviewListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .get_hire_interview_list(GetHireInterviewListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .get_hire_interview_list(GetHireInterviewListReq::default())
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
        "items": [
            {
                "id": "6949805467799537964",
                "begin_time": 1618899376474,
                "end_time": 1618999376474,
                "round": 0,
                "interview_record_list": [
                    {
                        "id": "6949805467799537964",
                        "user_id": "1618899376474",
                        "content": "符合要求，推荐录用",
                        "commit_status": 1,
                        "conclusion": 1,
                        "interview_score": {
                            "id": "6949805467799537964",
                            "level": 3,
                            "zh_name": "第一轮面试",
                            "zh_description": "通过，能力达到要求， 建议录用",
                            "en_name": "English name",
                            "en_description": "Pass, ability to meet the requirements, suggest to hire"
                        },
                        "interviewer": {
                            "id": "1213213123123",
                            "name": {
                                "zh_cn": "周小二",
                                "en_us": "Tony Ma"
                            }
                        }
                    }
                ],
                "feedback_submit_time": 1659318415000,
                "stage_id": "634324253532232",
                "application_id": "634324253532232",
                "stage": {
                    "id": "1213213123123",
                    "name": {
                        "zh_cn": "笔试",
                        "en_us": "writing"
                    }
                },
                "creator": {
                    "id": "1213213123123",
                    "name": {
                        "zh_cn": "张二二",
                        "en_us": "Tony Ma"
                    }
                },
                "biz_create_time": 1618999376474,
                "biz_modify_time": 1618999376474,
                "interview_round_summary": 1,
                "interview_arrangement_id": "1111111",
                "interview_type": 1,
                "talent_time_zone": {
                    "code": "UTC+08:00",
                    "name": {
                        "zh_cn": "亚洲/上海",
                        "en_us": "Asia/Shanghai"
                    }
                },
                "contact_user": {
                    "id": "1213213123123",
                    "name": {
                        "zh_cn": "张三",
                        "en_us": "Tony Ma"
                    }
                },
                "contact_mobile": "13333333333",
                "remark": "这是一个备注",
                "address": {
                    "id": "6949805467799537964",
                    "name": {
                        "zh_cn": "中国",
                        "en_us": "China"
                    },
                    "district": {
                        "code": "DS_1",
                        "name": {
                            "zh_cn": "武侯区",
                            "en_us": "Wu Hou"
                        }
                    },
                    "city": {
                        "code": "CT_1",
                        "name": {
                            "zh_cn": "成都",
                            "en_us": "Chengdu"
                        }
                    },
                    "state": {
                        "code": "ST_1",
                        "name": {
                            "zh_cn": "四川省",
                            "en_us": "Si Chuan"
                        }
                    },
                    "country": {
                        "code": "CN_1",
                        "name": {
                            "zh_cn": "中国",
                            "en_us": "China"
                        }
                    }
                },
                "video_type": 1,
                "arrangement_status": 1,
                "arrangement_type": 1,
                "arrangement_appointment_kind": 1,
                "meeting_room_list": [
                    {
                        "room_id": "6949805467799537964",
                        "room_name": "OCG111",
                        "building_name": "OCG",
                        "reserved_status": 1,
                        "floor_name": "17"
                    }
                ],
                "interview_round_type": {
                    "id": "1213213123123",
                    "name": {
                        "zh_cn": "第一轮",
                        "en_us": "Round 1"
                    }
                }
            }
        ],
        "has_more": true,
        "page_token": "1234452132"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetHireInterviewListRespInner>(RESP);
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