use biz::translate::TranslateRepo as BizTranslateRepo;
use deeplx::{DeepLX, DeepLXTranslationResult};
use pkgs::Error;
use std::sync::Arc;

pub struct TranslateRepo {
    translator: Arc<DeepLX>,
}

impl TranslateRepo {
    pub fn new(translator: Arc<DeepLX>) -> Self {
        Self { translator }
    }
}

#[async_trait::async_trait]
impl BizTranslateRepo for TranslateRepo {
    async fn translate_free(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
        tag_handling: Option<&str>,
    ) -> Result<DeepLXTranslationResult, Error> {
        match self
            .translator
            .translate(source_lang, target_lang, text, tag_handling, None)
            .await
        {
            Ok(res) => Ok(res),
            Err(e) => {
                tracing::error!(%e);
                Err(Error::InternalServerError)
            }
        }
    }
}
