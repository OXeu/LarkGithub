//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/report/get_top_user>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::vc::VcService;

impl<'c, IStore: Store, IClient: HttpClient> VcService<'c, IStore, IClient> {
    /// **api 版本: 2024-04-29T02:58:52+00:00**
    ///
    /// ## 获取 Top 用户列表
    ///
    /// 获取一段时间内组织内会议使用的 Top 用户列表。
    ///
    /// 支持最近90天内的数据查询；默认返回前10位，最多可查询前100位
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/report/get_top_user>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/vc-v1/report/get_top_user>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Freport%2Fget_top_user>
    pub async fn get_vc_top_user_report(
        &self,
        req: GetVcTopUserReportReq,
    ) -> Result<(GetVcTopUserReportResp, CommonResponse), Error> {
        self.get_vc_top_user_report_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_vc_top_user_report](#method.get_vc_top_user_report) 函数
    pub async fn get_vc_top_user_report_with_opt(
        &self,
        req: GetVcTopUserReportReq,
        method_option: MethodOption,
    ) -> Result<(GetVcTopUserReportResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_vc_top_user_report(&req) {
                tracing::info!("[lark] Vc#GetVcTopUserReport **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#GetVcTopUserReport call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "GetVcTopUserReport",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/vc/v1/reports/get_top_user",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetVcTopUserReportRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetVcTopUserReportReq {
    /// 开始时间（unix时间，单位sec）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1608888867"
    #[api(kind = "query", name = "start_time", v_type = "var", option = "false")]
    pub start_time: String,
    /// 结束时间（unix时间，单位sec）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1608889966"
    #[api(kind = "query", name = "end_time", v_type = "var", option = "false")]
    pub end_time: String,
    /// 取前多少位
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "10"
    #[api(kind = "query", name = "limit", v_type = "var", option = "false")]
    pub limit: i64,
    /// 排序依据（降序）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `meeting_count`: 会议数量
    ///
    /// `meeting_duration`: 会议时长
    #[api(kind = "query", name = "order_by", v_type = "var", option = "false")]
    pub order_by: i64,
    /// 数据驻留地（传参前提是租户存在多个驻留地数据且开通了该查询功能）
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `CN`: 中国大陆
    ///
    /// `VA`: 美国
    ///
    /// `SG`: 新加坡
    ///
    /// `JP`: 日本
    #[api(kind = "query", name = "unit", v_type = "var", option = "false")]
    pub unit: i64,
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
struct GetVcTopUserReportRespInner {
    #[serde(flatten)]
    data: Option<GetVcTopUserReportResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetVcTopUserReportResp {
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
    /// top用户列表
    #[serde(
        rename = "top_user_report",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub top_user_report: Vec<ReportTopUserSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ReportTopUserSubResp {
    /// 用户ID
    ///
    /// **示例值**: "ou_3ec3f6a28a0d08c45d895276e8e5e19b"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 用户名
    ///
    /// **示例值**: "name"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 用户类型
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `lark_user`: 飞书用户
    ///
    /// `room_user`: rooms用户
    ///
    /// `doc_user`: 文档用户
    ///
    /// `neo_user`: neo单品用户
    ///
    /// `neo_guest_user`: neo单品游客用户
    ///
    /// `pstn_user`: pstn用户
    ///
    /// `sip_user`: sip用户
    #[serde(
        rename = "user_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub user_type: i64,
    /// 会议数量
    ///
    /// **示例值**: "100"
    #[serde(
        rename = "meeting_count",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub meeting_count: String,
    /// 会议时长（单位min）
    ///
    /// **示例值**: "3000"
    #[serde(
        rename = "meeting_duration",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub meeting_duration: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::vc::VcServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetVcTopUserReportReq) -> Result<(GetVcTopUserReportResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetVcTopUserReportReq) -> Result<(GetVcTopUserReportResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_vc_top_user_report<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetVcTopUserReportReq, GetVcTopUserReportResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_vc_top_user_report(
            &self,
            req: &GetVcTopUserReportReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetVcTopUserReportReq, GetVcTopUserReportResp, Arc<dyn MockFunc>>(
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
        api::gen::vc::get_vc_top_user_report::{
            GetVcTopUserReportReq, GetVcTopUserReportResp, GetVcTopUserReportRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_get_vc_top_user_report(|_| {
                Ok((GetVcTopUserReportResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .vc()
            .get_vc_top_user_report(GetVcTopUserReportReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .vc()
            .get_vc_top_user_report(GetVcTopUserReportReq::default())
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
        "top_user_report": [
            {
                "id": "ou_3ec3f6a28a0d08c45d895276e8e5e19b",
                "name": "name",
                "user_type": 1,
                "meeting_count": "100",
                "meeting_duration": "3000"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetVcTopUserReportRespInner>(RESP);
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
