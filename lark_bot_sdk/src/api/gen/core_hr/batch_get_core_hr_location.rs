//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/location/batch_get>
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
    /// **api 版本: 2024-02-06T02:23:31+00:00**
    ///
    /// ## 通过地点 ID 批量获取地点信息
    ///
    /// 通过地点 ID 批量获取地点信息
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/location/batch_get>
    ///
    /// new doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/location/batch_get>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2FuAjLw4CM%2FukTMukTMukTM%2Fcorehr-v2%2Flocation%2Fbatch_get>
    pub async fn batch_get_core_hr_location(
        &self,
        req: BatchGetCoreHrLocationReq,
    ) -> Result<(BatchGetCoreHrLocationResp, CommonResponse), Error> {
        self.batch_get_core_hr_location_with_opt(req, Default::default())
            .await
    }

    /// 参见 [batch_get_core_hr_location](#method.batch_get_core_hr_location) 函数
    pub async fn batch_get_core_hr_location_with_opt(
        &self,
        req: BatchGetCoreHrLocationReq,
        method_option: MethodOption,
    ) -> Result<(BatchGetCoreHrLocationResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_batch_get_core_hr_location(&req) {
                tracing::info!("[lark] CoreHr#BatchGetCoreHrLocation **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#BatchGetCoreHrLocation call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "BatchGetCoreHrLocation",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/corehr/v2/locations/batch_get",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (BatchGetCoreHrLocationRespInner, _) =
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
pub struct BatchGetCoreHrLocationReq {
    /// 地点 ID 列表
    ///
    /// **是否必填**: 是
    ///
    /// **数据校验规则**：
    ///
    /// - **长度范围**: `1` 字符- `100` 字符
    #[api(kind = "body", name = "location_ids")]
    pub location_ids: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct BatchGetCoreHrLocationRespInner {
    #[serde(flatten)]
    data: Option<BatchGetCoreHrLocationResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetCoreHrLocationResp {
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
    /// 查询的地点信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<LocationSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct LocationSubResp {
    /// 地点 ID
    ///
    /// **示例值**: "4718803945687580505"
    #[serde(
        rename = "location_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub location_id: String,
    /// 地点基本信息
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "hiberarchy_common",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hiberarchy_common: HiberarchyCommonSubResp,
    /// 地点用途
    #[serde(
        rename = "location_usage_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub location_usage_list: Vec<EnumSubResp>,
    /// 地址
    #[serde(
        rename = "address",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub address: Vec<AddressSubResp>,
    /// 工时制度
    ///
    /// **示例值**: "4690238309151997779"
    #[serde(
        rename = "working_hours_type_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub working_hours_type_id: String,
    /// 生效时间
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "2020-05-01 00:00:00"
    #[serde(
        rename = "effective_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub effective_time: String,
    /// 失效时间
    ///
    /// **示例值**: "2020-05-02 00:00:00"
    #[serde(
        rename = "expiration_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expiration_time: String,
    /// 自定义字段
    #[serde(
        rename = "custom_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_fields: Vec<CustomFieldDataSubResp>,
    /// 区域设置
    ///
    /// **示例值**: "zh_cn"
    #[serde(
        rename = "locale",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub locale: EnumSubResp,
    /// 时区
    ///
    /// **示例值**: "123456789"
    #[serde(
        rename = "time_zone_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub time_zone_id: String,
    /// 默认显示语言
    ///
    /// **示例值**: "123456789"
    #[serde(
        rename = "display_language_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display_language_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct HiberarchyCommonSubResp {
    /// 上级组织
    ///
    /// **示例值**: "4719168654814483759"
    #[serde(
        rename = "parent_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub parent_id: String,
    /// 名称
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: Vec<I18nSubResp>,
    /// 组织类型
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: EnumSubResp,
    /// 启用
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "active",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub active: bool,
    /// 生效时间
    ///
    /// **示例值**: "2020-05-01 00:00:00"
    #[serde(
        rename = "effective_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub effective_time: String,
    /// 失效时间
    ///
    /// **示例值**: "2020-05-02 00:00:00"
    #[serde(
        rename = "expiration_time",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub expiration_time: String,
    /// 编码
    ///
    /// **示例值**: "12456"
    #[serde(
        rename = "code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub code: String,
    /// 描述
    #[serde(
        rename = "description",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub description: Vec<I18nSubResp>,
    /// 树形排序，代表同层级的部门排序序号
    ///
    /// **示例值**: "123"
    #[serde(
        rename = "tree_order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tree_order: String,
    /// 列表排序，代表所有部门的混排序号
    ///
    /// **示例值**: "123"
    #[serde(
        rename = "list_order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub list_order: String,
    /// 自定义字段
    #[serde(
        rename = "custom_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_fields: Vec<ObjectFieldDataSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct EnumSubResp {
    /// 枚举值
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "phone_type"
    #[serde(
        rename = "enum_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub enum_name: String,
    /// 枚举多语展示
    #[serde(
        rename = "display",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub display: Vec<I18nSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AddressSubResp {
    /// 完整地址（本地文字）
    ///
    /// **示例值**: "中国北京北京"
    #[serde(
        rename = "full_address_local_script",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub full_address_local_script: String,
    /// 完整地址（西方文字）
    ///
    /// **示例值**: "Beijing, Beijing, China,"
    #[serde(
        rename = "full_address_western_script",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub full_address_western_script: String,
    /// 地址 ID
    ///
    /// **示例值**: "6989822217869624863"
    #[serde(
        rename = "address_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub address_id: String,
    /// 国家 / 地区
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "6862995757234914824"
    #[serde(
        rename = "country_region_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub country_region_id: String,
    /// 主要行政区
    ///
    /// **示例值**: "6863326815667095047"
    #[serde(
        rename = "region_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub region_id: String,
    /// 地址行 1（非拉丁语系的本地文字）
    ///
    /// **示例值**: "丹佛测试地址-纽埃时区"
    #[serde(
        rename = "local_address_line1",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub local_address_line1: String,
    /// 地址行 2（非拉丁语系的本地文字）
    ///
    /// **示例值**: "PoewH"
    #[serde(
        rename = "local_address_line2",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub local_address_line2: String,
    /// 地址行 3（非拉丁语系的本地文字）
    ///
    /// **示例值**: "PoewH"
    #[serde(
        rename = "local_address_line3",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub local_address_line3: String,
    /// 地址行 4（非拉丁语系的本地文字）
    ///
    /// **示例值**: "jmwJc"
    #[serde(
        rename = "local_address_line4",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub local_address_line4: String,
    /// 地址行 5（非拉丁语系的本地文字）
    ///
    /// **示例值**: "jmwJc"
    #[serde(
        rename = "local_address_line5",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub local_address_line5: String,
    /// 地址行 6（非拉丁语系的本地文字）
    ///
    /// **示例值**: "jmwJc"
    #[serde(
        rename = "local_address_line6",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub local_address_line6: String,
    /// 地址行 7（非拉丁语系的本地文字）
    ///
    /// **示例值**: "jmwJc"
    #[serde(
        rename = "local_address_line7",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub local_address_line7: String,
    /// 地址行 8（非拉丁语系的本地文字）
    ///
    /// **示例值**: "rafSu"
    #[serde(
        rename = "local_address_line8",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub local_address_line8: String,
    /// 地址行 9（非拉丁语系的本地文字）
    ///
    /// **示例值**: "McPRG"
    #[serde(
        rename = "local_address_line9",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub local_address_line9: String,
    /// 邮政编码
    ///
    /// **示例值**: "611530"
    #[serde(
        rename = "postal_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub postal_code: String,
    /// 地址类型
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "address_type_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub address_type_list: Vec<EnumSubResp>,
    /// 主要地址
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_primary",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_primary: bool,
    /// 公开地址
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "is_public",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub is_public: bool,
    /// 自定义字段
    #[serde(
        rename = "custom_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_fields: Vec<CustomFieldDataSubResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CustomFieldDataSubResp {
    /// 自定义字段 apiname，即自定义字段的唯一标识
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "name"
    #[serde(
        rename = "custom_api_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_api_name: String,
    /// 自定义字段名称
    #[serde(
        rename = "name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub name: CustomNameSubResp,
    /// 自定义字段类型
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: i64,
    /// 字段值，是 json 转义后的字符串，根据元数据定义不同，字段格式不同（如 123, 123.23, "true", ["id1","id2"], "2006-01-02 15:04:05"）
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "\"231\""
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 语言
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh-CN"
    #[serde(
        rename = "lang",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub lang: String,
    /// 内容
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "张三"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ObjectFieldDataSubResp {
    /// 字段名
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "name"
    #[serde(
        rename = "field_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub field_name: String,
    /// 字段值，是json转义后的字符串，根据元数据定义不同，字段格式不同(123, 123.23, true, [\"id1\",\"id2\], 2006-01-02 15:04:05])
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "Sandy"
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CustomNameSubResp {
    /// 中文
    ///
    /// **示例值**: "自定义姓名"
    #[serde(
        rename = "zh_cn",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub zh_cn: String,
    /// 英文
    ///
    /// **示例值**: "Custom Name"
    #[serde(
        rename = "en_us",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub en_us: String,
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
        Fn(BatchGetCoreHrLocationReq) -> Result<(BatchGetCoreHrLocationResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    BatchGetCoreHrLocationReq,
                ) -> Result<(BatchGetCoreHrLocationResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_batch_get_core_hr_location<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            BatchGetCoreHrLocationReq,
            BatchGetCoreHrLocationResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_batch_get_core_hr_location(
            &self,
            req: &BatchGetCoreHrLocationReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<
                Mocker,
                BatchGetCoreHrLocationReq,
                BatchGetCoreHrLocationResp,
                Arc<dyn MockFunc>,
            >(self.cli.instance_id, req)
        }
    }
}

#[cfg(feature = "test-util")]
#[cfg(test)]
mod test {
    use crate::{
        api::gen::core_hr::batch_get_core_hr_location::{
            BatchGetCoreHrLocationReq, BatchGetCoreHrLocationResp, BatchGetCoreHrLocationRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_batch_get_core_hr_location(|_| {
                Ok((
                    BatchGetCoreHrLocationResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .batch_get_core_hr_location(BatchGetCoreHrLocationReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .batch_get_core_hr_location(BatchGetCoreHrLocationReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "location_ids": [
        "1215"
    ]
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::BatchGetCoreHrLocationReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "items": [
            {
                "location_id": "4718803945687580505",
                "hiberarchy_common": {
                    "parent_id": "4719168654814483759",
                    "name": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ],
                    "type": {
                        "enum_name": "phone_type",
                        "display": [
                            {
                                "lang": "zh-CN",
                                "value": "张三"
                            }
                        ]
                    },
                    "active": true,
                    "effective_time": "2020-05-01 00:00:00",
                    "expiration_time": "2020-05-02 00:00:00",
                    "code": "12456",
                    "description": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ],
                    "tree_order": "123",
                    "list_order": "123",
                    "custom_fields": [
                        {
                            "field_name": "name",
                            "value": "Sandy"
                        }
                    ]
                },
                "location_usage_list": [
                    {
                        "enum_name": "phone_type",
                        "display": [
                            {
                                "lang": "zh-CN",
                                "value": "张三"
                            }
                        ]
                    }
                ],
                "address": [
                    {
                        "full_address_local_script": "中国北京北京",
                        "full_address_western_script": "Beijing, Beijing, China,",
                        "address_id": "6989822217869624863",
                        "country_region_id": "6862995757234914824",
                        "region_id": "6863326815667095047",
                        "local_address_line1": "丹佛测试地址-纽埃时区",
                        "local_address_line2": "PoewH",
                        "local_address_line3": "PoewH",
                        "local_address_line4": "jmwJc",
                        "local_address_line5": "jmwJc",
                        "local_address_line6": "jmwJc",
                        "local_address_line7": "jmwJc",
                        "local_address_line8": "rafSu",
                        "local_address_line9": "McPRG",
                        "postal_code": "611530",
                        "address_type_list": [
                            {
                                "enum_name": "phone_type",
                                "display": [
                                    {
                                        "lang": "zh-CN",
                                        "value": "张三"
                                    }
                                ]
                            }
                        ],
                        "is_primary": true,
                        "is_public": true,
                        "custom_fields": [
                            {
                                "custom_api_name": "name",
                                "name": {
                                    "zh_cn": "自定义姓名",
                                    "en_us": "Custom Name"
                                },
                                "type": 1,
                                "value": "\"231\""
                            }
                        ]
                    }
                ],
                "working_hours_type_id": "4690238309151997779",
                "effective_time": "2020-05-01 00:00:00",
                "expiration_time": "2020-05-02 00:00:00",
                "custom_fields": [
                    {
                        "custom_api_name": "name",
                        "name": {
                            "zh_cn": "自定义姓名",
                            "en_us": "Custom Name"
                        },
                        "type": 1,
                        "value": "\"231\""
                    }
                ],
                "locale": {
                    "enum_name": "phone_type",
                    "display": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ]
                },
                "time_zone_id": "123456789",
                "display_language_id": "123456789"
            }
        ]
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<BatchGetCoreHrLocationRespInner>(RESP);
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