pub use self::{localization::Localization, ui::UiExt};
#[cfg(feature = "macros")]
pub use unic_langid::langid;

const ID_SOURCE: &str = "EguiL20n";

pub mod localization;
pub mod ui;
