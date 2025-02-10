pub use self::{context::ContextExt, localization::Localization, ui::UiExt};
#[cfg(feature = "macros")]
pub use unic_langid::langid;

const ID_SOURCE: &str = "EguiL20n";

mod context;
mod localization;
pub mod ui;
