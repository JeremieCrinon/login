use iced::{
    Element, Task, widget::{
        button, column, text, Text, text_input
    }
};
use std::collections::HashMap;

use crate::Message;

#[derive(Debug, Clone)]
pub struct Login {
    email: String,
    password: String,
    error: String,
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    EmailChanged(String),
    PasswordChanged(String),
    Send,
}

impl Login {
    pub fn new() -> (Self, Task<LoginMessage>) {
        (
            Login {
                email: String::new(),
                password: String::new(),
                error: String::new()
            },
            Task::none(),
        )
    }

    pub(crate) fn update(&mut self, message: LoginMessage) -> Task<Message> {
        match message {
            LoginMessage::EmailChanged(new_email) => {
                println!("New email: {}", &new_email);
                self.email = new_email;
                Task::none()
            }
            LoginMessage::PasswordChanged(new_password) => {
                println!("New password: {}", &new_password);
                self.password = new_password;
                Task::none()
            }
            LoginMessage::Send => {
                println!("Send form");
                self.error = "This is just a quick test of error displaying".to_string();
                Task::none()
            }
        }
    }

    pub fn view<'a>(&'a self, translations: &'a HashMap<String, String>) -> Element<'a, Message> {
        let error_text: Option<Text> = if self.error.is_empty() {None} else {Some(
            text(self.error.as_str())
                .style(text::danger)
        )};

        let email_input = text_input(translations["email"].as_str(), &self.email)
            .on_input(|s| LoginMessage::EmailChanged(s).into());
            
        let password_input = text_input(translations["password"].as_str(), &self.password)
            .on_input(|s| LoginMessage::PasswordChanged(s).into())
            .secure(true);

        let send_button = button(translations["login_send"].as_str())
            .on_press(LoginMessage::Send.into());

        let login_form = column![
            error_text,
            email_input,
            password_input,
            send_button
        ];

        column![
            text(translations["login_title"].as_str()),
            login_form
        ].into()
    }
}

impl From<LoginMessage> for Message {
    fn from(message: LoginMessage) -> Self {
        Self::Login(message)
    }
}
