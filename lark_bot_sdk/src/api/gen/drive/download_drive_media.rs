//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/download>
// Code generated by gen_api. DO NOT EDIT.

use crate::api::DownloadResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};
use serde::{Deserialize, Serialize};

use crate::api::gen::drive::DriveService;

impl<'c, IStore: Store, IClient: HttpClient> DriveService<'c, IStore, IClient> {
    /// **api 版本: 2024-06-06T12:20:56+00:00**
    ///
    /// ## 下载素材
    ///
    /// 下载各类云文档中的素材，例如电子表格中的图片。该接口支持通过在请求头添加`Range` 参数分片下载素材。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/download>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/drive-v1/media/download>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fdrive-v1%2Fmedia%2Fdownload>
    pub async fn download_drive_media(
        &self,
        req: DownloadDriveMediaReq,
    ) -> Result<(DownloadResp<IClient>, CommonResponse), Error> {
        self.download_drive_media_with_opt(req, Default::default())
            .await
    }

    /// 参见 [download_drive_media](#method.download_drive_media) 函数
    pub async fn download_drive_media_with_opt(
        &self,
        req: DownloadDriveMediaReq,
        method_option: MethodOption,
    ) -> Result<(DownloadResp<IClient>, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_download_drive_media(&req) {
                tracing::info!("[lark] Drive#DownloadDriveMedia **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#DownloadDriveMedia call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "DownloadDriveMedia",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/medias/:file_token/download",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (bin_data, bin_name, common_resp) = self.cli.do_download_req(req).await?;
        let resp = DownloadResp {
            data: bin_data,
            name: bin_name,
        };

        Ok((resp, common_resp))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, lark_bot_sdk_macros::ApiReqParams)]
pub struct DownloadDriveMediaReq {
    /// 素材文件的 token。获取方式如下所示：
    ///
    /// * 新版文档：通过[获取文档所有块](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/list)接口获取指定文件块（File Block）或图片块（Image Block）的 token，即为素材的 token。
    ///
    /// * 电子表格：通过[读取多个范围](https://open.feishu.cn/document/ukTMukTMukTM/ukTMzUjL5EzM14SOxMTN)接口获取指定附件的
    ///
    /// `fileToken` 参数，即为素材的 token。
    ///
    /// * 多维表格：通过[列出记录](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/list)接口获取指定附件的 `file_token`，即为素材的 token。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "boxcnrHpsg1QDqXAAAyachabcef"
    #[api(kind = "path", name = "file_token")]
    pub file_token: String,
    /// 拥有高级权限的多维表格在下载素材时，需要添加额外的扩展信息作为 URL 查询参数鉴权。详情参考[素材概述-extra 参数说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/introduction)。未填正确填写该参数的接口将返回 403 的 HTTP 状态码。
    ///
    /// **示例值**: "无"
    #[api(kind = "query", name = "extra", v_type = "var", option = "false")]
    pub extra: String,
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

    pub trait MockFunc<IClient: HttpClient>:
        Fn(DownloadDriveMediaReq) -> Result<(DownloadResp<IClient>, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            IClient: HttpClient,
            T: Fn(DownloadDriveMediaReq) -> Result<(DownloadResp<IClient>, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc<IClient> for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_download_drive_media<F: MockFunc<IClient>>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            DownloadDriveMediaReq,
            DownloadResp<IClient>,
            Arc<dyn MockFunc<IClient>>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_download_drive_media(
            &self,
            req: &DownloadDriveMediaReq,
        ) -> Option<Arc<dyn MockFunc<IClient>>> {
            do_mock::<
                Mocker,
                DownloadDriveMediaReq,
                DownloadResp<IClient>,
                Arc<dyn MockFunc<IClient>>,
            >(self.cli.instance_id, req)
        }
    }
}