//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/list>
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
    /// **api 版本: 2024-05-10T06:43:45+00:00**
    ///
    /// ## 批量查询公司
    ///
    /// 批量查询公司。
    ///

    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/list>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/corehr-v1/organization-management/company/list>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fcorehr-v1%2Forganization-management%2Fcompany%2Flist>
    pub async fn get_core_hr_company_list(
        &self,
        req: GetCoreHrCompanyListReq,
    ) -> Result<(GetCoreHrCompanyListResp, CommonResponse), Error> {
        self.get_core_hr_company_list_with_opt(req, Default::default())
            .await
    }

    /// 参见 [get_core_hr_company_list](#method.get_core_hr_company_list) 函数
    pub async fn get_core_hr_company_list_with_opt(
        &self,
        req: GetCoreHrCompanyListReq,
        method_option: MethodOption,
    ) -> Result<(GetCoreHrCompanyListResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_get_core_hr_company_list(&req) {
                tracing::info!("[lark] CoreHr#GetCoreHrCompanyList **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] CoreHr#GetCoreHrCompanyList call api");

        let req = ApiRequest {
            scope: "CoreHr",
            api: "GetCoreHrCompanyList",
            method: http::Method::GET,
            url: String::new() + self.cli.open_base_url.as_ref() + "/open-apis/corehr/v1/companies",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (GetCoreHrCompanyListRespInner, _) = self.cli.do_req(req).await?;
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
pub struct GetCoreHrCompanyListReq {
    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    ///
    /// **示例值**: "1231231987"
    #[api(kind = "query", name = "page_token", v_type = "var", option = "false")]
    pub page_token: String,
    /// 分页大小
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "100"
    #[api(kind = "query", name = "page_size", v_type = "var", option = "false")]
    pub page_size: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct GetCoreHrCompanyListRespInner {
    #[serde(flatten)]
    data: Option<GetCoreHrCompanyListResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetCoreHrCompanyListResp {
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
    /// 查询的公司信息
    #[serde(
        rename = "items",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub items: Vec<CompanySubResp>,
    /// 是否还有更多项
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "has_more",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    ///
    /// **示例值**: "1234452132"
    #[serde(
        rename = "page_token",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub page_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct CompanySubResp {
    /// 公司 ID
    ///
    /// **示例值**: "4692472714243080020"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 层级关系，内层字段见实体
    ///
    /// **是否必填**: 是
    #[serde(
        rename = "hiberarchy_common",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub hiberarchy_common: HiberarchyCommonSubResp,
    /// 性质，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)公司类型（company_type）枚举定义部分获得
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: EnumSubResp,
    /// 行业，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)行业（industry）枚举定义部分获得
    #[serde(
        rename = "industry_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub industry_list: Vec<EnumSubResp>,
    /// 法定代表人
    #[serde(
        rename = "legal_representative",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub legal_representative: Vec<I18nSubResp>,
    /// 邮编
    ///
    /// **示例值**: "邮编"
    #[serde(
        rename = "post_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub post_code: String,
    /// 纳税人识别号
    ///
    /// **示例值**: "123456840"
    #[serde(
        rename = "tax_payer_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tax_payer_id: String,
    /// 是否保密
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "confidential",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub confidential: bool,
    /// 主体类型，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)主体类型（company_sub_type）枚举定义部分获得
    #[serde(
        rename = "sub_type_list",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub sub_type_list: Vec<EnumSubResp>,
    /// 是否为分公司
    ///
    /// **示例值**: "true"
    #[serde(
        rename = "branch_company",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub branch_company: bool,
    /// 主要负责人
    #[serde(
        rename = "primary_manager",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub primary_manager: Vec<I18nSubResp>,
    /// 自定义字段
    #[serde(
        rename = "custom_fields",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub custom_fields: Vec<ObjectFieldDataSubResp>,
    /// 默认币种
    #[serde(
        rename = "currency",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency: CurrencySubResp,
    /// 电话
    #[serde(
        rename = "phone",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub phone: PhoneNumberAndAreaCodeSubResp,
    /// 传真
    #[serde(
        rename = "fax",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub fax: PhoneNumberAndAreaCodeSubResp,
    /// 完整注册地址
    #[serde(
        rename = "registered_office_address",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub registered_office_address: Vec<I18nSubResp>,
    /// 完整办公地址
    #[serde(
        rename = "office_address",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub office_address: Vec<I18nSubResp>,
    /// 注册地址详细信息
    #[serde(
        rename = "registered_office_address_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub registered_office_address_info: AddressSubResp,
    /// 办公地址详细信息
    #[serde(
        rename = "office_address_info",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub office_address_info: AddressSubResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct HiberarchyCommonSubResp {
    /// 上级 ID
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
    /// 组织类型，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)组织类型（organization_type）枚举定义部分获得
    #[serde(
        rename = "type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub body_type: EnumSubResp,
    /// 是否启用
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
    /// 树形排序
    ///
    /// **示例值**: "001000"
    #[serde(
        rename = "tree_order",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub tree_order: String,
    /// 列表排序
    ///
    /// **示例值**: "001000-001000"
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
    /// **示例值**: "type_1"
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
pub struct CurrencySubResp {
    /// 货币id
    ///
    /// **示例值**: "1"
    #[serde(
        rename = "id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub id: String,
    /// 货币所属国家/地区id，详细信息可通过[【查询国家/地区信息】](https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/location_data/list)接口查询获得
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "country_region_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub country_region_id: String,
    /// 货币名称
    #[serde(
        rename = "currency_name",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency_name: Vec<I18nSubResp>,
    /// 数字代码
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "numeric_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub numeric_code: i64,
    /// 三位字母代码
    ///
    /// **示例值**: "12"
    #[serde(
        rename = "currency_alpha_3_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub currency_alpha_3_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PhoneNumberAndAreaCodeSubResp {
    /// 区号
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "123123"
    #[serde(
        rename = "area_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub area_code: EnumSubResp,
    /// 号码
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "213213"
    #[serde(
        rename = "phone_number",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub phone_number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AddressSubResp {
    /// 邮政编码
    ///
    /// **示例值**: "611530"
    #[serde(
        rename = "postal_code",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub postal_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct I18nSubResp {
    /// 名称信息的语言
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "zh-CN"
    #[serde(
        rename = "lang",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub lang: String,
    /// 名称信息的内容
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
    /// 字段值，是json转义后的字符串，根据元数据定义不同，字段格式不同(如123, 123.23, "true", [\"id1\",\"id2\"], "2006-01-02 15:04:05")
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "\"Sandy\""
    #[serde(
        rename = "value",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub value: String,
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
        Fn(GetCoreHrCompanyListReq) -> Result<(GetCoreHrCompanyListResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    GetCoreHrCompanyListReq,
                ) -> Result<(GetCoreHrCompanyListResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> CoreHrServiceMocker<'c, IStore, IClient> {
        pub fn mock_get_core_hr_company_list<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<
            Mocker,
            GetCoreHrCompanyListReq,
            GetCoreHrCompanyListResp,
            Arc<dyn MockFunc>,
        > {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_get_core_hr_company_list(
            &self,
            req: &GetCoreHrCompanyListReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, GetCoreHrCompanyListReq, GetCoreHrCompanyListResp, Arc<dyn MockFunc>>(
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
        api::gen::core_hr::get_core_hr_company_list::{
            GetCoreHrCompanyListReq, GetCoreHrCompanyListResp, GetCoreHrCompanyListRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .core_hr()
            .mock()
            .mock_get_core_hr_company_list(|_| {
                Ok((
                    GetCoreHrCompanyListResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .core_hr()
            .get_core_hr_company_list(GetCoreHrCompanyListReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .core_hr()
            .get_core_hr_company_list(GetCoreHrCompanyListReq::default())
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
                "id": "4692472714243080020",
                "hiberarchy_common": {
                    "parent_id": "4719168654814483759",
                    "name": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ],
                    "type": {
                        "enum_name": "type_1",
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
                    "tree_order": "001000",
                    "list_order": "001000-001000",
                    "custom_fields": [
                        {
                            "field_name": "name",
                            "value": "\"Sandy\""
                        }
                    ]
                },
                "type": {
                    "enum_name": "type_1",
                    "display": [
                        {
                            "lang": "zh-CN",
                            "value": "张三"
                        }
                    ]
                },
                "industry_list": [
                    {
                        "enum_name": "type_1",
                        "display": [
                            {
                                "lang": "zh-CN",
                                "value": "张三"
                            }
                        ]
                    }
                ],
                "legal_representative": [
                    {
                        "lang": "zh-CN",
                        "value": "张三"
                    }
                ],
                "post_code": "邮编",
                "tax_payer_id": "123456840",
                "confidential": true,
                "sub_type_list": [
                    {
                        "enum_name": "type_1",
                        "display": [
                            {
                                "lang": "zh-CN",
                                "value": "张三"
                            }
                        ]
                    }
                ],
                "branch_company": true,
                "primary_manager": [
                    {
                        "lang": "zh-CN",
                        "value": "张三"
                    }
                ],
                "custom_fields": [
                    {
                        "field_name": "name",
                        "value": "\"Sandy\""
                    }
                ],
                "currency": {
                    "id": "1",
                    "country_region_id": "12",
                    "currency_name": [
                        {
                            "lang": "zh-CN",
                            "value": "刘梓新"
                        }
                    ],
                    "numeric_code": 12,
                    "currency_alpha_3_code": "12"
                },
                "phone": {
                    "area_code": {
                        "enum_name": "phone_type",
                        "display": [
                            {
                                "lang": "zh-CN",
                                "value": "刘梓新"
                            }
                        ]
                    },
                    "phone_number": "213213"
                },
                "fax": {
                    "area_code": {
                        "enum_name": "phone_type",
                        "display": [
                            {
                                "lang": "zh-CN",
                                "value": "刘梓新"
                            }
                        ]
                    },
                    "phone_number": "213213"
                },
                "registered_office_address": [
                    {
                        "lang": "zh-CN",
                        "value": "刘梓新"
                    }
                ],
                "office_address": [
                    {
                        "lang": "zh-CN",
                        "value": "刘梓新"
                    }
                ],
                "registered_office_address_info": {
                    "postal_code": "611530"
                },
                "office_address_info": {
                    "postal_code": "611530"
                }
            }
        ],
        "has_more": true,
        "page_token": "1234452132"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<GetCoreHrCompanyListRespInner>(RESP);
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