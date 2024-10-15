//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-version/get>
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
    /// **api 版本: 2024-07-15T07:04:52+00:00**
    ///
    /// ## 获取文档版本信息
    ///
    /// 该接口用于获取文档或电子表格指定版本的信息，包括标题、标识、创建者、创建时间等。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-version/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/docs/drive-v1/file-version/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fdocs%2Fdrive-v1%2Ffile-version%2Fget>
    pub async fn get_drive_file_version(
        &self,
        req: GetDriveFileVersionReq,
    ) -> Result<(GetDriveFileVersionResp, CommonResponse), Error> {
        self.get_drive_file_version_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_drive_file_version](#method.get_drive_file_version) 函数
    pub async fn get_drive_file_version_with_opt(
        &self,
        req: GetDriveFileVersionReq,
        method_option: MethodOption,
    ) -> Result<(GetDriveFileVersionResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_drive_file_version(&req) {
                tracing::info!("[lark] Drive#GetDriveFileVersion **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Drive#GetDriveFileVersion call api");

        let req = ApiRequest {
            scope: "Drive",
            api: "GetDriveFileVersion",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/drive/v1/files/:file_token/versions/:version_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            need_user_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetDriveFileVersionRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetDriveFileVersionReq {
    /// 源文档的 token。获取方式参考 [如何获取云文档相关 token](https://open.feishu.cn/document/ukTMukTMukTM/uczNzUjL3czM14yN3MTN#08bb5df6)。
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "shtbcqqoXZJaKYrfN5IHQgabcef"
    #[api(kind = "path", name = "file_token")]
    pub file_token: String,
    /// 版本文档的版本标识
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "fnJfyX"
    #[api(kind = "path", name = "version_id")]
    pub version_id: String,
    /// 源文档的类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "docx"
    ///
    /// **可选值**:
    ///
    /// `Docx`: 新版文档
    ///
    /// `Sheet`: 电子表格
    #[api(kind = "query", name = "obj_type", v_type = "var", option = "false")]
    pub obj_type: String,
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
struct GetDriveFileVersionRespInner {
    #[serde(flatten)]
    data: Option<GetDriveFileVersionResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDriveFileVersionResp {
    /// \-
    #[serde(
        rename = "data",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub data: VersionSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct VersionSubResp {
    /// 版本文档的标题
    ///
    /// **示例值**: "项目文档 第1版"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 版本文档的版本标识
    ///
    /// **示例值**: "fnJfyX"
    #[serde(
        rename = "version",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub version: String,
    /// 当前版本对应的源文档的 token
    ///
    /// **示例值**: "doxbcyvqZlSc9WlHvQMlSJabcef"
    #[serde(
        rename = "parent_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_token: String,
    /// 版本文档的所有者的 ID
    ///
    /// **示例值**: "694699009591869450"
    #[serde(
        rename = "owner_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub owner_id: String,
    /// 版本文档的创建者的 ID
    ///
    /// **示例值**: "694699009591869451"
    #[serde(
        rename = "creator_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub creator_id: String,
    /// 版本文档的创建时间，Unix 时间戳，单位为秒
    ///
    /// **示例值**: "1660708537"
    #[serde(
        rename = "create_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub create_time: String,
    /// 版本文档的更新时间
    ///
    /// **示例值**: "1660708537"
    #[serde(
        rename = "update_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub update_time: String,
    /// 版本文档的状态
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `StatusExist`: 正常状态
    ///
    /// `StatusDeleted`: 删除状态
    ///
    /// `StatusTrash`: 回收站状态
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: String,
    /// 版本文档的类型
    ///
    /// **示例值**: "docx"
    ///
    /// **可选值**:
    ///
    /// `Docx`: 新版文档
    ///
    /// `Sheet`: 电子表格
    #[serde(
        rename = "obj_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub obj_type: String,
    /// 源文档的类型
    ///
    /// **示例值**: "docx"
    ///
    /// **可选值**:
    ///
    /// `ObjTypeDocx`: 新版文档
    ///
    /// `ObjTypeSheet`: 电子表格
    #[serde(
        rename = "parent_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_type: String,
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
        Fn(GetDriveFileVersionReq) -> Result<(GetDriveFileVersionResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetDriveFileVersionReq,
                ) -> Result<(GetDriveFileVersionResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> DriveServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_drive_file_version<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetDriveFileVersionReq, GetDriveFileVersionResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_drive_file_version(
            &self,
            req: &GetDriveFileVersionReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetDriveFileVersionReq, GetDriveFileVersionResp, Arc<dyn MockFunc>>(
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
        api::gen::drive::get_drive_file_version::{
            GetDriveFileVersionReq, GetDriveFileVersionResp, GetDriveFileVersionRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .drive()
            .mock()
            .mock_get_drive_file_version(|_| {
                Ok((
                    GetDriveFileVersionResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .drive()
            .get_drive_file_version(GetDriveFileVersionReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .drive()
            .get_drive_file_version(GetDriveFileVersionReq::default())
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
        "name": "项目文档 第1版",
        "version": "fnJfyX",
        "parent_token": "doxbcyvqZlSc9WlHvQMlSJabcef",
        "owner_id": "694699009591869450",
        "creator_id": "694699009591869451",
        "create_time": "1660708537",
        "update_time": "1660708537",
        "status": "0",
        "obj_type": "docx",
        "parent_type": "docx"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetDriveFileVersionRespInner>(RESP);
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
