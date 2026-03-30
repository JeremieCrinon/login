use iced::{
    Element, Fill, Font, Padding, Task, font::Weight, widget::{
        button, column, container, text, text_input, row
    }
};

use crate::{AppState, Message};
use crate::CONFIG;
use crate::styles::card;

/// The new_account iced struct
#[derive(Debug, Clone)]
pub struct NewAccount {
    email: String,
    password: String,
    password_confirm: String,
    error: Option<String>,
    working: bool,
}

/// The messages of the new_account page
#[derive(Debug, Clone)]
pub enum NewAccountMessage {
    EmailChanged(String),
    PasswordChanged(String),
    PasswordConfirmChanged(String),
    Send,
}

impl NewAccount {
    pub fn new(state: &AppState) -> (Self, Task<NewAccountMessage>) {
        (
            NewAccount {
                email: state.user_email.clone().unwrap_or("".to_string()),
                password: String::new(),
                password_confirm: String::new(),
                error: None,
                working: false
            },
            Task::none(),
        )
    }

    pub(crate) fn update(&mut self, message: NewAccountMessage, state: &AppState) -> Task<Message> {
        match message {
            NewAccountMessage::EmailChanged(new_email) => {
                self.email = new_email;
                Task::none()
            }
            NewAccountMessage::PasswordChanged(new_password) => {
                self.password = new_password;
                Task::none()
            }
            NewAccountMessage::PasswordConfirmChanged(new_password) => {
                self.password_confirm = new_password;
                Task::none()
            }
            NewAccountMessage::Send => {
                println!("Send");
                Task::none()
            }
        }
    }

    pub fn view<'a>(&'a self, state: &'a AppState) -> Element<'a, Message> {
        // Get the translations from the state
        let translations = &state.translations;

        // If there is an error, we create a text element with the error, else it's None
        let error_text = match &self.error {
            Some(e) => {Some(
                container(
                    text(e.as_str())
                        .style(text::danger)
                )
                .padding(10)
            )},
            None => None
        };

        // We create the form inputs
        let email_input = container(
            text_input(translations["email"].as_str(), &self.email)
                .on_input(|e| NewAccountMessage::EmailChanged(e).into())
                .on_submit(NewAccountMessage::Send.into())
        )
        .padding(Padding {
            top: 0.0,
            left: 10.0,
            right: 10.0,
            bottom: 0.0,
        });
            
        let password_input = container(
            text_input(translations["password"].as_str(), &self.password)
                .on_input(|p| NewAccountMessage::PasswordChanged(p).into())
                .on_submit(NewAccountMessage::Send.into())
                .secure(true)
        )
        .padding(Padding {
            top: 10.0,
            left: 10.0,
            right: 10.0,
            bottom: 0.0,
        });

        let password_confirm_input = container(
            text_input(translations["password_confirm"].as_str(), &self.password_confirm)
                .on_input(|p| NewAccountMessage::PasswordConfirmChanged(p).into())
                .on_submit(NewAccountMessage::Send.into())
                .secure(true)
        )
        .padding(Padding {
            top: 10.0,
            left: 10.0,
            right: 10.0,
            bottom: 0.0,
        });

        let send_button = container(
            if self.working {
                button(text(format!("{}...", translations["loading"])))
            } else {
                button(translations["new_account_send"].as_str())
                    .on_press(NewAccountMessage::Send.into())
            }
        );

        let logout_button = container(
            button(translations["logout"].as_str())
                .on_press(Message::Logout)
        )
        .padding(Padding {
            top: 0.0,
            left: 10.0,
            right: 10.0,
            bottom: 0.0,
        });
        
        let buttons = container(
            row![
                send_button,
                logout_button
            ]
        )
        .padding(10);

        let new_account_form = container(
            column![
                error_text,
                email_input,
                password_input,
                password_confirm_input,
                buttons
            ]
        )
        .center_x(300);

        let new_account_card = container(
            column![
                container(text(translations["new_account_title"].as_str())
                    .size(36)
                    .font(Font {
                        weight: Weight::Bold,
                        ..Font::DEFAULT
                    })
                    .width(Fill)
                    .center(),
                )
                .padding(20),

                container(
                    new_account_form
                )
                .center_x(Fill)
                
            ]

        )
        .center(400)
        .style(card)
        .padding(10);

        container(
            new_account_card
        )
        .center(Fill)
        .into()

    }
}

// Allow NewAccountMessage to be converted to Message with .into()
impl From<NewAccountMessage> for Message {
    fn from(message: NewAccountMessage) -> Self {
        Self::NewAccount(message)
    }
}
