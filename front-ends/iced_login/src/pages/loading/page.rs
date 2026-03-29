use iced::{
    Element, Length::Fill, widget::{
        column, container, text
    }
};
use iced_aw::Spinner;

use crate::{AppState, Message};

/// The loading iced struct
#[derive(Debug, Clone)]
pub struct Loading {}

impl Loading {
    pub fn new() -> Self {
        Loading {}
    }

    pub fn view<'a>(&'a self, state: &'a AppState) -> Element<'a, Message> {
        // Get the translations from the AppState
        let translations = &state.translations;

        let loading_text = container(
            text(format!("{}...", translations["loading"]))
                .size(24)
        )
        .center_x(Fill);

        let spinner = container(
            Spinner::new()
                .width(32)
                .height(32)
        )
        .center_x(Fill)
        .padding(10);

        let loading_content = column![
            spinner,
            loading_text
        ];

        container(
            loading_content
        )
        .center(Fill)
        .padding(10)
        .into()


    }
}
