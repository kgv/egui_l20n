use crate::UiExt as _;
use egui::{Response, RichText, Sense, Ui, Widget, WidgetText};
use egui_phosphor::regular::TRANSLATE;

/// Locale button
pub struct LocaleButton {
    title: Option<WidgetText>,
}

impl LocaleButton {
    pub fn new() -> Self {
        Self { title: None }
    }
}

impl Widget for LocaleButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let title = self
            .title
            .unwrap_or_else(|| RichText::new(TRANSLATE).into());
        ui.menu_button(title, |ui| {
            let mut response = ui.allocate_response(Default::default(), Sense::click());
            let mut current_value = ui.language_identifier();
            for selected_value in ui.language_identifiers() {
                let text = selected_value.to_string();
                response |= ui.selectable_value(&mut current_value, selected_value, text);
            }
            if response.changed() {
                ui.set_language_identifier(current_value);
            }
            if response.clicked() {
                ui.close_menu();
            }
        })
        .response
    }
}
