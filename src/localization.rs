use fluent::{FluentResource, concurrent::FluentBundle};
use std::ops::Deref;
use tracing::{Level, enabled, error};
use unic_langid::LanguageIdentifier;

/// Localization
pub struct Localization(FluentBundle<FluentResource>);

impl Localization {
    pub fn new(language_identifier: LanguageIdentifier) -> Self {
        let bundle = FluentBundle::new_concurrent(vec![language_identifier.into()]);
        Localization(bundle)
    }

    pub fn with_sources(&mut self, sources: &[&str]) {
        for &source in sources {
            self.with_source(source);
        }
    }

    pub fn with_source(&mut self, source: &str) {
        let resource = match FluentResource::try_new(source.to_owned()) {
            Ok(resource) => resource,
            Err((resource, errors)) => {
                if enabled!(Level::WARN) {
                    for error in errors {
                        error!(%error);
                    }
                }
                resource
            }
        };
        if let Err(errors) = self.0.add_resource(resource) {
            if enabled!(Level::WARN) {
                for error in errors {
                    error!(%error);
                }
            }
        }
    }
}

impl Default for Localization {
    fn default() -> Self {
        Self::new(LanguageIdentifier::default())
    }
}

impl Deref for Localization {
    type Target = FluentBundle<FluentResource>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
