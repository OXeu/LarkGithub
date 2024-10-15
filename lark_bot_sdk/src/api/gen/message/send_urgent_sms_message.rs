//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/urgent_sms>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::message::MessageService;

impl<'c, IStore: Store, IClient: HttpClient> MessageService<'c, IStore, IClient> {
    /// **api 版本: 2024-06-21T09:04:24+00:00**
    ///
    /// ## 发送短信加急
    ///
    /// 对指定消息进行应用内加急与短信加急。
    ///
    /// 特别说明：
    ///
    /// - 通过接口产生的短信加急将消耗企业的加急额度，请慎重调用
    ///
    /// - 通过[租户管理后台](https://admin.feishu.cn/)-费用中心-短信/电话加急 可以查看当前额度
    ///
    /// 注意事项:
    ///
    /// - 需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
    ///
    /// - 只能加急机器人自己发送的消息
    ///
    /// - 加急时机器人仍需要在加急消息所在的群组中
    ///
    /// - 加急用户的未读加急总数不能超过200条
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/urgent_sms>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/im-v1/buzz-messages/urgent_sms>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fim-v1%2Fbuzz-messages%2Furgent_sms>
    pub async fn send_urgent_sms_message(
        &self,
        req: SendUrgentSmsMessageReq,
    ) -> Result<(SendUrgentSmsMessageResp, CommonResponse), Error> {
        self.send_urgent_sms_message_with_opt(req, Default::default())
            .await
    }

    /// 参见 [send_urgent_sms_message](#method.send_urgent_sms_message) 函数
    pub async fn send_urgent_sms_message_with_opt(
        &self,
        req: SendUrgentSmsMessageReq,
        method_option: MethodOption,
    ) -> Result<(SendUrgentSmsMessageResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_send_urgent_sms_message(&req) {
                tracing::info!("[lark] Message#SendUrgentSmsMessage **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Message#SendUrgentSmsMessage call api");

        let req = ApiRequest {
            scope: "Message",
            api: "SendUrgentSmsMessage",
            method: http::Method::PATCH,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/im/v1/messages/:message_id/urgent_sms",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (SendUrgentSmsMessageRespInner, _) = self.cli.do_req(req).await?;
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
pub struct SendUrgentSmsMessageReq {
    /// 待加急的消息ID，详情参见[消息ID说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/intro#ac79c1c2)
    ///
    /// **注意**：不支持批量消息ID（bm_xxx）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "om_dc13264520392913993dd051dba21dcf"
    #[api(kind = "path", name = "message_id")]
    pub message_id: String,
    /// 用户 ID 类型
    ///
    /// **是否必填**: 是
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
    /// 目标用户的ID，列表不可为空；推荐使用 OpenID，获取方式可参考文档[如何获取 Open ID？](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-obtain-openid)
    ///
    /// **注意**：
    ///
    /// 请确保所填的用户ID正确，并且用户在加急消息所在的群组中
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "["ou_6yf8af6bgb9100449565764t3382b168"]"
    #[api(kind = "body", name = "user_id_list")]
    pub user_id_list: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct SendUrgentSmsMessageRespInner {
    #[serde(flatten)]
    data: Option<SendUrgentSmsMessageResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SendUrgentSmsMessageResp {
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
    /// 无效的用户ID
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "invalid_user_id_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub invalid_user_id_list: Vec<String>,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::message::MessageServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(SendUrgentSmsMessageReq) -> Result<(SendUrgentSmsMessageResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    SendUrgentSmsMessageReq,
                ) -> Result<(SendUrgentSmsMessageResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MessageServiceMocker<'c, IStore, IClient> {
        pub fn mock_send_urgent_sms_message<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            SendUrgentSmsMessageReq,
            SendUrgentSmsMessageResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_send_urgent_sms_message(
            &self,
            req: &SendUrgentSmsMessageReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, SendUrgentSmsMessageReq, SendUrgentSmsMessageResp, Arc<dyn MockFunc>>(
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
        api::gen::message::send_urgent_sms_message::{
            SendUrgentSmsMessageReq, SendUrgentSmsMessageResp, SendUrgentSmsMessageRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .message()
            .mock()
            .mock_send_urgent_sms_message(|_| {
                Ok((
                    SendUrgentSmsMessageResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .message()
            .send_urgent_sms_message(SendUrgentSmsMessageReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .message()
            .send_urgent_sms_message(SendUrgentSmsMessageReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "user_id_list": [
        "ou_6yf8af6bgb9100449565764t3382b168"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::SendUrgentSmsMessageReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "invalid_user_id_list": [
            "ou_6yf8af6bgb9100449565764t3382b168"
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<SendUrgentSmsMessageRespInner>(RESP);
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
