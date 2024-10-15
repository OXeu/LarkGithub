//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_prepare>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::drive::DriveService;

impl<'c, IStore: Store, IClient: HttpClient> DriveService<'c, IStore, IClient> {
    /// **api 版本: 2024-06-06T13:01:50+00:00**
    ///
    /// ## 分片上传素材-预上传
    ///
    /// 发送初始化请求，以获取上传事务 ID 和分片策略，为[上传素材分片](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_part)做准备。平台固定以 4MB 的大小对素材进行分片。了解完整的分片上传素材流程，参考[素材概述](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/introduction)。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_prepare>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/drive-v1/media/multipart-upload-media/upload_prepare>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fdrive-v1%2Fmedia%2Fmultipart-upload-media%2Fupload_prepare>
    pub async fn prepare_upload_drive_media(
        &self,
        req: PrepareUploadDriveMediaReq,
    ) -> Result<(PrepareUploadDriveMediaResp, CommonResponse), Error> {
        self.prepare_upload_drive_media_with_opt(req, Default::default())
            .await
    }

    /// 参见 [prepare_upload_drive_media](#method.prepare_upload_drive_media) 函数
    pub async fn prepare_upload_drive_media_with_opt(
        &self,
        req: PrepareUploadDriveMediaReq,
        method_option: MethodOption,
    ) -> Result<(PrepareUploadDriveMediaResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_prepare_upload_drive_media(&req) {
                tracing::info!("[lark] Drive#PrepareUploadDriveMedia **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#PrepareUploadDriveMedia call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "PrepareUploadDriveMedia",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/medias/upload_prepare",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (PrepareUploadDriveMediaRespInner, _) =
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
pub struct PrepareUploadDriveMediaReq {
    /// 素材的文件名称
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "demo.jpeg"
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `250` 字符
    #[api(kind = "body", name = "file_name")]
    pub file_name: String,
    /// 上传点的类型。你可根据上传的文件类型与云文档类型确定上传点类型。例如，要将一张图片插入到新版文档（文件类型为 `docx`）中，需指定上传点为 `docx_image`；要将一个附件上传到新版文档中，需指定上传点为 `docx_file`。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "docx_image"
    ///
    /// **可选值**:
    ///
    /// `doc_image`: 旧版文档图片
    ///
    /// `docx_image`: 新版文档图片
    ///
    /// `sheet_image`: 电子表格图片
    ///
    /// `doc_file`: 文档文件
    ///
    /// `docx_file`: 新版文档文件
    ///
    /// `sheet_file`: 电子表格文件
    ///
    /// `vc_virtual_background`: vc 虚拟背景（灰度中，暂未开放）
    ///
    /// `bitable_image`: 多维表格图片
    ///
    /// `bitable_file`: 多维表格文件
    ///
    /// `moments`: 同事圈（灰度中，暂未开放）
    ///
    /// `ccm_import_open`: 云文档导入文件
    #[api(kind = "body", name = "parent_type")]
    pub parent_type: String,
    /// 上传点的 token，即要上传的云文档的 token，用于指定素材将要上传到的云文档或位置。参考 [素材概述](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/introduction) 了解上传点类型与上传点 token 的对应关系。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "doccnFivLCfJfblZjGZtxgabcef"
    #[api(kind = "body", name = "parent_node")]
    pub parent_node: String,
    /// 文件的大小，单位为字节
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "1024"
    #[api(kind = "body", name = "size")]
    pub size: i64,
    /// 其它扩展信息
    ///
    /// **示例值**: "{\"test\":\"test\"}"
    #[api(kind = "body", name = "extra")]
    pub extra: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct PrepareUploadDriveMediaRespInner {
    #[serde(flatten)]
    data: Option<PrepareUploadDriveMediaResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PrepareUploadDriveMediaResp {
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
    /// 分片上传事务 ID
    ///
    /// **示例值**: "7111211691345512356"
    #[serde(
        rename = "upload_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub upload_id: String,
    /// 分片大小策略
    ///
    /// **示例值**: "4194304"
    #[serde(
        rename = "block_size",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub block_size: i64,
    /// 分片数量
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "block_num",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub block_num: i64,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::drive::DriveServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(
            PrepareUploadDriveMediaReq,
        ) -> Result<(PrepareUploadDriveMediaResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    PrepareUploadDriveMediaReq,
                ) -> Result<(PrepareUploadDriveMediaResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_prepare_upload_drive_media<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            PrepareUploadDriveMediaReq,
            PrepareUploadDriveMediaResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_prepare_upload_drive_media(
            &self,
            req: &PrepareUploadDriveMediaReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                PrepareUploadDriveMediaReq,
                PrepareUploadDriveMediaResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::drive::prepare_upload_drive_media::{
            PrepareUploadDriveMediaReq, PrepareUploadDriveMediaResp,
            PrepareUploadDriveMediaRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_prepare_upload_drive_media(|_| {
                Ok((
                    PrepareUploadDriveMediaResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .prepare_upload_drive_media(PrepareUploadDriveMediaReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .prepare_upload_drive_media(PrepareUploadDriveMediaReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "file_name": "demo.jpeg",
    "parent_type": "docx_image",
    "parent_node": "doccnFivLCfJfblZjGZtxgabcef",
    "size": 1024,
    "extra": "{\"test\":\"test\"}"
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::PrepareUploadDriveMediaReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "upload_id": "7111211691345512356",
        "block_size": 4194304,
        "block_num": 1
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<PrepareUploadDriveMediaRespInner>(RESP);
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