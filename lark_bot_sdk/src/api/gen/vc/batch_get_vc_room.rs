//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room/mget>
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
    /// **api 版本: 2024-07-23T07:32:59+00:00**
    ///
    /// ## 批量查询会议室详情
    ///
    /// 该接口可以使用会议室 ID 批量查询会议室详情。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room/mget>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/vc-v1/room/mget>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Froom%2Fmget>
    pub async fn batch_get_vc_room(
        &self,
        req: BatchGetVcRoomReq,
    ) -> Result<(BatchGetVcRoomResp, CommonResponse), Error> {
        self.batch_get_vc_room_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_get_vc_room](#method.batch_get_vc_room) 函数
    pub async fn batch_get_vc_room_with_opt(
        &self,
        req: BatchGetVcRoomReq,
        method_option: MethodOption,
    ) -> Result<(BatchGetVcRoomResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_get_vc_room(&req) {
                tracing::info!("[lark] Vc#BatchGetVcRoom **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#BatchGetVcRoom call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "BatchGetVcRoom",
            method: http::Method::POST,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/vc/v1/rooms/mget",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchGetVcRoomRespInner, _) = self.cli.do_req(req).await?;
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
pub struct BatchGetVcRoomReq {
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
    /// 会议室id列表
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "["omm_4de32cf10a4358788ff4e09e37ebbf9b","omm_3c5dd7e09bac0c1758fcf9511bd1a771"]"
    #[api(kind = "body", name = "room_ids")]
    pub room_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchGetVcRoomRespInner {
    #[serde(flatten)]
    data: Option<BatchGetVcRoomResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetVcRoomResp {
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
    /// 会议室列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<RoomSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RoomSubResp {
    /// 会议室ID
    ///
    /// **示例值**: "omm_4de32cf10a4358788ff4e09e37ebbf9b"
    #[serde(
        rename = "room_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub room_id: String,
    /// 会议室名称
    ///
    /// **示例值**: "测试会议室"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 会议室能容纳的人数
    ///
    /// **示例值**: "10"
    #[serde(
        rename = "capacity",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub capacity: i64,
    /// 会议室的相关描述
    ///
    /// **示例值**: "测试会议室描述"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 会议室的展示ID
    ///
    /// **示例值**: "LM134742334"
    #[serde(
        rename = "display_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display_id: String,
    /// 自定义的会议室ID
    ///
    /// **示例值**: "1234"
    #[serde(
        rename = "custom_room_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_room_id: String,
    /// 层级ID
    ///
    /// **示例值**: "omb_4ad1a2c7a2fbc5fc9570f38456931293"
    #[serde(
        rename = "room_level_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub room_level_id: String,
    /// 层级路径
    ///
    /// **示例值**: "[omb_8d020b12fe49e82847c2af3c193d5754,omb_8d020b12fe49e82847c2af3c193d5754]"
    #[serde(
        rename = "path",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub path: Vec<String>,
    /// 会议室状态
    #[serde(
        rename = "room_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub room_status: RoomStatusSubResp,
    /// 设施信息列表
    #[serde(
        rename = "device",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub device: Vec<DeviceSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RoomStatusSubResp {
    /// 是否启用会议室
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: bool,
    /// 会议室未来状态为启用或禁用
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "schedule_status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub schedule_status: bool,
    /// 禁用开始时间（unix时间，单位sec）
    ///
    /// **示例值**: "1652356050"
    #[serde(
        rename = "disable_start_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub disable_start_time: String,
    /// 禁用结束时间（unix时间，单位sec，数值0表示永久禁用）
    ///
    /// **示例值**: "1652442450"
    #[serde(
        rename = "disable_end_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub disable_end_time: String,
    /// 禁用原因
    ///
    /// **示例值**: "测试占用"
    #[serde(
        rename = "disable_reason",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub disable_reason: String,
    /// 联系人列表，id类型由user_id_type参数决定
    #[serde(
        rename = "contact_ids",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub contact_ids: Vec<String>,
    /// 是否在禁用时发送通知给预定了该会议室的员工
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "disable_notice",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub disable_notice: bool,
    /// 是否在恢复启用时发送通知给联系人
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "resume_notice",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub resume_notice: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct DeviceSubResp {
    /// 设施名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "电话"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
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
        Fn(BatchGetVcRoomReq) -> Result<(BatchGetVcRoomResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(BatchGetVcRoomReq) -> Result<(BatchGetVcRoomResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_get_vc_room<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, BatchGetVcRoomReq, BatchGetVcRoomResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_get_vc_room(
            &self,
            req: &BatchGetVcRoomReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, BatchGetVcRoomReq, BatchGetVcRoomResp, Arc<dyn MockFunc>>(
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
        api::gen::vc::batch_get_vc_room::{
            BatchGetVcRoomReq, BatchGetVcRoomResp, BatchGetVcRoomRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_batch_get_vc_room(|_| {
                Ok((BatchGetVcRoomResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .vc()
            .batch_get_vc_room(BatchGetVcRoomReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .vc()
            .batch_get_vc_room(BatchGetVcRoomReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "room_ids": [
        "omm_32864aa53cad6fc3c866aeb0d15cd0cc"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchGetVcRoomReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            {
                "room_id": "omm_4de32cf10a4358788ff4e09e37ebbf9b",
                "name": "测试会议室",
                "capacity": 10,
                "description": "测试会议室描述",
                "display_id": "LM134742334",
                "custom_room_id": "1234",
                "room_level_id": "omb_4ad1a2c7a2fbc5fc9570f38456931293",
                "path": [
                    "omb_8d020b12fe49e82847c2af3c193d5754"
                ],
                "room_status": {
                    "status": true,
                    "schedule_status": true,
                    "disable_start_time": "1652356050",
                    "disable_end_time": "1652442450",
                    "disable_reason": "测试占用",
                    "contact_ids": [
                        "ou_3ec3f6a28a0d08c45d895276e8e5e19b"
                    ],
                    "disable_notice": true,
                    "resume_notice": true
                },
                "device": [
                    {
                        "name": "电话"
                    }
                ]
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<BatchGetVcRoomRespInner>(RESP);
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
