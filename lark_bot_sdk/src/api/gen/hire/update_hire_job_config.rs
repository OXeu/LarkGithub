//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/update_config>
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
    /// **api 版本: 2024-03-15T02:51:49+00:00**
    ///
    /// ## 更新职位设置
    ///
    /// 更新职位设置，包括面试评价表、Offer 申请表等。接口将按照所选择的「更新选项」进行设置参数校验和更新。若设置的必填字段更新时未填写内容，接口将报错无法完成更新
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/update_config>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/recruitment-related-configuration/job/update_config>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Frecruitment-related-configuration%2Fjob%2Fupdate_config>
    pub async fn update_hire_job_config(
        &self,
        req: UpdateHireJobConfigReq,
    ) -> Result<(UpdateHireJobConfigResp, CommonResponse), Error> {
        self.update_hire_job_config_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_hire_job_config](#method.update_hire_job_config) 函数
    pub async fn update_hire_job_config_with_opt(
        &self,
        req: UpdateHireJobConfigReq,
        method_option: MethodOption,
    ) -> Result<(UpdateHireJobConfigResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_update_hire_job_config(&req) {
                tracing::info!("[lark] Hire#UpdateHireJobConfig **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#UpdateHireJobConfig call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "UpdateHireJobConfig",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/hire/v1/jobs/:job_id/update_config",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateHireJobConfigRespInner, _) = self.cli.do_req(req).await?;
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
pub struct UpdateHireJobConfigReq {
    /// 职位 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6960663240925956660"
    #[api(kind = "path", name = "job_id")]
    pub job_id: String,
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
    /// Offer 申请表，枚举通过接口「获取 Offer 申请表列表」获取
    ///
    /// **示例值**: "6960663240925956573"
    #[api(kind = "body", name = "offer_apply_schema_id")]
    pub offer_apply_schema_id: Option<String>,
    /// Offer 审批流，枚举通过接口「获取 Offer 审批流列表」获取
    ///
    /// **示例值**: "6960663240925956572"
    #[api(kind = "body", name = "offer_process_conf")]
    pub offer_process_conf: Option<String>,
    /// 建议评估人 ID 列表
    ///
    /// **示例值**: "6960663240925956573"
    #[api(kind = "body", name = "recommended_evaluator_id_list")]
    pub recommended_evaluator_id_list: Vec<Option<String>>,
    /// 更新选项，传入要更新的配置项
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    #[api(kind = "body", name = "update_option_list")]
    pub update_option_list: Vec<Option<i64>>,
    /// 面试评价表，枚举通过接口[获取面试评价表信息](https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/interview-settings/list)获取
    ///
    /// **示例值**: "6960663240925956571"
    #[api(kind = "body", name = "assessment_template_biz_id")]
    pub assessment_template_biz_id: Option<String>,
    /// 建议面试官列表
    #[api(kind = "body", name = "interview_round_conf_list")]
    pub interview_round_conf_list: Vec<Option<JobConfigInterviewRoundConfSubReq>>,
    /// 关联招聘需求，支持关联多个，枚举通过接口「获取招聘需求」获取
    ///
    /// **示例值**: "6960663240925956572"
    #[api(kind = "body", name = "jr_id_list")]
    pub jr_id_list: Vec<Option<String>>,
    /// 面试登记表ID，当在飞书招聘「设置 - 信息登记表使用设置 - 面试登记表使用方式」中选择「HR 按职位选择登记表」时，该字段为必填；否则该字段不生效。
    ///
    /// **示例值**: "6930815272790114324"
    #[api(kind = "body", name = "interview_registration_schema_id")]
    pub interview_registration_schema_id: Option<String>,
    /// 入职登记表ID，当在飞书招聘「设置 - 信息登记表使用设置 - 入职登记表使用方式」中选择「HR 按职位选择登记表」时，该字段为必填；否则该字段不生效。
    ///
    /// **示例值**: "6930815272790114324"
    #[api(kind = "body", name = "onboard_registration_schema_id")]
    pub onboard_registration_schema_id: Option<String>,
    /// 面试轮次类型 ID 列表
    #[api(kind = "body", name = "interview_round_type_conf_list")]
    pub interview_round_type_conf_list: Vec<Option<JobConfigRoundTypeSubReq>>,
    /// 关联职位列表，如职位为实体职位则关联虚拟职位id，如职位为虚拟职位则关联实体职位id
    ///
    /// **示例值**: "6960663240925956574"
    #[api(kind = "body", name = "related_job_id_list")]
    pub related_job_id_list: Vec<Option<String>>,
    /// 自助约面配置
    #[api(kind = "body", name = "interview_appointment_config")]
    pub interview_appointment_config: Option<InterviewAppointmentConfigSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InterviewAppointmentConfigSubReq {
    /// 是否开启面试官自助约面
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "enable_interview_appointment_by_interviewer",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enable_interview_appointment_by_interviewer: Option<bool>,
    /// 配置详情
    #[serde(
        rename = "config",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub config: Option<InterviewAppointmentConfigContentSubReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobConfigInterviewRoundConfSubReq {
    /// 建议面试官 ID 列表
    ///
    /// **示例值**: "6960663240925956571"
    #[serde(
        rename = "interviewer_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interviewer_id_list: Vec<Option<String>>,
    /// 面试轮次
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "round",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub round: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobConfigRoundTypeSubReq {
    /// 面试轮次类型业务 ID
    ///
    /// **示例值**: "7012129842917837100"
    #[serde(
        rename = "round_biz_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub round_biz_id: Option<String>,
    /// 面试评价表业务 ID
    ///
    /// **示例值**: "6960663240925956632"
    #[serde(
        rename = "assessment_template_biz_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub assessment_template_biz_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InterviewAppointmentConfigContentSubReq {
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
    /// `视屏面试`: 视频面试
    #[serde(
        rename = "interview_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_type: Option<i64>,
    /// 时区
    ///
    /// **示例值**: "Asia/Shanghai"
    #[serde(
        rename = "talent_timezone_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub talent_timezone_code: Option<String>,
    /// 联系人id
    ///
    /// **示例值**: "ou_c99c5f35d542efc7ee492afe11af19ef"
    #[serde(
        rename = "contact_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contact_user_id: Option<String>,
    /// 联系人电话
    ///
    /// **示例值**: "151********"
    #[serde(
        rename = "contact_mobile",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contact_mobile: Option<String>,
    /// 联系人邮箱
    ///
    /// **示例值**: "test@email"
    #[serde(
        rename = "contact_email",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contact_email: Option<String>,
    /// 地址id
    ///
    /// **示例值**: "6960663240925956576"
    #[serde(
        rename = "address_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub address_id: Option<String>,
    /// 地址id
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `zoom`: zoom
    ///
    /// `牛客技术类型`: 牛客技术类型
    ///
    /// `牛客非技术类型`: 牛客非技术类型
    ///
    /// `赛码`: 赛码
    ///
    /// `飞书`: 飞书
    ///
    /// `Hackerrank`: Hackerrank
    ///
    /// `飞书(含代码考核)`: 飞书(含代码考核)
    ///
    /// `不使用系统工具`: 不使用系统工具
    #[serde(
        rename = "video_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub video_type: Option<i64>,
    /// 抄送人id list
    ///
    /// **示例值**: "ou_c99c5f35d542efc7ee492afe11af19ef"
    #[serde(
        rename = "cc",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cc: Vec<Option<String>>,
    /// 备注
    ///
    /// **示例值**: "备注"
    #[serde(
        rename = "remark",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub remark: Option<String>,
    /// 面试通知模板
    ///
    /// **示例值**: "6960663240925956573"
    #[serde(
        rename = "interview_notification_template_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_notification_template_id: Option<String>,
    /// 预约通知模板
    ///
    /// **示例值**: "6960663240925956573"
    #[serde(
        rename = "appointment_notification_template_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub appointment_notification_template_id: Option<String>,
    /// 取消面试通知
    ///
    /// **示例值**: "6960663240925956573"
    #[serde(
        rename = "cancel_interview_notification_template_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cancel_interview_notification_template_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateHireJobConfigRespInner {
    #[serde(flatten)]
    data: Option<UpdateHireJobConfigResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateHireJobConfigResp {
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
    /// 职位信息
    #[serde(
        rename = "job_config",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_config: JobConfigResultSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobConfigResultSubResp {
    /// Offer 申请表
    #[serde(
        rename = "offer_apply_schema",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub offer_apply_schema: IdNameObjectSubResp,
    /// Offer 审批流
    #[serde(
        rename = "offer_process_conf",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub offer_process_conf: IdNameObjectSubResp,
    /// 建议评估人列表
    #[serde(
        rename = "recommended_evaluator_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub recommended_evaluator_list: Vec<IdNameObjectSubResp>,
    /// 面试评价表
    #[serde(
        rename = "assessment_template",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub assessment_template: IdNameObjectSubResp,
    /// 职位 ID
    ///
    /// **示例值**: "6960663240925956574"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 建议面试官列表
    #[serde(
        rename = "interview_round_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_round_list: Vec<JobConfigInterviewRoundSubResp>,
    /// 招聘需求
    #[serde(
        rename = "job_requirement_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_requirement_list: Vec<IdNameObjectSubResp>,
    /// 面试登记表
    #[serde(
        rename = "interview_registration",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_registration: RegistrationInfoSubResp,
    /// 入职登记表
    #[serde(
        rename = "onboard_registration",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub onboard_registration: RegistrationInfoSubResp,
    /// 面试轮次类型列表
    #[serde(
        rename = "interview_round_type_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_round_type_list: Vec<JobConfigRoundTypeResultSubResp>,
    /// 关联职位列表
    #[serde(
        rename = "related_job_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub related_job_list: Vec<IdNameObjectSubResp>,
    /// 职位属性，1是实体职位，2是虚拟职位
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "job_attribute",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub job_attribute: i64,
    /// 面试官安排面试配置
    #[serde(
        rename = "interview_appointment_config",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_appointment_config: InterviewAppointmentConfigSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct IdNameObjectSubResp {
    /// ID
    ///
    /// **示例值**: "1213213123123"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: I18nSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RegistrationInfoSubResp {
    /// 面试登记表ID
    ///
    /// **示例值**: "6930815272790114324"
    #[serde(
        rename = "schema_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub schema_id: String,
    /// 面试登记表名称
    ///
    /// **示例值**: "默认登记表"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InterviewAppointmentConfigSubResp {
    /// 是否开启面试官安排面试
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "enable_interview_appointment_by_interviewer",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enable_interview_appointment_by_interviewer: bool,
    /// 配置详情
    #[serde(
        rename = "config",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub config: InterviewAppointmentConfigContentSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 中文
    ///
    /// **示例值**: "测试"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 英文
    ///
    /// **示例值**: "test"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobConfigInterviewRoundSubResp {
    /// 面试官列表
    #[serde(
        rename = "interviewer_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interviewer_list: Vec<IdNameObjectSubResp>,
    /// 面试轮次
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "round",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub round: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct JobConfigRoundTypeResultSubResp {
    /// 面试轮次类型
    #[serde(
        rename = "assessment_round",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub assessment_round: IdNameObjectSubResp,
    /// 面试评价表
    #[serde(
        rename = "assessment_template",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub assessment_template: IdNameObjectSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct InterviewAppointmentConfigContentSubResp {
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
    /// `视屏面试`: 视频面试
    #[serde(
        rename = "interview_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_type: i64,
    /// 时区
    ///
    /// **示例值**: "Asia/Shanghai"
    #[serde(
        rename = "talent_timezone_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub talent_timezone_code: String,
    /// 联系人id
    ///
    /// **示例值**: "ou_c99c5f35d542efc7ee492afe11af19ef"
    #[serde(
        rename = "contact_user_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contact_user_id: String,
    /// 联系人电话
    ///
    /// **示例值**: "177xxxx1773"
    #[serde(
        rename = "contact_mobile",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contact_mobile: String,
    /// 联系人邮箱
    ///
    /// **示例值**: "test@open.com"
    #[serde(
        rename = "contact_email",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contact_email: String,
    /// 地址id
    ///
    /// **示例值**: "6960663240925956576"
    #[serde(
        rename = "address_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub address_id: String,
    /// 视频面试类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `zoom`: zoom
    ///
    /// `牛客技术类型`: 牛客技术类型
    ///
    /// `牛客非技术类型`: 牛客非技术类型
    ///
    /// `赛码`: 赛码
    ///
    /// `飞书`: 飞书
    ///
    /// `Hackerrank`: Hackerrank
    ///
    /// `飞书(含代码考核)`: 飞书(含代码考核)
    ///
    /// `不使用系统工具`: 不使用系统工具
    #[serde(
        rename = "video_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub video_type: i64,
    /// 抄送人id list
    #[serde(
        rename = "cc",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cc: Vec<String>,
    /// 备注
    ///
    /// **示例值**: "备注"
    #[serde(
        rename = "remark",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub remark: String,
    /// 面试通知模板
    ///
    /// **示例值**: "6960663240925956573"
    #[serde(
        rename = "interview_notification_template_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub interview_notification_template_id: String,
    /// 预约通知模板
    ///
    /// **示例值**: "6960663240925956573"
    #[serde(
        rename = "appointment_notification_template_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub appointment_notification_template_id: String,
    /// 取消面试通知
    ///
    /// **示例值**: "6960663240925956573"
    #[serde(
        rename = "cancel_interview_notification_template_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub cancel_interview_notification_template_id: String,
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
        Fn(UpdateHireJobConfigReq) -> Result<(UpdateHireJobConfigResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateHireJobConfigReq,
                ) -> Result<(UpdateHireJobConfigResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_hire_job_config<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, UpdateHireJobConfigReq, UpdateHireJobConfigResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_hire_job_config(
            &self,
            req: &UpdateHireJobConfigReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, UpdateHireJobConfigReq, UpdateHireJobConfigResp, Arc<dyn MockFunc>>(
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
        api::gen::hire::update_hire_job_config::{
            UpdateHireJobConfigReq, UpdateHireJobConfigResp, UpdateHireJobConfigRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_update_hire_job_config(|_| {
                Ok((
                    UpdateHireJobConfigResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .update_hire_job_config(UpdateHireJobConfigReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .update_hire_job_config(UpdateHireJobConfigReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "offer_apply_schema_id": "6960663240925956573",
    "offer_process_conf": "6960663240925956572",
    "recommended_evaluator_id_list": [
        "6966533137982392320"
    ],
    "update_option_list": [
        6
    ],
    "assessment_template_biz_id": "6960663240925956571",
    "interview_round_conf_list": [
        {
            "interviewer_id_list": [
                "6960663240925956571"
            ],
            "round": 1
        }
    ],
    "jr_id_list": [
        "6966533137982392320"
    ],
    "interview_registration_schema_id": "6930815272790114324",
    "onboard_registration_schema_id": "6930815272790114324",
    "interview_round_type_conf_list": [
        {
            "round_biz_id": "7012129842917837100",
            "assessment_template_biz_id": "6960663240925956632"
        }
    ],
    "related_job_id_list": [
        "6966533137982392320"
    ],
    "interview_appointment_config": {
        "enable_interview_appointment_by_interviewer": true,
        "config": {
            "interview_type": 1,
            "talent_timezone_code": "Asia/Shanghai",
            "contact_user_id": "ou_c99c5f35d542efc7ee492afe11af19ef",
            "contact_mobile": "151********",
            "contact_email": "test@email",
            "address_id": "6960663240925956576",
            "video_type": 1,
            "cc": [
                "6930815272790114324"
            ],
            "remark": "备注",
            "interview_notification_template_id": "6960663240925956573",
            "appointment_notification_template_id": "6960663240925956573",
            "cancel_interview_notification_template_id": "6960663240925956573"
        }
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateHireJobConfigReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "job_config": {
            "offer_apply_schema": {
                "id": "1213213123123",
                "name": {
                    "zh_cn": "测试",
                    "en_us": "test"
                }
            },
            "offer_process_conf": {
                "id": "1213213123123",
                "name": {
                    "zh_cn": "测试",
                    "en_us": "test"
                }
            },
            "recommended_evaluator_list": [
                {
                    "id": "1213213123123",
                    "name": {
                        "zh_cn": "测试",
                        "en_us": "test"
                    }
                }
            ],
            "assessment_template": {
                "id": "1213213123123",
                "name": {
                    "zh_cn": "测试",
                    "en_us": "test"
                }
            },
            "id": "6960663240925956574",
            "interview_round_list": [
                {
                    "interviewer_list": [
                        {
                            "id": "1213213123123",
                            "name": {
                                "zh_cn": "测试",
                                "en_us": "test"
                            }
                        }
                    ],
                    "round": 1
                }
            ],
            "job_requirement_list": [
                {
                    "id": "1213213123123",
                    "name": {
                        "zh_cn": "测试",
                        "en_us": "test"
                    }
                }
            ],
            "interview_registration": {
                "schema_id": "6930815272790114324",
                "name": "默认登记表"
            },
            "onboard_registration": {
                "schema_id": "6930815272790114324",
                "name": "默认登记表"
            },
            "interview_round_type_list": [
                {
                    "assessment_round": {
                        "id": "1213213123123",
                        "name": {
                            "zh_cn": "测试",
                            "en_us": "test"
                        }
                    },
                    "assessment_template": {
                        "id": "1213213123123",
                        "name": {
                            "zh_cn": "测试",
                            "en_us": "test"
                        }
                    }
                }
            ],
            "related_job_list": [
                {
                    "id": "1213213123123",
                    "name": {
                        "zh_cn": "测试",
                        "en_us": "test"
                    }
                }
            ],
            "job_attribute": 1,
            "interview_appointment_config": {
                "enable_interview_appointment_by_interviewer": true,
                "config": {
                    "interview_type": 1,
                    "talent_timezone_code": "Asia/Shanghai",
                    "contact_user_id": "ou_c99c5f35d542efc7ee492afe11af19ef",
                    "contact_mobile": "177xxxx1773",
                    "contact_email": "test@open.com",
                    "address_id": "6960663240925956576",
                    "video_type": 1,
                    "cc": [
                        "ou_c99c5f35d542efc7ee492afe11af19ef"
                    ],
                    "remark": "备注",
                    "interview_notification_template_id": "6960663240925956573",
                    "appointment_notification_template_id": "6960663240925956573",
                    "cancel_interview_notification_template_id": "6960663240925956573"
                }
            }
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<UpdateHireJobConfigRespInner>(RESP);
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