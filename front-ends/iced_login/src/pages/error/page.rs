use iced::{
    Element, Length::Fill, Padding, widget::{
        button, column, container, row, text
    }
};

use crate::{AppState, Message};

/// The error iced struct
#[derive(Debug, Clone)]
pub struct Error {}

impl Error {
    pub fn new() -> Self {
        Error {}
    }

    pub fn view<'a>(&'a self, state: &'a AppState) -> Element<'a, Message> {
        // Get the translations from the AppState
        let translations = &state.translations;

        let error_text = container(
            text(format!("{}", translations["error_content"]))
                .size(24)
        )
        .center_x(Fill);

        let retry_button = button(translations["error_retry"].as_str())
            .on_press(Message::RedirectUser);

        let logout_button = container(
            button(translations["error_logout"].as_str())
                .on_press(Message::Logout)
        )
            .padding(Padding{
                left: 10.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0
            });
 
        let buttons = container(
            row![
                retry_button,
                logout_button
            ]
        )
            .center_x(Fill)
            .padding(10);

        let error_content = column![
            error_text,
            buttons
        ];

        container(
            error_content
        )
        .center(Fill)
        .padding(10)
        .into()


    }
}
