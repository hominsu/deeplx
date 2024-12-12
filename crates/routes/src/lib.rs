use axum::{routing::post, Router};
use biz::translate::TranslateUsecase;
use conf::Config;
use std::sync::{Arc, RwLock};

mod translate;

#[derive(Clone)]
pub struct AppState {
    pub translate_uc: Arc<TranslateUsecase>,
    pub config: Arc<RwLock<Config>>,
}

pub fn router<T>(state: AppState) -> Router {
    Router::new()
        .route("/translate", post(translate::translate_free))
        .route("/v1/translate", post(translate::translate_pro))
        .route("/v2/translate", post(translate::translate_official))
        .with_state(state)
}
