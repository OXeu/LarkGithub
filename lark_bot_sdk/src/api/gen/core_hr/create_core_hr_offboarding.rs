//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/offboarding/submit>
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
    /// **api 版本: 2024-07-15T11:21:53+00:00**
    ///
    /// ## 操作员工离职
    ///
    /// 该接口用于发起员工离职。若发起成功，会生成一条员工的离职数据，同时产生相应的事件。参考[离职申请状态变更](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/offboarding/events/status_updated)
    ///
    /// 该接口暂不支持员工数据鉴权，拥有接口权限即可操作对应租户所有员工的离职，使用时请注意数据安全。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/offboarding/submit>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/offboarding/submit>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Foffboarding%2Fsubmit>
    pub async fn create_core_hr_offboarding(
        &self,
        req: CreateCoreHrOffboardingReq,
    ) -> Result<(CreateCoreHrOffboardingResp, CommonResponse), Error> {
        self.create_core_hr_offboarding_with_opt(req, Default::default())
            .await
    }

    /// 参见 [create_core_hr_offboarding](#method.create_core_hr_offboarding) 函数
    pub async fn create_core_hr_offboarding_with_opt(
        &self,
        req: CreateCoreHrOffboardingReq,
        method_option: MethodOption,
    ) -> Result<(CreateCoreHrOffboardingResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_create_core_hr_offboarding(&req) {
                tracing::info!("[lark] CoreHr#CreateCoreHrOffboarding **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#CreateCoreHrOffboarding call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "CreateCoreHrOffboarding",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v1/offboardings/submit",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (CreateCoreHrOffboardingRespInner, _) =
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
pub struct CreateCoreHrOffboardingReq {
    /// 用户 ID 类型
    ///
    /// **示例值**: "people_corehr_id"
    ///
    /// **可选值**:
    ///
    /// `open_id`: 标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。[了解更多：如何获取 Open ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// `union_id`: 标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。[了解更多：如何获取 Union ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)
    ///
    /// `user_id`: 标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。[了解更多：如何获取 User ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)
    ///
    /// `people_corehr_id`: 以飞书人事的 ID 来识别用户
    #[api(
        kind = "query",
        name = "user_id_type",
        v_type = "var",
        option = "false"
    )]
    pub user_id_type: String,
    /// 离职方式，目前只支持直接离职
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `Termination_of_dismissal`: 直接离职
    #[api(kind = "body", name = "offboarding_mode")]
    pub offboarding_mode: i64,
    /// 雇员 ID。ID 类型与查询参数 user_id_type 的取值一致。例如，当user_id_type为user_id时，该字段取员工的user_id，若user_id_type为people_corehr_id时，则取该员工的人事雇佣ID。获取员工对应ID参考[如何获取自己的 User ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)、[如何获取自己的 Union ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)、[如何获取自己的雇佣 ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)。员工的人事雇佣ID需要先获取User ID后，通过[ID 转换](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/common_data-id/convert)获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6982509313466189342"
    #[api(kind = "body", name = "employment_id")]
    pub employment_id: String,
    /// 离职日期，入参格式应为YYYY-MM-DD
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2022-05-18"
    #[api(kind = "body", name = "offboarding_date")]
    pub offboarding_date: String,
    /// 离职原因，可通过接口
    ///
    /// [【查询员工离职原因列表】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/offboarding/query)获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "reason_for_offboarding_option8"
    #[api(kind = "body", name = "offboarding_reason_unique_identifier")]
    pub offboarding_reason_unique_identifier: String,
    /// 离职原因说明，长度限制6000个字符，该字段允许为空
    ///
    /// **示例值**: "离职原因说明"
    #[api(kind = "body", name = "offboarding_reason_explanation")]
    pub offboarding_reason_explanation: Option<String>,
    /// 操作发起人 ID。取值逻辑与雇佣ID保持一致，即当user_id_type为user_id时，该字段取员工的user_id，若user_id_type为people_corehr_id时，则取该员工的人事雇佣ID。获取员工对应ID参考[如何获取自己的 User ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-user-id)、[如何获取自己的 Union ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-union-id)、[如何获取自己的雇佣 ID](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)。员工的人事雇佣ID需要先获取User ID后，通过[ID 转换](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/common_data-id/convert)获取
    ///
    /// 注意：
    ///
    /// 1.只有操作发起人可以撤销流程
    ///
    /// 2.为空时，默认系统发起人
    ///
    /// **示例值**: "6982509313466189341"
    #[api(kind = "body", name = "initiator_id")]
    pub initiator_id: Option<String>,
    /// 是否加入离职屏蔽名单
    ///
    /// 注意：
    ///
    /// 1.取值为true时，屏蔽原因（block_reason）为必填。
    ///
    /// 2.取值为false时，不允许填写屏蔽原因（block_reason）和屏蔽原因说明（block_reason_explanation）。
    ///
    /// 3.取值为空时，不允许填写屏蔽原因（block_reason）和屏蔽原因说明（block_reason_explanation）。
    ///
    /// 4.操作离职时如果选择加入屏蔽名单，只有当员工离职生效后才会进入到屏蔽名单。
    ///
    /// **示例值**: "false"
    #[api(kind = "body", name = "add_block_list")]
    pub add_block_list: Option<bool>,
    /// 屏蔽原因
    ///
    /// 注意：
    ///
    /// 1.该字段取值于 [人员档案配置](https://people.feishu.cn/people/hr-settings/profile) > 信息配置 > 离职信息 的屏蔽原因字段选项集。
    ///
    /// 2.枚举字段值也可通过[获取字段详情](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/custom_field/get_by_param)获取，参考接口返回的 字段详情 > 字段类型配置信息 > 选项配置信息 > 选项信息 > 枚举常量集 API name
    ///
    /// 3.该字段是否必填取决于是否加入离职屏蔽名单(add_block_list)
    ///
    /// **示例值**: "红线"
    #[api(kind = "body", name = "block_reason")]
    pub block_reason: Option<String>,
    /// 屏蔽原因说明，该字段允许为空
    ///
    /// **示例值**: "xx 年 xx 月 xx 日因 xx 原因红线"
    #[api(kind = "body", name = "block_reason_explanation")]
    pub block_reason_explanation: Option<String>,
    /// 离职自定义字段。
    ///
    /// 注意：可填写的字段范围参考[人员档案配置](https://people.feishu.cn/people/hr-settings/profile) > 信息配置 > 离职信息 中的自定义字段
    #[api(kind = "body", name = "custom_fields")]
    pub custom_fields: Vec<Option<ObjectFieldDataSubReq>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ObjectFieldDataSubReq {
    /// 字段唯一标识
    ///
    /// 注意：
    ///
    /// 1.该字段取值于[人员档案配置](https://people.feishu.cn/people/hr-settings/profile) > 信息配置 > 离职信息 中各字段的字段编码
    ///
    /// 2.该字段也可以通过[获取自定义字段列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/custom_field/query)获取
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "name"
    #[serde(
        rename = "field_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub field_name: String,
    /// 字段值，是json转义后的字符串，根据元数据定义不同，字段格式不同(如123, 123.23, "true", [\"id1\",\"id2\"], "2006-01-02 15:04:05")。
    ///
    /// 注意：
    ///
    /// 1.枚举字段的枚举值取值于[人员档案配置](https://people.feishu.cn/people/hr-settings/profile) > 信息配置 > 离职信息 对应字段选项集的选项编码。
    ///
    /// 2.枚举字段值也可通过[获取字段详情](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/custom_field/get_by_param)获取，参考接口返回的 字段详情 > 字段类型配置信息 > 选项配置信息 > 选项信息 > 枚举常量集 API name
    ///
    /// 3.人员字段目前只支持传入员工的雇佣ID。员工的人事雇佣ID需要先获取User ID后，通过[ID 转换](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/common_data-id/convert)获取
    ///
    /// 4.暂不支持填写附件类型字段。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "\"Sandy\""
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct CreateCoreHrOffboardingRespInner {
    #[serde(flatten)]
    data: Option<CreateCoreHrOffboardingResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateCoreHrOffboardingResp {
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
    /// 离职记录 id
    ///
    /// **示例值**: "7095671727698478604"
    #[serde(
        rename = "offboarding_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub offboarding_id: String,
    /// 雇员 id
    ///
    /// **示例值**: "6982509313466189342"
    #[serde(
        rename = "employment_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub employment_id: String,
    /// 离职原因
    ///
    /// **示例值**: "reason_for_offboarding_option8"
    #[serde(
        rename = "offboarding_reason_unique_identifier",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub offboarding_reason_unique_identifier: String,
    /// 离职日期
    ///
    /// **示例值**: "2022-05-18"
    #[serde(
        rename = "offboarding_date",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub offboarding_date: String,
    /// 离职原因说明
    ///
    /// **示例值**: "离职原因说明"
    #[serde(
        rename = "offboarding_reason_explanation",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub offboarding_reason_explanation: String,
    /// 是否加入离职屏蔽名单
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "add_block_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub add_block_list: bool,
    /// 屏蔽原因
    ///
    /// **示例值**: "红线"
    #[serde(
        rename = "block_reason",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub block_reason: String,
    /// 屏蔽原因说明
    ///
    /// **示例值**: "xx 年 xx 月 xx 日因 xx 原因红线"
    #[serde(
        rename = "block_reason_explanation",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub block_reason_explanation: String,
    /// 创建时间
    ///
    /// **示例值**: "2022-05-09 17:50:17"
    #[serde(
        rename = "created_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub created_time: String,
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
            CreateCoreHrOffboardingReq,
        ) -> Result<(CreateCoreHrOffboardingResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    CreateCoreHrOffboardingReq,
                ) -> Result<(CreateCoreHrOffboardingResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_create_core_hr_offboarding<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            CreateCoreHrOffboardingReq,
            CreateCoreHrOffboardingResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_create_core_hr_offboarding(
            &self,
            req: &CreateCoreHrOffboardingReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                CreateCoreHrOffboardingReq,
                CreateCoreHrOffboardingResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::create_core_hr_offboarding::{
            CreateCoreHrOffboardingReq, CreateCoreHrOffboardingResp,
            CreateCoreHrOffboardingRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_create_core_hr_offboarding(|_| {
                Ok((
                    CreateCoreHrOffboardingResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .create_core_hr_offboarding(CreateCoreHrOffboardingReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .create_core_hr_offboarding(CreateCoreHrOffboardingReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "offboarding_mode": 1,
    "employment_id": "6982509313466189342",
    "offboarding_date": "2022-05-18",
    "offboarding_reason_unique_identifier": "reason_for_offboarding_option8",
    "offboarding_reason_explanation": "离职原因说明",
    "initiator_id": "6982509313466189341",
    "add_block_list": false,
    "block_reason": "红线",
    "block_reason_explanation": "xx 年 xx 月 xx 日因 xx 原因红线",
    "custom_fields": [
        {
            "field_name": "name",
            "value": "\"Sandy\""
        }
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::CreateCoreHrOffboardingReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "offboarding_id": "7095671727698478604",
        "employment_id": "6982509313466189342",
        "offboarding_reason_unique_identifier": "reason_for_offboarding_option8",
        "offboarding_date": "2022-05-18",
        "offboarding_reason_explanation": "离职原因说明",
        "add_block_list": false,
        "block_reason": "红线",
        "block_reason_explanation": "xx 年 xx 月 xx 日因 xx 原因红线",
        "created_time": "2022-05-09 17:50:17"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<CreateCoreHrOffboardingRespInner>(RESP);
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
