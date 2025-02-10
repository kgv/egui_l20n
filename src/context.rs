use crate::{ID_SOURCE, Localization};
use egui::{Context, Id, ahash::HashMap};
use std::sync::Arc;
use unic_langid::LanguageIdentifier;

/// Localizations
type Localizations = HashMap<LanguageIdentifier, Arc<Localization>>;

/// Extension methods for [`Context`]
pub trait ContextExt {
    fn language_identifier(&self) -> LanguageIdentifier;

    fn language_identifiers(&self) -> Vec<LanguageIdentifier>;

    fn set_language_identifier(&self, language_identifier: LanguageIdentifier);

    fn localization(&self) -> Arc<Localization>;

    fn set_localization(&self, language_identifier: LanguageIdentifier, localization: Localization);
}

impl ContextExt for Context {
    fn language_identifier(&self) -> LanguageIdentifier {
        self.data_mut(|data| {
            data.get_temp_mut_or_default::<LanguageIdentifier>(
                Id::new(ID_SOURCE).with("LanguageIdentifier"),
            )
            .clone()
        })
    }

    fn language_identifiers(&self) -> Vec<LanguageIdentifier> {
        self.data_mut(|data| {
            data.get_temp_mut_or_default::<Localizations>(Id::new(ID_SOURCE).with("Localizations"))
                .keys()
                .cloned()
                .collect()
        })
    }

    fn set_language_identifier(&self, language_identifier: LanguageIdentifier) {
        self.data_mut(|data| {
            data.insert_temp(
                Id::new(ID_SOURCE).with("LanguageIdentifier"),
                language_identifier,
            );
        })
    }

    fn localization(&self) -> Arc<Localization> {
        let language_identifier = self.language_identifier();
        self.data_mut(|data| {
            data.get_temp_mut_or_default::<Localizations>(Id::new(ID_SOURCE).with("Localizations"))
                [&language_identifier]
                .clone()
        })
    }

    fn set_localization(
        &self,
        language_identifier: LanguageIdentifier,
        localization: Localization,
    ) {
        self.data_mut(|data| {
            data.get_temp_mut_or_default::<Localizations>(Id::new(ID_SOURCE).with("Localizations"))
                .insert(language_identifier, Arc::new(localization));
        })
    }
}
