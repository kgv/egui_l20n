pub use self::{context::ContextExt, localization::Localization, response::ResponseExt, ui::UiExt};
pub use unic_langid::LanguageIdentifier;
#[cfg(feature = "macros")]
pub use unic_langid::langid;

pub const ID_SOURCE: &str = "EguiL20n";

mod context;
mod localization;
mod response;
pub mod ui;
