//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation-assessment/patch>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::core_hr::CoreHrService;

impl<'c, IStore: Store, IClient: HttpClient> CoreHrService<'c, IStore, IClient> {
    /// **api 版本: 2024-02-06T02:32:54+00:00**
    ///
    /// ## 更新试用期考核信息
    ///
    /// 更新试用期的考核结果
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation-assessment/patch>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation-assessment/patch>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Fcorehr-v2%2Fprobation-assessment%2Fpatch>
    pub async fn update_core_hr_probation_assessment(
        &self,
        req: UpdateCoreHrProbationAssessmentReq,
    ) -> Result<(UpdateCoreHrProbationAssessmentResp, CommonResponse), Error> {
        self.update_core_hr_probation_assessment_with_opt(req, Default::default())
            .await
    }

    /// 参见 [update_core_hr_probation_assessment](#method.update_core_hr_probation_assessment) 函数
    pub async fn update_core_hr_probation_assessment_with_opt(
        &self,
        req: UpdateCoreHrProbationAssessmentReq,
        method_option: MethodOption,
    ) -> Result<(UpdateCoreHrProbationAssessmentResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self
                .mock()
                .get_mock_update_core_hr_probation_assessment(&req)
            {
                tracing::info!("[lark] CoreHr#UpdateCoreHrProbationAssessment **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#UpdateCoreHrProbationAssessment call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "UpdateCoreHrProbationAssessment",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v2/probation/assessments/:assessment_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (UpdateCoreHrProbationAssessmentRespInner, _) =
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
pub struct UpdateCoreHrProbationAssessmentReq {
    /// 考核结果 ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "7140964208476371331"
    #[api(kind = "path", name = "assessment_id")]
    pub assessment_id: String,
    /// 根据 client_token 是否一致来判断是否为同一请求
    ///
    /// **示例值**: "6822122262122064111"
    #[api(
        kind = "query",
        name = "client_token",
        v_type = "var",
        option = "false"
    )]
    pub client_token: String,
    /// 考核状态
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "completed"
    ///
    /// **可选值**:
    ///
    /// `not_started`: 未开始
    ///
    /// `in_process`: 进行中
    ///
    /// `completed`: 已完成
    ///
    /// `no_need`: 无需考核
    #[api(kind = "body", name = "assessment_status")]
    pub assessment_status: String,
    /// 试用期考核结果
    ///
    /// **示例值**: "approved"
    ///
    /// **可选值**:
    ///
    /// `approved`: 通过
    ///
    /// `rejected`: 不通过
    #[api(kind = "body", name = "assessment_result")]
    pub assessment_result: Option<String>,
    /// 考核得分
    ///
    /// **示例值**: "99.9"
    #[api(kind = "body", name = "assessment_score")]
    pub assessment_score: Option<f64>,
    /// 试用期考核等级，枚举值 api_name 可通过[【获取字段详情】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/custom_field/get_by_param)接口查询，查询参数如下：
    ///
    /// - object_api_name = "probation_management"
    ///
    /// - custom_api_name = "final_assessment_grade"
    ///
    /// **示例值**: "grade_a"
    #[api(kind = "body", name = "assessment_grade")]
    pub assessment_grade: Option<String>,
    /// 考核评语
    ///
    /// **示例值**: "超出预期"
    #[api(kind = "body", name = "assessment_comment")]
    pub assessment_comment: Option<String>,
    /// 考核结果页面超链接
    ///
    /// **示例值**: "暂无示例"
    #[api(kind = "body", name = "assessment_detail")]
    pub assessment_detail: Option<String>,
    /// 是否为最终考核结果
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "false"
    #[api(kind = "body", name = "is_final_asssessment")]
    pub is_final_asssessment: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct UpdateCoreHrProbationAssessmentRespInner {
    #[serde(flatten)]
    data: Option<UpdateCoreHrProbationAssessmentResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateCoreHrProbationAssessmentResp {
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

    use self::gen::core_hr::CoreHrServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            UpdateCoreHrProbationAssessmentReq,
        ) -> Result<(UpdateCoreHrProbationAssessmentResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    UpdateCoreHrProbationAssessmentReq,
                )
                    -> Result<(UpdateCoreHrProbationAssessmentResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_update_core_hr_probation_assessment<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            UpdateCoreHrProbationAssessmentReq,
            UpdateCoreHrProbationAssessmentResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_update_core_hr_probation_assessment(
            &self,
            req: &UpdateCoreHrProbationAssessmentReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                UpdateCoreHrProbationAssessmentReq,
                UpdateCoreHrProbationAssessmentResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::update_core_hr_probation_assessment::{
            UpdateCoreHrProbationAssessmentReq, UpdateCoreHrProbationAssessmentResp,
            UpdateCoreHrProbationAssessmentRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_update_core_hr_probation_assessment(|_| {
                Ok((
                    UpdateCoreHrProbationAssessmentResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .update_core_hr_probation_assessment(UpdateCoreHrProbationAssessmentReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .update_core_hr_probation_assessment(UpdateCoreHrProbationAssessmentReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "assessment_status": "completed",
    "assessment_result": "approved",
    "assessment_score": 99.9,
    "assessment_grade": "grade_a",
    "assessment_comment": "超出预期",
    "assessment_detail": "暂无示例",
    "is_final_asssessment": false
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::UpdateCoreHrProbationAssessmentReqBody>(REQ) {
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
        let res = serde_json::from_str::<UpdateCoreHrProbationAssessmentRespInner>(RESP);
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