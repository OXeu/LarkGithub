//! doc url: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/evaluation/list>
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
    /// **api 版本: 2023-11-02T08:49:38+00:00**
    ///
    /// ## 获取简历评估信息
    ///
    /// 获取简历评估信息。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/evaluation/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/hire-v1/candidate-management/delivery-process-management/evaluation/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fhire-v1%2Fcandidate-management%2Fdelivery-process-management%2Fevaluation%2Flist>
    pub async fn get_hire_evaluation_list(
        &self,
        req: GetHireEvaluationListReq,
    ) -> Result<(GetHireEvaluationListResp, CommonResponse), Error> {
        self.get_hire_evaluation_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_hire_evaluation_list](#method.get_hire_evaluation_list) 函数
    pub async fn get_hire_evaluation_list_with_opt(
        &self,
        req: GetHireEvaluationListReq,
        method_option: MethodOption,
    ) -> Result<(GetHireEvaluationListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_hire_evaluation_list(&req) {
                tracing::info!("[lark] Hire#GetHireEvaluationList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Hire#GetHireEvaluationList call api");

        let req = ApiRequest {
            scope: "Hire",
            api: "GetHireEvaluationList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/hire/v1/evaluations",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetHireEvaluationListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetHireEvaluationListReq {
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "eyJvZmZzZXQiOjEsInRpbWVzdGFtcCI6MTY0MDc2NTYzMjA4OCwiaWQiOm51bGx9"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 每页获取记录数量，最大100
    ///
    /// **示例值**: "100"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 投递 ID
    ///
    /// **示例值**: "6875569957036738823"
    #[api(
        kind = "query",
        name = "application_id",
        v_type = "var",
        option = "false"
    )]
    pub application_id: String,
    /// 最早更新时间，毫秒级时间戳
    ///
    /// **示例值**: "1600843767338"
    #[api(
        kind = "query",
        name = "update_start_time",
        v_type = "var",
        option = "false"
    )]
    pub update_start_time: String,
    /// 最晚更新时间，毫秒级时间戳
    ///
    /// **示例值**: "1600843938726"
    #[api(
        kind = "query",
        name = "update_end_time",
        v_type = "var",
        option = "false"
    )]
    pub update_end_time: String,
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
    ///
    /// `people_admin_id`: 以people_admin_id来识别用户
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetHireEvaluationListRespInner {
    #[serde(flatten)]
    data: Option<GetHireEvaluationListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetHireEvaluationListResp {
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
    /// **示例值**: "eyJvZmZzZXQiOjEsInRpbWVzdGFtcCI6MTY0MDc2NTYzMjA4OCwiaWQiOm51bGx9"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
    /// 简历评估信息列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<EvaluationSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EvaluationSubResp {
    /// 评估 ID
    ///
    /// **示例值**: "6875295756292425998"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 投递 ID
    ///
    /// **示例值**: "6875569957036738823"
    #[serde(
        rename = "application_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub application_id: String,
    /// 投递阶段
    ///
    /// **示例值**: "6784315427607595268"
    #[serde(
        rename = "stage_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub stage_id: String,
    /// 创建人user_id
    ///
    /// **示例值**: "ou_aaf83d1b2c856ead36aa9a38784b9a5c"
    #[serde(
        rename = "creator_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub creator_id: String,
    /// 评估人user_id
    ///
    /// **示例值**: "ou_aaf83d1b2c856ead36aa9a38784b9a5c"
    #[serde(
        rename = "evaluator_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub evaluator_id: String,
    /// 提交状态
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `已提交`: 已提交
    ///
    /// `未提交`: 未提交
    #[serde(
        rename = "commit_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub commit_status: i64,
    /// 评估结论
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `通过`: 通过
    ///
    /// `未通过`: 未通过
    #[serde(
        rename = "conclusion",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub conclusion: i64,
    /// 评估详情
    ///
    /// **示例值**: "这位同学很优秀"
    #[serde(
        rename = "content",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub content: String,
    /// 创建时间
    ///
    /// **示例值**: "1600843767338"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 最近更新时间
    ///
    /// **示例值**: "1600843937733"
    #[serde(
        rename = "update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_time: String,
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
        Fn(GetHireEvaluationListReq) -> Result<(GetHireEvaluationListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetHireEvaluationListReq,
                ) -> Result<(GetHireEvaluationListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> HireServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_hire_evaluation_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetHireEvaluationListReq,
            GetHireEvaluationListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_hire_evaluation_list(
            &self,
            req: &GetHireEvaluationListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetHireEvaluationListReq, GetHireEvaluationListResp, Arc<dyn MockFunc>>(
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
        api::gen::hire::get_hire_evaluation_list::{
            GetHireEvaluationListReq, GetHireEvaluationListResp, GetHireEvaluationListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .hire()
            .mock()
            .mock_get_hire_evaluation_list(|_| {
                Ok((
                    GetHireEvaluationListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .hire()
            .get_hire_evaluation_list(GetHireEvaluationListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .hire()
            .get_hire_evaluation_list(GetHireEvaluationListReq::default())
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
        "has_more": true,
        "page_token": "eyJvZmZzZXQiOjEsInRpbWVzdGFtcCI6MTY0MDc2NTYzMjA4OCwiaWQiOm51bGx9",
        "items": [
            {
                "id": "6875295756292425998",
                "application_id": "6875569957036738823",
                "stage_id": "6784315427607595268",
                "creator_id": "ou_aaf83d1b2c856ead36aa9a38784b9a5c",
                "evaluator_id": "ou_aaf83d1b2c856ead36aa9a38784b9a5c",
                "commit_status": 1,
                "conclusion": 1,
                "content": "这位同学很优秀",
                "create_time": "1600843767338",
                "update_time": "1600843937733"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetHireEvaluationListRespInner>(RESP);
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