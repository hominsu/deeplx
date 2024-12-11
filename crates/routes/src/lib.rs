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
        .with_state(state)
}
