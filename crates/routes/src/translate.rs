use crate::AppState;

use axum::{extract::State, response::Response};
use pkgs::{Error, Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct PayloadFree {
    pub text: String,
    pub source_lang: String,
    pub target_lang: String,
    pub tag_handling: Option<String>,
}

pub async fn translate_free(
    State(state): State<AppState>,
    Json(payload): Json<PayloadFree>,
) -> Result<Response, Error> {
    let text = payload.text;
    let source_lang = payload.source_lang;
    let target_lang = payload.target_lang;
    let tag_handling = payload.tag_handling.as_deref();

    if tag_handling.is_some_and(|tag_handling| !matches!(tag_handling, "html" | "xml")) {
        return Err(Error::InvalidTagHandling);
    }

    state
        .translate_uc
        .translate_free(&text, &source_lang, &target_lang, tag_handling)
        .await
}
