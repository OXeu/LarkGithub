//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/pin/list>
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
    /// **api 版本: 2024-06-21T09:04:21+00:00**
    ///
    /// ## 获取群内 Pin 消息
    ///
    /// 获取所在群内指定时间范围内的所有 Pin 消息。
    ///
    /// 注意事项：
    ///
    /// - 需要开启[机器人能力](https://open.feishu.cn/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-enable-bot-ability)
    ///
    /// - 获取Pin消息时，机器人必须在群组中
    ///
    /// - 获取的Pin消息按Pin的创建时间降序排列
    ///
    /// - 接口默认限流为==50 QPS==
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/pin/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/im-v1/pin/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fim-v1%2Fpin%2Flist>
    pub async fn get_message_pin_list(
        &self,
        req: GetMessagePinListReq,
    ) -> Result<(GetMessagePinListResp, CommonResponse), Error> {
        self.get_message_pin_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_message_pin_list](#method.get_message_pin_list) 函数
    pub async fn get_message_pin_list_with_opt(
        &self,
        req: GetMessagePinListReq,
        method_option: MethodOption,
    ) -> Result<(GetMessagePinListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_message_pin_list(&req) {
                tracing::info!("[lark] Message#GetMessagePinList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Message#GetMessagePinList call api");

        let req = ApiRequest {
            scope: "Message",
            api: "GetMessagePinList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/im/v1/pins",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetMessagePinListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetMessagePinListReq {
    /// 待获取Pin消息的Chat ID，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "oc_234jsi43d3ssi993d43545f"
    #[api(kind = "query", name = "chat_id", v_type = "var", option = "false")]
    pub chat_id: String,
    /// Pin信息的起始时间（毫秒级时间戳）。若未填写默认获取到群聊内最早的Pin信息
    ///
    /// **示例值**: "1658632251800"
    #[api(kind = "query", name = "start_time", v_type = "var", option = "false")]
    pub start_time: String,
    /// Pin信息的结束时间（毫秒级时间戳）。若未填写默认从群聊内最新的Pin信息开始获取
    ///
    /// **注意**：`end_time`值应大于`start_time`值
    ///
    /// **示例值**: "1658731646425"
    #[api(kind = "query", name = "end_time", v_type = "var", option = "false")]
    pub end_time: String,
    /// 此次调用中使用的分页的大小
    ///
    /// **示例值**: "20"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: i64,
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "GxmvlNRvP0NdQZpa7yIqf_Lv_QuBwTQ8tXkX7w-irAghVD_TvuYd1aoJ1LQph86O-XImC4X9j9FhUPhXQDvtrQ=="
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetMessagePinListRespInner {
    #[serde(flatten)]
    data: Option<GetMessagePinListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetMessagePinListResp {
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
    /// Pin的操作信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<PinSubResp>,
    /// 是否还有更多项
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "GxmvlNRvP0NdQZpa7yIqf_Lv_QuBwTQ8tXkX7w-irAghVD_TvuYd1aoJ1LQph86O-XImC4X9j9FhUPhXQDvtrQ=="
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PinSubResp {
    /// Pin的消息ID
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "om_dc13264520392913993dd051dba21dcf"
    #[serde(
        rename = "message_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub message_id: String,
    /// Pin消息所在的群聊ID
    ///
    /// **示例值**: "oc_a0553eda9014c201e6969b478895c230"
    #[serde(
        rename = "chat_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub chat_id: String,
    /// Pin的操作人ID
    ///
    /// **示例值**: "ou_7d8a6e6df7621556ce0d21922b676706ccs"
    #[serde(
        rename = "operator_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_id: String,
    /// Pin的操作人ID类型。当Pin的操作人为用户时，为==open_id==；当Pin的操作人为机器人时，为==app_id==
    ///
    /// **示例值**: "open_id"
    #[serde(
        rename = "operator_id_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub operator_id_type: String,
    /// Pin的创建时间（毫秒级时间戳）
    ///
    /// **示例值**: "1615380573211"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
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
        Fn(GetMessagePinListReq) -> Result<(GetMessagePinListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetMessagePinListReq) -> Result<(GetMessagePinListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> MessageServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_message_pin_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetMessagePinListReq, GetMessagePinListResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_message_pin_list(
            &self,
            req: &GetMessagePinListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetMessagePinListReq, GetMessagePinListResp, Arc<dyn MockFunc>>(
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
        api::gen::message::get_message_pin_list::{
            GetMessagePinListReq, GetMessagePinListResp, GetMessagePinListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .message()
            .mock()
            .mock_get_message_pin_list(|_| {
                Ok((GetMessagePinListResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .message()
            .get_message_pin_list(GetMessagePinListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .message()
            .get_message_pin_list(GetMessagePinListReq::default())
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
                "message_id": "om_dc13264520392913993dd051dba21dcf",
                "chat_id": "oc_a0553eda9014c201e6969b478895c230",
                "operator_id": "ou_7d8a6e6df7621556ce0d21922b676706ccs",
                "operator_id_type": "open_id",
                "create_time": "1615380573211"
            }
        ],
        "has_more": true,
        "page_token": "GxmvlNRvP0NdQZpa7yIqf_Lv_QuBwTQ8tXkX7w-irAghVD_TvuYd1aoJ1LQph86O-XImC4X9j9FhUPhXQDvtrQ=="
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetMessagePinListRespInner>(RESP);
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