use std::collections::HashMap;

use iced::{
    Element, Fill, Font, Padding, Task, font::Weight, widget::{
        button, column, container, text, text_input, row
    }
};

use crate::{AppState, Message};
use crate::CONFIG;
use crate::styles::card;

/// The unverfied_email iced struct
#[derive(Debug, Clone)]
pub struct VerifyEmail {
    code: String,
    error: Option<String>,
    working: bool,
}

/// The messages of the unverified_email page
#[derive(Debug, Clone)]
pub enum VerifyEmailMessage {
    CodeChanged(String),
    Send,
    Receive(Result<String, (u16, String)>),
}

impl VerifyEmail {
    pub fn new() -> (Self, Task<VerifyEmailMessage>) {
        (
            VerifyEmail {
                code: String::new(),
                error: None,
                working: false
            },
            Task::none(),
        )
    }

    pub(crate) fn update(&mut self, message: VerifyEmailMessage, state: &AppState) -> Task<Message> {
        match message {
            VerifyEmailMessage::CodeChanged(new_code) => {
                self.code = new_code;
                Task::none()
            }
            VerifyEmailMessage::Send => {
                self.working = true;
                self.error = None;

                // Clone everything as self won't be available in the async call
                let client = state.reqwest_client.clone();           
                let code = self.code.clone();
                let api_url = CONFIG.api_url.clone();
                let token = state.token.clone();

                let token = match token {
                    Some(t) => t,
                    None => return Task::done(Message::RedirectUser),
                };

                return Task::perform(
                    async move {
                        // Create the request body as a hashmap and put the content in
                        let mut body = HashMap::new();
                        body.insert("code", code);

                        // Make the request with reqwest
                        let result = client
                            .post(format!("{}/verify-email", api_url))
                            .json(&body)
                            .header("Authorization", format!("Bearer {}", token))
                            .send()
                            .await;

                        match result {
                            Ok(response) => { // If the response if ok, it doesn't mean the status code is 2xx, it just means the request got a response
                                let status = response.status(); // Get the status code
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
                        VerifyEmailMessage::Receive(result).into()
                    }
                );
            }
            VerifyEmailMessage::Receive(result) => {
                self.working = false;
                let translations = &state.translations;

                match result {
                    Ok(_) => {
                        // Everything is fine, we call the RedirectUser message that will updated the roles and redirect the user where they should be
                        return Task::done(Message::RedirectUser);
                    }
                    Err((status, body)) => {
                        // Print the error for debuging
                        println!("Error status {}: {}", status, body);

                        // If the error is 401, we call the redirect user message that will check if the token is valid
                        if status == 401 {
                            return Task::done(Message::RedirectUser);
                        }

                        // If the status is 400, the code isn't valid, we display a message to the user
                        if status == 400 {
                            self.error = Some(translations["verify_email_invalid_code"].to_string())
                        }

                        // Else, it's an unexpected error
                        self.error = Some(translations["unknown_error"].to_string());
                        return Task::none();
                    }
                }
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
        let code_input = container(
            text_input(translations["verify_email_code"].as_str(), &self.code)
                .on_input(|e| VerifyEmailMessage::CodeChanged(e).into())
                .on_submit(VerifyEmailMessage::Send.into())
        )
        .padding(Padding {
            top: 0.0,
            left: 10.0,
            right: 10.0,
            bottom: 0.0,
        });
           
        let send_button = container(
            if self.working {
                button(text(format!("{}...", translations["loading"])))
            } else {
                button(translations["verify_email_send"].as_str())
                    .on_press(VerifyEmailMessage::Send.into())
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

        let verify_email_form = container(
            column![
                error_text,
                code_input,
                buttons
            ]
        )
        .center_x(400);

        let verify_email_card = container(
            column![
                container(text(translations["verify_email_title"].as_str())
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
                    verify_email_form
                )
                .center_x(Fill)
                
            ]

        )
        .center_x(500)
        .center_y(400)
        .style(card)
        .padding(10);

        container(
            verify_email_card
        )
        .center(Fill)
        .into()

    }
}

// Allow VerifyEmailMessage to be converted to Message with .into()
impl From<VerifyEmailMessage> for Message {
    fn from(message: VerifyEmailMessage) -> Self {
        Self::VerifyEmail(message)
    }
}
