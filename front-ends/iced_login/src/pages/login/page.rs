use std::collections::HashMap;

use iced::{
    Element, Fill, Font, Padding, Task, font::Weight, widget::{
        button, column, container, text, text_input
    }
};

use crate::{AppState, Message, Page, pages::test::Test};
use crate::CONFIG;
use crate::styles::card;

/// The login iced struct
#[derive(Debug, Clone)]
pub struct Login {
    email: String,
    password: String,
    error: String,
    working: bool,
}

/// The messages of this page
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
                self.email = new_email;
                Task::none()
            }
            LoginMessage::PasswordChanged(new_password) => {
                self.password = new_password;
                Task::none()
            }
            LoginMessage::Send => {
                self.working = true;
                self.error = String::new();

                // Clone everything as self won't be available in the async call
                let email = self.email.clone();
                let password = self.password.clone();
                let client = state.reqwest_client.clone();
                let api_url = CONFIG.api_url.clone();

                return Task::perform(
                    async move {
                        // Create the request body as a hashmap and put the content in
                        let mut body = HashMap::new();
                        body.insert("email", email);
                        body.insert("password", password);

                        // Make the request with reqwest
                        let result = client
                            .post(format!("{}/login", api_url))
                            .json(&body) // Put the body as json
                            .send()
                            .await;

                        match result {
                            Ok(response) => { // If the result is ok, it doesn't mean the request is successfull, just that it got a response
                                let status = response.status(); // Get the response status code
                                let text = response.text().await.unwrap_or_default(); // Get the response content as text
                                
                                if status.is_success() { // If the status code is 2xx
                                    Ok(text)
                                } else { // Else return an error with the code
                                    Err((status.as_u16(), text))
                                }
                            }
                            Err(e) => Err((0, e.to_string())) // If the result isn't ok, it means we didn't get a response from the server at all, so we just put a 0 status code 
                        }
                    },
                    |result: Result<String, (u16, String)>| {
                        LoginMessage::Receive(result).into() // When the async job is finished, we call the receive message that will handle the update 
                    }                
                );
            }
            LoginMessage::Receive(result) => {
                self.working = false;
                let translations = &state.translations;

                match result {
                    Ok(res) => {
                        // Parse the json so we can use it
                        let json: serde_json::Value = match serde_json::from_str(&res) {
                            Ok(v) => v,
                            Err(e) => {
                                println!("Error parsing json from login response: {}", e);
                                self.error = translations["unknown_error"].to_string();
                                return Task::none();
                            }
                        };

                        // Get the token from the response
                        let token = match json["token"].as_str() {
                            Some(t) => t,
                            None => {
                                println!("No token in response");
                                self.error = translations["unknown_error"].to_string();
                                return Task::none();
                            }
                        };

                        // Create a new keyring entry for the token
                        let entry = match keyring::Entry::new(CONFIG.app_name.as_str(), "token") {
                            Ok(e) => e,
                            Err(e) => {
                                println!("Error creating keyring entry: {}", e);
                                self.error = translations["unknown_error"].to_string();
                                return Task::none();
                            }
                        };

                        // Set the token in the entry
                        if let Err(e) = entry.set_password(token) {
                            println!("Error storing token in keyring: {}", e);
                            self.error = translations["unknown_error"].to_string();
                            return Task::none();
                        }

                        // Return a message to navigate to the test page (temporary, it will return to an handler to redirect them where they should be in the future)
                        return Task::done(Message::Navigate(Page::Test(Test::new().0)));
                    }
                    Err((status, body)) => {
                        // Print the error for debuging
                        println!("Error status {}: {}", status, body);

                        // If the error is 400, the credentials are invalid (expected error) else it's an unexpected error
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
        // Get the translations from the state
        let translations = &state.translations;

        // If there is an error we create a text element with the error, else it's None
        let error_text = if self.error.is_empty() {None} else {Some(
            container(
                text(self.error.as_str())
                    .style(text::danger)
            )
            .padding(10)
        )};

        // We create the form inputs
        let email_input = container(
            text_input(translations["email"].as_str(), &self.email)
                .on_input(|s| LoginMessage::EmailChanged(s).into())
                .on_submit(LoginMessage::Send.into())
        )
        .padding(Padding {
            top: 0.0,
            left: 10.0,
            right: 10.0,
            bottom: 0.0,
        });
            
        let password_input = container(
            text_input(translations["password"].as_str(), &self.password)
                .on_input(|s| LoginMessage::PasswordChanged(s).into())
                .on_submit(LoginMessage::Send.into())
                .secure(true)
        )
        .padding(Padding {
            top: 10.0,
            left: 10.0,
            right: 10.0,
            bottom: 0.0,
        });

        let send_button = container(
            button(translations["login_send"].as_str())
                .on_press(LoginMessage::Send.into())
        )
        .padding(Padding {
            top: 10.0,
            left: 10.0,
            right: 10.0,
            bottom: 0.0,
        });

        let login_form = container(
            column![
                error_text,
                email_input,
                password_input,
                send_button
            ]
        )
        .center_x(300);

        let login_card = container(
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
                .padding(20),

                container(
                    login_form
                )
                .center_x(Fill)
                
            ]

        )
        .center(300)
        .style(card)
        .padding(10);

        container(
            login_card
        )
        .center(Fill)
        .into()
    }
}

// Allow LoginMessage to be converted to Message with .into()
impl From<LoginMessage> for Message {
    fn from(message: LoginMessage) -> Self {
        Self::Login(message)
    }
}
