use self::locale_button::LocaleButton;
use crate::{ID_SOURCE, Localization};
use egui::{Context, Id, Response, Ui, ahash::HashMap};
use std::sync::Arc;
use unic_langid::LanguageIdentifier;

/// Localizations
type Localizations = HashMap<LanguageIdentifier, Arc<Localization>>;

/// Extension methods for [`Ui`]
pub trait UiExt {
    fn language_identifier(&self) -> LanguageIdentifier;

    fn language_identifiers(&self) -> Vec<LanguageIdentifier>;

    fn set_language_identifier(&self, language_identifier: LanguageIdentifier);

    fn localization(&self) -> Arc<Localization>;

    fn set_localization(&self, language_identifier: LanguageIdentifier, localization: Localization);

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

    fn locale_button(&mut self) -> Response {
        self.add(LocaleButton::new())
    }
}

/// Extension methods for [`Context`]
trait ContextExt {
    fn language_identifier(&self) -> LanguageIdentifier;

    fn language_identifiers(&self) -> Vec<LanguageIdentifier>;

    fn set_language_identifier(&self, language_identifier: LanguageIdentifier);

    fn localization(&self) -> Arc<Localization>;

    fn set_localization(&self, language_identifier: LanguageIdentifier, localization: Localization);
}

impl ContextExt for Context {
    fn language_identifier(&self) -> LanguageIdentifier {
        self.data_mut(|data| {
            data.get_persisted_mut_or_default::<LanguageIdentifier>(
                Id::new(ID_SOURCE).with("LanguageIdentifier"),
            )
            .clone()
        })
    }

    fn language_identifiers(&self) -> Vec<LanguageIdentifier> {
        self.data_mut(|data| {
            data.get_persisted_mut_or_default::<Localizations>(
                Id::new(ID_SOURCE).with("Localizations"),
            )
            .keys()
            .cloned()
            .collect()
        })
    }

    fn set_language_identifier(&self, language_identifier: LanguageIdentifier) {
        self.data_mut(|data| {
            data.insert_persisted(
                Id::new(ID_SOURCE).with("LanguageIdentifier"),
                language_identifier,
            );
        })
    }

    fn localization(&self) -> Arc<Localization> {
        let language_identifier = self.language_identifier();
        self.data_mut(|data| {
            data.get_persisted_mut_or_default::<Localizations>(
                Id::new(ID_SOURCE).with("Localizations"),
            )[&language_identifier]
                .clone()
        })
    }

    fn set_localization(
        &self,
        language_identifier: LanguageIdentifier,
        localization: Localization,
    ) {
        self.data_mut(|data| {
            data.get_persisted_mut_or_default::<Localizations>(
                Id::new(ID_SOURCE).with("Localizations"),
            )
            .insert(language_identifier, Arc::new(localization));
        })
    }
}

pub mod locale_button;
