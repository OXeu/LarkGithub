//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/mget>
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
    /// **api 版本: 2024-07-23T07:32:58+00:00**
    ///
    /// ## 批量查询会议室层级详情
    ///
    /// 该接口可以使用会议室层级 ID 批量查询会议室层级详情。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/mget>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/vc-v1/room_level/mget>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fvc-v1%2Froom_level%2Fmget>
    pub async fn batch_get_vc_room_level(
        &self,
        req: BatchGetVcRoomLevelReq,
    ) -> Result<(BatchGetVcRoomLevelResp, CommonResponse), Error> {
        self.batch_get_vc_room_level_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_get_vc_room_level](#method.batch_get_vc_room_level) 函数
    pub async fn batch_get_vc_room_level_with_opt(
        &self,
        req: BatchGetVcRoomLevelReq,
        method_option: MethodOption,
    ) -> Result<(BatchGetVcRoomLevelResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_get_vc_room_level(&req) {
                tracing::info!("[lark] Vc#BatchGetVcRoomLevel **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Vc#BatchGetVcRoomLevel call api");

        let req = ApiRequest {
            scope: "Vc",
            api: "BatchGetVcRoomLevel",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/vc/v1/room_levels/mget",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchGetVcRoomLevelRespInner, _) = self.cli.do_req(req).await?;
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
pub struct BatchGetVcRoomLevelReq {
    /// 层级ID列表
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "["omb_4ad1a2c7a2fbc5fc9570f38456931293"]"
    #[api(kind = "body", name = "level_ids")]
    pub level_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchGetVcRoomLevelRespInner {
    #[serde(flatten)]
    data: Option<BatchGetVcRoomLevelResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetVcRoomLevelResp {
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
    /// 会议室层级列表
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<RoomLevelSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct RoomLevelSubResp {
    /// 层级ID
    ///
    /// **示例值**: "层级ID"
    #[serde(
        rename = "room_level_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub room_level_id: String,
    /// 层级名称
    ///
    /// **示例值**: "测试层级"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 父层级ID
    ///
    /// **示例值**: "omb_4ad1a2c7a2fbc5fc9570f38456931293"
    #[serde(
        rename = "parent_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_id: String,
    /// 层级路径
    ///
    /// **示例值**: "[omb_8d020b12fe49e82847c2af3c193d5754, omb_8d020b12fe49e82847c2af3c193d5754]"
    #[serde(
        rename = "path",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub path: Vec<String>,
    /// 是否有子层级
    ///
    /// **示例值**: "false"
    #[serde(
        rename = "has_child",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_child: bool,
    /// 自定义层级ID
    ///
    /// **示例值**: "10000"
    #[serde(
        rename = "custom_group_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_group_id: String,
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
        Fn(BatchGetVcRoomLevelReq) -> Result<(BatchGetVcRoomLevelResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchGetVcRoomLevelReq,
                ) -> Result<(BatchGetVcRoomLevelResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> VcServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_get_vc_room_level<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, BatchGetVcRoomLevelReq, BatchGetVcRoomLevelResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_get_vc_room_level(
            &self,
            req: &BatchGetVcRoomLevelReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, BatchGetVcRoomLevelReq, BatchGetVcRoomLevelResp, Arc<dyn MockFunc>>(
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
        api::gen::vc::batch_get_vc_room_level::{
            BatchGetVcRoomLevelReq, BatchGetVcRoomLevelResp, BatchGetVcRoomLevelRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .vc()
            .mock()
            .mock_batch_get_vc_room_level(|_| {
                Ok((
                    BatchGetVcRoomLevelResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .vc()
            .batch_get_vc_room_level(BatchGetVcRoomLevelReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .vc()
            .batch_get_vc_room_level(BatchGetVcRoomLevelReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "level_ids": [
        "omb_4ad1a2c7a2fbc5fc9570f38456931293"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchGetVcRoomLevelReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            {
                "room_level_id": "层级ID",
                "name": "测试层级",
                "parent_id": "omb_4ad1a2c7a2fbc5fc9570f38456931293",
                "path": [
                    "omb_4ad1a2c7a2fbc5fc9570f38456931293"
                ],
                "has_child": false,
                "custom_group_id": "10000"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<BatchGetVcRoomLevelRespInner>(RESP);
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