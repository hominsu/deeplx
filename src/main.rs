use biz::translate::TranslateUsecase;
use data::translate::TranslateRepo;
use deeplx::{Config, DeepLX};
use pkgs::exit::shutdown_signal;
use std::{future::IntoFuture, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let manager = conf::manager("configs/").with_watcher(shutdown_signal());
    let config = manager.config();
    let manager_fu = manager.into_future();

    let translator = Arc::new(DeepLX::new(Config::default()));
    let translate_repo = Arc::new(TranslateRepo::new(translator.clone()));
    let translate_usecase = Arc::new(TranslateUsecase::new(translate_repo.clone()));
    let state = routes::AppState {
        translate_uc: translate_usecase,
        config: config.clone(),
    };
    let app = routes::router::<TranslateRepo>(state).layer(TraceLayer::new_for_http());

    let addr = config.read().unwrap().addr.clone();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let serve_fu = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .into_future();

    let _ = tokio::join!(serve_fu, manager_fu);
}
