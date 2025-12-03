pub use self::{context::ContextExt, localization::Localization, response::ResponseExt, ui::UiExt};
pub use unic_langid::LanguageIdentifier;
#[cfg(feature = "macros")]
pub use unic_langid::langid;

pub const ID_SOURCE: &str = "EguiL20n";

pub mod prelude {
    pub use crate::{ContextExt as _, Localization, ResponseExt as _, UiExt as _};
}

mod context;
mod localization;
mod response;
pub mod ui;
