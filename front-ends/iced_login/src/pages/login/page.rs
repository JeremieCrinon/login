use std::collections::HashMap;

use iced::{
    Element, Task, widget::{
        button, column, text, Text, text_input
    }
};

use crate::{AppState, Message};
use crate::CONFIG;

#[derive(Debug, Clone)]
pub struct Login {
    email: String,
    password: String,
    error: String,
    working: bool,
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    EmailChanged(String),
    PasswordChanged(String),
    Send,
    Receive(Result<String, String>),
}

impl Login {
    pub fn new() -> (Self, Task<LoginMessage>) {
        (
            Login {
                email: String::new(),
                password: String::new(),
                error: String::new(),
                working: false
            },
            Task::none(),
        )
    }

    pub(crate) fn update(&mut self, message: LoginMessage, state: &AppState) -> Task<Message> {
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
                self.working = true;

                let email = self.email.clone();
                let password = self.password.clone();
                let client = state.reqwest_client.clone();
                let api_url = CONFIG.api_url.clone();

                return Task::perform(
                    async move {
                        let mut body = HashMap::new();
                        body.insert("email", email);
                        body.insert("password", password);

                        let result = client
                            .post(format!("{}/login", api_url))
                            .json(&body)
                            .send()
                            .await;

                        match result {
                            Ok(response) => {
                                match response.text().await {
                                    Ok(text) => Ok(text),
                                    Err(e) => Err(e.to_string()),
                                }
                            }
                            Err(e) => Err(e.to_string())
                        }
                    },
                    |result: Result<String, String>| {
                        LoginMessage::Receive(result).into()
                    }                
                );
            }
            LoginMessage::Receive(result) => {
                self.working = false;
                match result {
                    Ok(res) => {
                        println!("Success, {}", res);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        self.error = state.translations["unknown_error"].to_string();
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view<'a>(&'a self, state: &'a AppState) -> Element<'a, Message> {
        let translations = &state.translations;

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
