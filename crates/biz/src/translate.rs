use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use deeplx::DeepLXTranslationResult;
use pkgs::{Error, Json};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct TranslateResult {
    pub code: u16,
    pub id: i64,
    pub data: String,
    pub alternatives: Vec<String>,
    pub source_lang: String,
    pub target_lang: String,
    pub method: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct TranslateResultUnknown {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[async_trait::async_trait]
pub trait TranslateRepo: Send + Sync {
    async fn translate_free(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
        tag_handling: Option<&str>,
    ) -> Result<DeepLXTranslationResult, Error>;
}

pub struct TranslateUsecase {
    repo: Arc<dyn TranslateRepo>,
}

impl TranslateUsecase {
    pub fn new(repo: Arc<dyn TranslateRepo>) -> Self {
        Self { repo }
    }

    pub async fn translate_free(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
        tag_handling: Option<&str>,
    ) -> Result<Response, Error> {
        let res = self
            .repo
            .translate_free(text, source_lang, target_lang, tag_handling)
            .await?;

        match res.code {
            200 => Ok(Json(TranslateResult {
                code: res.code as u16,
                id: res.id,
                data: res.data,
                alternatives: res.alternatives,
                source_lang: res.source_lang,
                target_lang: res.target_lang,
                method: res.method,
            })
            .with_status_code(StatusCode::OK)
            .into_response()),
            _ => Ok(Json(TranslateResultUnknown {
                code: res.code as u16,
                message: res.message,
            })
            .with_status_code(StatusCode::from_u16(res.code as u16).unwrap())
            .into_response()),
        }
    }
}
