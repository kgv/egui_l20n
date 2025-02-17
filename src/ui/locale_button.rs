use crate::UiExt as _;
use egui::{Response, RichText, Sense, TextStyle, Ui, Widget};
use egui_phosphor::regular::TRANSLATE;

/// Locale button
pub struct LocaleButton {
    text: RichText,
}

impl LocaleButton {
    pub fn new() -> Self {
        Self {
            text: RichText::new(TRANSLATE),
        }
    }

    /// See [`RichText::size`].
    #[inline]
    pub fn size(self, size: f32) -> Self {
        Self {
            text: self.text.size(size),
        }
    }

    /// See [`RichText::text_style`].
    #[inline]
    pub fn text_style(self, text_style: TextStyle) -> Self {
        Self {
            text: self.text.text_style(text_style),
        }
    }
}

impl Widget for LocaleButton {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.menu_button(self.text, |ui| {
            let mut response = ui.allocate_response(Default::default(), Sense::click());
            let mut current_value = ui.language_identifier();
            for selected_value in ui.language_identifiers() {
                let text = RichText::new(selected_value.to_string()).heading();
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
