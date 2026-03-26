use std::collections::HashMap;

use iced::{
    Element, Fill, Font, Task, font::Weight, widget::{
        Text, button, column, text, text_input, container
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
    Receive(Result<String, (u16, String)>),
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
                self.error = String::new();

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
                                let status = response.status();
                                let text = response.text().await.unwrap_or_default();
                                
                                if status.is_success() {
                                    Ok(text)
                                } else {
                                    Err((status.as_u16(), text))
                                }
                            }
                            Err(e) => Err((0, e.to_string()))
                        }
                    },
                    |result: Result<String, (u16, String)>| {
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
                    Err((status, body)) => {
                        println!("Error status {}: {}", status, body);
                        let translations = &state.translations;

                        self.error = match status {
                            400 => translations["login_invalid"].to_string(),
                            _ => translations["unknown_error"].to_string()
                        };
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
            .on_input(|s| LoginMessage::EmailChanged(s).into())
            .on_submit(LoginMessage::Send.into());
            
        let password_input = text_input(translations["password"].as_str(), &self.password)
            .on_input(|s| LoginMessage::PasswordChanged(s).into())
            .on_submit(LoginMessage::Send.into())
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
            container(text(translations["login_title"].as_str())
                .size(36)
                .font(Font {
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                })
                .width(Fill)
                .center(),
            )
            .padding(10),

            login_form
        ].into()
    }
}

impl From<LoginMessage> for Message {
    fn from(message: LoginMessage) -> Self {
        Self::Login(message)
    }
}
