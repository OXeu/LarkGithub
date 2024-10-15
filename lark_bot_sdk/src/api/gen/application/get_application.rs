//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/get>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::application::ApplicationService;

impl<'c, IStore: Store, IClient: HttpClient> ApplicationService<'c, IStore, IClient> {
    /// **api 版本: 2023-09-12T09:30:18+00:00**
    ///
    /// ## 获取应用信息
    ///
    /// 根据app_id获取应用的基础信息
    ///
    /// 商店应用必须正式发布版本后，才可以调用该接口获取应用信息。如果灰度发布应用，调用该接口将会报错 210504 错误码。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/get>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/application-v6/application/get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fapplication-v6%2Fapplication%2Fget>
    pub async fn get_application(
        &self,
        req: GetApplicationReq,
    ) -> Result<(GetApplicationResp, CommonResponse), Error> {
        self.get_application_with_opt(req, Default::default()).await
    }

    /// 参见 [get_application](#method.get_application) 函数
    pub async fn get_application_with_opt(
        &self,
        req: GetApplicationReq,
        method_option: MethodOption,
    ) -> Result<(GetApplicationResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_application(&req) {
                tracing::info!("[lark] Application#GetApplication **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Application#GetApplication call api");

        let req = ApiRequest {
            scope: "Application",
            api: "GetApplication",
            method: http::Method::GET,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/application/v6/applications/:app_id",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetApplicationRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetApplicationReq {
    /// 应用的 app_id，需要查询其他应用信息时，必须申请[获取应用信息](https://open.feishu.cn/document/ukTMukTMukTM/uQjN3QjL0YzN04CN2cDN)权限，仅查询本应用信息时，可填入 "me" 或者应用自身 app_id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "cli_9b445f5258795107"
    #[api(kind = "path", name = "app_id")]
    pub app_id: String,
    /// 指定获取应用在该语言下的信息
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh_cn"
    ///
    /// **可选值**:
    ///
    /// `zh_cn`: 中文
    ///
    /// `en_us`: 英文
    ///
    /// `ja_jp`: 日文
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[api(kind = "query", name = "lang", v_type = "var", option = "false")]
    pub lang: String,
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
struct GetApplicationRespInner {
    #[serde(flatten)]
    data: Option<GetApplicationResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetApplicationResp {
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
    /// 应用数据
    #[serde(
        rename = "app",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub app: ApplicationSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApplicationSubResp {
    /// 应用的 app_id
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "cli_9b445f5258795107"
    #[serde(
        rename = "app_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub app_id: String,
    /// 应用创建者（所有者）
    ///
    /// **示例值**: "ou_d317f090b7258ad0372aa53963cda70d"
    #[serde(
        rename = "creator_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub creator_id: String,
    /// 应用状态
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `disable`: 停用状态
    ///
    /// `enable`: 启用状态
    ///
    /// `not_enabled`: 未启用状态
    ///
    /// `unknown`: 未知状态
    #[serde(
        rename = "status",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub status: i64,
    /// 应用类型
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `self_build`: 自建应用
    ///
    /// `isv`: 应用商店应用
    ///
    /// `isp`: 个人应用商店应用
    ///
    /// `unknown`: 未知应用类型
    #[serde(
        rename = "scene_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub scene_type: i64,
    /// 付费类型
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `free`: 免费
    ///
    /// `paid`: 付费
    #[serde(
        rename = "payment_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub payment_type: i64,
    /// 安全设置中的重定向 URL
    #[serde(
        rename = "redirect_urls",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub redirect_urls: Vec<String>,
    /// 发布在线上的应用版本 ID，若没有则为空
    ///
    /// **示例值**: "oav_d317f090b7258ad0372aa53963cda70d"
    #[serde(
        rename = "online_version_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub online_version_id: String,
    /// 在审核中的版本 ID，若没有则为空
    ///
    /// **示例值**: "oav_d317f090b7258ad0372aa53963cda70d"
    #[serde(
        rename = "unaudit_version_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub unaudit_version_id: String,
    /// 应用名称
    ///
    /// **示例值**: "应用名称"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "app_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub app_name: String,
    /// 应用图标 url
    ///
    /// **示例值**: "https://sf1-ttcdn-tos.pstatp.com/img/avatar/d279000ca4d3f7f6aaff~72x72.jpg"
    #[serde(
        rename = "avatar_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub avatar_url: String,
    /// 应用默认描述
    ///
    /// **示例值**: "应用描述"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 应用权限列表
    #[serde(
        rename = "scopes",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub scopes: Vec<AppScopeSubResp>,
    /// 后台主页地址
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "back_home_url",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub back_home_url: String,
    /// 应用的国际化信息列表
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "i18n",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n: Vec<AppI18nInfoSubResp>,
    /// 应用主语言
    ///
    /// **示例值**: "zh_cn"
    ///
    /// **可选值**:
    ///
    /// `zh_cn`: 中文
    ///
    /// `en_us`: 英文
    ///
    /// `ja_jp`: 日文
    #[serde(
        rename = "primary_language",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub primary_language: String,
    /// 应用分类的国际化描述
    ///
    /// **数据校验规则**：
    ///
    /// - **最大长度**: `3` 字符
    #[serde(
        rename = "common_categories",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub common_categories: Vec<String>,
    /// 应用的所有者信息
    #[serde(
        rename = "owner",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub owner: ApplicationOwnerSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ApplicationOwnerSubResp {
    /// 应用所有者类型
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "0"
    ///
    /// **可选值**:
    ///
    /// `LarkTechnology`: 飞书科技
    ///
    /// `LarkPartners`: 飞书合作伙伴
    ///
    /// `EnterpriseMember`: 企业内成员
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: i64,
    /// 应用所有者ID
    ///
    /// **示例值**: "ou_d317f090b7258ad0372aa53963cda70d"
    #[serde(
        rename = "owner_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub owner_id: String,
    /// 应用开发商名称(仅商店应用返回)
    ///
    /// **示例值**: "test tenant"
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 应用开发商服务台链接(仅商店应用返回)
    ///
    /// **示例值**: "https://applink.feishu.cn/client/helpdesk/open?id=6940534140529803284"
    #[serde(
        rename = "help_desk",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub help_desk: String,
    /// 应用开发商的邮箱(仅商店应用返回)
    ///
    /// **示例值**: "test123@163.com"
    #[serde(
        rename = "email",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub email: String,
    /// 应用开发商的手机号(仅商店应用返回)
    ///
    /// **示例值**: "1234534234234"
    #[serde(
        rename = "phone",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppScopeSubResp {
    /// 应用权限
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "contact:user.base"
    #[serde(
        rename = "scope",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub scope: String,
    /// 应用权限的国际化描述
    ///
    /// **示例值**: "获取应用信息"
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 权限等级描述
    ///
    /// **示例值**: "1"
    ///
    /// **可选值**:
    ///
    /// `low_level`: 普通权限
    ///
    /// `high_level`: 高级权限
    ///
    /// `super_level`: 超敏感权限
    ///
    /// `unknown_level`: 未知等级
    #[serde(
        rename = "level",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub level: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppI18nInfoSubResp {
    /// 国际化语言的 key
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh_cn"
    ///
    /// **可选值**:
    ///
    /// `zh_cn`: 中文
    ///
    /// `en_us`: 英文
    ///
    /// `ja_jp`: 日文
    #[serde(
        rename = "i18n_key",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub i18n_key: String,
    /// 应用国际化名称
    ///
    /// **示例值**: "应用名称"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: String,
    /// 应用国际化描述（副标题）
    ///
    /// **示例值**: "应用描述"
    ///
    /// **数据校验规则**：
    ///
    /// - **最小长度**: `1` 字符
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: String,
    /// 国际化帮助文档链接
    ///
    /// **示例值**: "https://www.example.com"
    #[serde(
        rename = "help_use",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub help_use: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::application::ApplicationServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(GetApplicationReq) -> Result<(GetApplicationResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(GetApplicationReq) -> Result<(GetApplicationResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> ApplicationServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_application<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, GetApplicationReq, GetApplicationResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_application(
            &self,
            req: &GetApplicationReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetApplicationReq, GetApplicationResp, Arc<dyn MockFunc>>(
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
        api::gen::application::get_application::{
            GetApplicationReq, GetApplicationResp, GetApplicationRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .application()
            .mock()
            .mock_get_application(|_| {
                Ok((GetApplicationResp::default(), CommonResponse::default()))
            })
            .build();
        let res = lark
            .application()
            .get_application(GetApplicationReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .application()
            .get_application(GetApplicationReq::default())
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
        "app": {
            "app_id": "cli_9b445f5258795107",
            "creator_id": "ou_d317f090b7258ad0372aa53963cda70d",
            "status": 1,
            "scene_type": 0,
            "payment_type": 0,
            "redirect_urls": [
                "https://www.example.com"
            ],
            "online_version_id": "oav_d317f090b7258ad0372aa53963cda70d",
            "unaudit_version_id": "oav_d317f090b7258ad0372aa53963cda70d",
            "app_name": "应用名称",
            "avatar_url": "https://sf1-ttcdn-tos.pstatp.com/img/avatar/d279000ca4d3f7f6aaff~72x72.jpg",
            "description": "应用描述",
            "scopes": [
                {
                    "scope": "contact:user.base",
                    "description": "获取应用信息",
                    "level": 1
                }
            ],
            "back_home_url": "https://www.example.com",
            "i18n": [
                {
                    "i18n_key": "zh_cn",
                    "name": "应用名称",
                    "description": "应用描述",
                    "help_use": "https://www.example.com"
                }
            ],
            "primary_language": "zh_cn",
            "common_categories": [
                "分析工具"
            ],
            "owner": {
                "type": 0,
                "owner_id": "ou_d317f090b7258ad0372aa53963cda70d",
                "name": "test tenant",
                "help_desk": "https://applink.feishu.cn/client/helpdesk/open?id=6940534140529803284",
                "email": "test123@163.com",
                "phone": "1234534234234"
            }
        }
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetApplicationRespInner>(RESP);
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