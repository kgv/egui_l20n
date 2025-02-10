use self::locale_button::LocaleButton;
use crate::{ContextExt, Localization};
use egui::{Response, Ui};
use fluent_content::Content;
use std::sync::Arc;
use unic_langid::LanguageIdentifier;

/// Extension methods for [`Ui`]
pub trait UiExt {
    fn language_identifier(&self) -> LanguageIdentifier;

    fn language_identifiers(&self) -> Vec<LanguageIdentifier>;

    fn set_language_identifier(&self, language_identifier: LanguageIdentifier);

    fn localization(&self) -> Arc<Localization>;

    fn set_localization(&self, language_identifier: LanguageIdentifier, localization: Localization);

    fn localize(&self, key: &str) -> String;

    fn try_localize(&self, key: &str) -> Option<String>;

    fn locale_button(&mut self) -> Response;
}

impl UiExt for Ui {
    fn language_identifier(&self) -> LanguageIdentifier {
        self.ctx().language_identifier()
    }

    fn language_identifiers(&self) -> Vec<LanguageIdentifier> {
        self.ctx().language_identifiers()
    }

    fn set_language_identifier(&self, language_identifier: LanguageIdentifier) {
        self.ctx().set_language_identifier(language_identifier)
    }

    fn localization(&self) -> Arc<Localization> {
        self.ctx().localization()
    }

    fn set_localization(
        &self,
        language_identifier: LanguageIdentifier,
        localization: Localization,
    ) {
        self.ctx()
            .set_localization(language_identifier, localization)
    }

    fn localize(&self, key: &str) -> String {
        self.try_localize(key).unwrap_or_default()
    }

    fn try_localize(&self, key: &str) -> Option<String> {
        self.localization().content(key)
    }

    fn locale_button(&mut self) -> Response {
        self.add(LocaleButton::new())
    }
}

pub mod locale_button;
