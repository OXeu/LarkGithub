//! doc url: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/ai/speech_to_text-v1/speech/file_recognize>
// Code generated by gen_api. DO NOT EDIT.

use serde::{Deserialize, Serialize};

use crate::api::BaseResp;
use crate::api::HasBaseResp;
use crate::core::model::{ApiRequest, CommonResponse, MethodOption, ReqParam, StreamReqParam};
use crate::{
    core::{http_client::HttpClient, store::Store},
    error::Error,
};

use crate::api::gen::ai::AiService;

impl<'c, IStore: Store, IClient: HttpClient> AiService<'c, IStore, IClient> {
    /// **api 版本: 2024-01-03T03:20:41+00:00**
    ///
    /// ## 识别语音文件 (ASR)
    ///
    /// 语音文件识别接口，上传整段语音文件进行一次性识别。接口适合 60 秒以内音频识别。
    ///
    /// 单租户限流：20QPS，同租户下的应用没有限流，共享本租户的 20QPS 限流。免费版不支持调用。
    ///
    /// doc: <https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/ai/speech_to_text-v1/speech/file_recognize>
    ///
    /// new doc: <https://open.larkoffice.com/document/server-docs/ai/speech_to_text-v1/file_recognize>
    ///
    /// api url: <https://open.feishu.cn/document_portal/v1/document/get_detail?fullPath=%2Fserver-docs%2Fai%2Fspeech_to_text-v1%2Ffile_recognize>
    pub async fn recognize_speech_file(
        &self,
        req: RecognizeSpeechFileReq,
    ) -> Result<(RecognizeSpeechFileResp, CommonResponse), Error> {
        self.recognize_speech_file_with_opt(req, Default::default())
            .await
    }

    /// 参见 [recognize_speech_file](#method.recognize_speech_file) 函数
    pub async fn recognize_speech_file_with_opt(
        &self,
        req: RecognizeSpeechFileReq,
        method_option: MethodOption,
    ) -> Result<(RecognizeSpeechFileResp, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_recognize_speech_file(&req) {
                tracing::info!("[lark] Ai#RecognizeSpeechFile **mocking** api");
                return f(req);
            }
        }

        tracing::info!("[lark] Ai#RecognizeSpeechFile call api");

        let req = ApiRequest {
            scope: "Ai",
            api: "RecognizeSpeechFile",
            method: http::Method::POST,
            url: String::new()
                + self.cli.open_base_url.as_ref()
                + "/open-apis/speech_to_text/v1/speech/file_recognize",
            param_data: req.gen_param(),
            method_option,
            need_tenant_access_token: true,
            ..Default::default()
        };

        let (resp, common_resp): (RecognizeSpeechFileRespInner, _) = self.cli.do_req(req).await?;
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
pub struct RecognizeSpeechFileReq {
    /// 语音资源
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "speech")]
    pub speech: SpeechSubReq,
    /// 配置属性
    ///
    /// **是否必填**: 是
    #[api(kind = "body", name = "config")]
    pub config: FileConfigSubReq,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct SpeechSubReq {
    /// pcm格式音频文件（文件识别）或音频分片（流式识别）经base64编码后的内容
    ///
    /// **示例值**: "PdmrfE267Cd/Z9KpmNFh71A2PSJZxSp7+8upCg=="
    #[serde(
        rename = "speech",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub speech: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct FileConfigSubReq {
    /// 仅包含字母数字和下划线的 16 位字符串作为文件的标识，用户生成
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "qwe12dd34567890w"
    #[serde(
        rename = "file_id",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub file_id: String,
    /// 语音格式，目前仅支持：pcm
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "pcm"
    #[serde(
        rename = "format",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub format: String,
    /// 引擎类型，目前仅支持：16k_auto 中英混合
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "16k_auto"
    #[serde(
        rename = "engine_type",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub engine_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
struct RecognizeSpeechFileRespInner {
    #[serde(flatten)]
    data: Option<RecognizeSpeechFileResp>,
    #[serde(flatten)]
    base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecognizeSpeechFileResp {
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
    /// 语音识别后的文本信息
    ///
    /// **是否必填**: 是
    ///
    /// **示例值**: "你好，使用飞书吧"
    #[serde(
        rename = "recognition_text",
        deserialize_with = "crate::utils::serde_helper::null_to_default"
    )]
    pub recognition_text: String,
}

#[cfg(feature = "test-util")]
mod test_utils {
    use std::sync::Arc;

    use self::gen::ai::AiServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    pub trait MockFunc:
        Fn(RecognizeSpeechFileReq) -> Result<(RecognizeSpeechFileResp, CommonResponse), Error>
        + Send
        + Sync
        + 'static
    {
    }
    impl<
            T: Fn(
                    RecognizeSpeechFileReq,
                ) -> Result<(RecognizeSpeechFileResp, CommonResponse), Error>
                + Send
                + Sync
                + 'static,
        > MockFunc for T
    {
    }

    impl<'c, IStore: Store, IClient: HttpClient> AiServiceMocker<'c, IStore, IClient> {
        pub fn mock_recognize_speech_file<F: MockFunc>(
            &self,
            f: F,
        ) -> MockerBuilder<Mocker, RecognizeSpeechFileReq, RecognizeSpeechFileResp, Arc<dyn MockFunc>>
        {
            MockerBuilder::new(self.cli.instance_id, Arc::new(f))
        }

        pub(super) fn get_mock_recognize_speech_file(
            &self,
            req: &RecognizeSpeechFileReq,
        ) -> Option<Arc<dyn MockFunc>> {
            do_mock::<Mocker, RecognizeSpeechFileReq, RecognizeSpeechFileResp, Arc<dyn MockFunc>>(
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
        api::gen::ai::recognize_speech_file::{
            RecognizeSpeechFileReq, RecognizeSpeechFileResp, RecognizeSpeechFileRespInner,
        },
        core::{model::CommonResponse, Lark},
    };

    #[tokio::test]
    async fn test_mock() {
        let lark = Lark::new("".to_owned(), "".into());
        let mocker = lark
            .ai()
            .mock()
            .mock_recognize_speech_file(|_| {
                Ok((
                    RecognizeSpeechFileResp::default(),
                    CommonResponse::default(),
                ))
            })
            .build();
        let res = lark
            .ai()
            .recognize_speech_file(RecognizeSpeechFileReq::default())
            .await;
        assert!(res.is_ok());
        mocker.clear();
        let res = lark
            .ai()
            .recognize_speech_file(RecognizeSpeechFileReq::default())
            .await;
        assert!(res.is_err());
    }

    const REQ: &str = r#"{
    "speech": {
        "speech": "PdmrfE267Cd/Z9KpmNFh71A2PSJZxSp7+8upCg=="
    },
    "config": {
        "file_id": "qwe12dd34567890w",
        "format": "pcm",
        "engine_type": "16k_auto"
    }
}"#;

    #[test]
    fn test_req() {
        if REQ == "{}" {
            return;
        }
        if let Err(e) = serde_json::from_str::<super::RecognizeSpeechFileReqBody>(REQ) {
            panic!("{}", e);
        }
    }

    const RESP: &str = r#"{
    "code": 0,
    "msg": "success",
    "data": {
        "recognition_text": "你好，使用飞书吧"
    }
}"#;
    #[test]
    fn test_resp() {
        let res = serde_json::from_str::<RecognizeSpeechFileRespInner>(RESP);
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
