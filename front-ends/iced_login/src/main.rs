mod pages;
mod translator;
mod config;
mod styles;
mod helpers;

use std::collections::HashMap;
use iced::{
    Element, Task, keyboard, Subscription, Event, 
    widget::operation::{focus_next, focus_previous}
};

use pages::login::{Login, LoginMessage};
use pages::test::{Test, TestMessage};
use pages::loading::Loading;
use translator::translator::Translator;
use helpers::get_token_from_keychain;
use config::CONFIG;

/// This struct contains the things we need to pass to pages.
pub struct AppState {
    pub translations: HashMap<String, String>,
    pub reqwest_client: reqwest::Client,
    pub token: Option<String>,
    pub user_email: Option<String>,
    pub user_roles: Option<Vec<String>>,
}

/// This contains the list of the pages. If you add a new page, and it here
#[derive(Debug, Clone)]
pub enum Page {
    Login(Login),
    Test(Test),
    Loading(Loading),
}

/// This is the main iced struct. It will handle the displaying of the other pages
pub struct UI {
    page: Page, // The current page we are displaying
    state: AppState
}

/// This is the main message. All the child pages will be extending this enum with their messages
#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    RedirectUser, // Redirect a user to where they should be depending on where they are
    RedirectUserHandler {token: String, roles: Vec<String>, email: String}, // Handle the redirection and state update if the user is logged in after a redirect user call 
    ChangeToken(String), // Change the token to a new one
    Logout,
    
    // Pages
    Login(LoginMessage), // When a child calls a message of itself, it will be actually a Message containing it's own message
    Test(TestMessage),

    // Tab nav
    FocusNext,
    FocusPrevious,
}

impl UI {
    pub fn new() -> (Self, Task<Message>) {
        let translator = Translator::new();
        let available = translator.available_locales();

        // Take the first system locale available in the translator or use en
        let locale = sys_locale::get_locales()
            .find_map(|sys_locale| {
                let lang = sys_locale.split(&['-', '_'][..]).next().unwrap_or(&sys_locale);
                available.contains(&lang).then_some(lang.to_string())
            })
            .unwrap_or_else(|| "en".to_string());

        let translations = translator.get_translation(&locale);

        let client = reqwest::Client::new(); // Create a single reqwest client as creating a new one for each request is slow

        let state = AppState {translations, reqwest_client: client, token: None, user_email: None, user_roles: None}; // Create the appState that will contain eveything the pages needs

        (
            UI {
                page: Page::Loading(Loading::new()), // Start with the loading page for the time working to know where the user should be 
                // page: Page::Login(Login::new().0), // Temporary to test login page
                state: state // Add the appState here
            },
            Task::done(Message::RedirectUser),
        )
    }
    
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match (&mut self.page, message) {
            (_, Message::Navigate(page)) => {
                self.page = page;
                Task::none()
            }
            (_, Message::FocusNext) => {
                focus_next()
            }
            (_, Message::FocusPrevious) => {
                focus_previous()
            }
            (_, Message::ChangeToken(token)) => {
                // Set the token in memory for quick access on app's lifetime
                self.state.token = Some(token.clone());

                // Create a new keyring entry for the token
                let entry = match keyring::Entry::new(CONFIG.app_name.as_str(), "token") {
                    Ok(e) => e,
                    Err(e) => {
                        println!("Error creating keyring entry: {}", e);
                        //TODO: Redirect to an error page with a button to return to where the user should be
                        return Task::none();
                    }
                };

                // Set the token in the entry
                if let Err(e) = entry.set_password(token.as_str()) {
                    println!("Error storing token in keyring: {}", e);
                    //TODO: Redirect to an error page with a button to return to where the user should be
                    return Task::none();
                }

                Task::done(Message::RedirectUser)
            }
            (_, Message::Logout) => {
                if let Ok(entry) = keyring::Entry::new(CONFIG.app_name.as_str(), "token") {
                    let _ = entry.delete_credential();  // Ignore any error
                }

                self.state.token = None;

                return Task::done(Message::Navigate(Page::Login(Login::new().0)));
            }
            (_, Message::RedirectUser) => {
                self.page = Page::Loading(Loading::new());

                let state = &self.state;
                let client = state.reqwest_client.clone();
                let api_url = CONFIG.api_url.clone();

                let token = get_token_from_keychain();

                // If we don't already have a token in keychain, it means the user isn't authenticated
                let token = match token {
                    Some(t) => t,
                    None => return Task::done(Message::Navigate(Page::Login(Login::new().0))),
                };

                Task::perform(
                    async move {
                        let result = client
                            .get(format!("{}/user-infos", api_url))
                            .header("Authorization", format!("Bearer {}", token))
                            .send()
                            .await;
                        
                        match result {
                            Ok(response) => {
                                let status = response.status();
                                if status == reqwest::StatusCode::UNAUTHORIZED {
                                    Err("unauthorized".to_string())
                                } else {
                                    let text = match response.text().await {
                                        Ok(r) => r,
                                        Err(e) => {
                                            println!("Error parsing the response into text: {}", e);
                                            return Err("unknown".to_string());
                                        }
                                    };

                                    let json: serde_json::Value = match serde_json::from_str(&text) {
                                        Ok(j) => j,
                                        Err(e) => {
                                            println!("Error parsing json from response: {}", e);
                                            return Err("unknown".to_string());
                                        }
                                    };

                                    let roles = match json["roles"].as_array() {
                                        Some(r) => r.clone(),
                                        None => {
                                            println!("Error getting the roles from parsed response");
                                            return Err("unknown".to_string());
                                        }
                                    };

                                    let roles: Vec<String> = roles.iter().map(|r| r.as_str().unwrap_or_default().to_string()).collect();

                                    let email = match json["user_mail"].as_str() {
                                        Some(e) => e,
                                        None  => {
                                            println!("Error getting the email from parsed response");
                                            return Err("unknown".to_string());
                                        }
                                    };

                                    Ok((token, roles, email.to_string()))
                                }
                            }
                            Err(e) => {
                                println!("Error getting the user from token: {}", e);
                                Err(e.to_string())
                            }
                        }
                    },
                    |result| {
                        match result {
                            Ok((token, roles, email)) => {
                                Message::RedirectUserHandler { token, roles, email }
                            },
                            Err(e) if e == "unauthorized" => Message::Logout,
                            Err(_) => Message::Navigate(Page::Test(Test::new().0)), // TODO: Redirect to error page
                        }
                    }
                )            
            }
            (_, Message::RedirectUserHandler { token, roles, email }) => {
                // Set in the state infos about the user
                self.state.token = Some(token); // We also set it here, the keychain is slow compared to memory
                self.state.user_roles = Some(roles.clone());
                self.state.user_email = Some(email);

                if roles.contains(&"new_account".to_string()) {
                    println!("Redirect to new account page");
                    //TODO: Redirect to new account page
                    return Task::done(Message::Navigate(Page::Test(Test::new().0)));
                }

                if roles.contains(&"unverified_email".to_string()) {
                    println!("Redirect to unverified email page");
                    //TODO: Redirect to unverified email page
                    return Task::done(Message::Navigate(Page::Test(Test::new().0)));
                }

                //TODO: Redirect to the dashboard
                return Task::done(Message::Navigate(Page::Test(Test::new().0)));
            }
            (Page::Login(page), Message::Login(msg)) => {
                page.update(msg, &self.state) // Pass to the child page it's own message
            }
            (Page::Test(page), Message::Test(msg)) => {
                page.update(msg)
            }
            (page, message) => {
                panic!("Incorrect message routing:\npage {:?}\nreceived message {:?}", page, message)
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.page { // Display the correct page depending on which is chosen
            Page::Login(login) => login.view(&self.state),
            Page::Test(test) => test.view(&self.state),
            Page::Loading(loading) => loading.view(&self.state),
        }
    }

    /// This is for tab navigation
    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _status, _id| {
            if let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event {
                if let keyboard::Key::Named(keyboard::key::Named::Tab) = key {
                    return if modifiers.shift() {
                        Some(Message::FocusPrevious)
                    } else {
                        Some(Message::FocusNext)
                    };
                }
            }
            None
        })
    }
}

fn main() -> iced::Result {
    iced::application(UI::new, UI::update, UI::view)
        .subscription(UI::subscription)
        .title(CONFIG.app_display_name.as_str())
        .run()
}
