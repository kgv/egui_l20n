use crate::UiExt as _;
use egui::Response;

/// Extension methods for [`Response`]
pub trait ResponseExt {
    fn on_disabled_hover_localized(self, key: &str) -> Response;

    fn on_hover_localized(self, key: &str) -> Response;
}

impl ResponseExt for Response {
    fn on_disabled_hover_localized(self, key: &str) -> Response {
        self.on_disabled_hover_ui(|ui| {
            ui.label(ui.localize(key));
        })
    }

    fn on_hover_localized(self, key: &str) -> Response {
        self.on_hover_ui(|ui| {
            ui.label(ui.localize(key));
        })
    }
}
