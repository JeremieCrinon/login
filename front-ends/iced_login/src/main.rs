mod pages;
mod translator;
mod config;
mod styles;

use std::collections::HashMap;
use iced::{
    Element, Task, keyboard, Subscription, Event, 
    widget::operation::{focus_next, focus_previous}
};

use pages::login::{Login, LoginMessage};
use pages::test::{Test, TestMessage};
use pages::loading::Loading;
use translator::translator::Translator;

use config::CONFIG;

/// This struct contains the things we need to pass to pages.
pub struct AppState {
    pub translations: HashMap<String, String>,
    pub reqwest_client: reqwest::Client,
    pub token: Option<String>,
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
    ChangeToken(String), // Change the token to a new one
    
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

        let state = AppState {translations, reqwest_client: client, token: None}; // Create the appState that will contain eveything the pages needs

        (
            UI {
                page: Page::Loading(Loading::new()), // Start with the loading page for the time working to know where the user should be 
                // page: Page::Login(Login::new().0), // Temporary to test login page
                state: state // Add the appState here
            },
            Task::none(),
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
            (_, Message::RedirectUser) => {
                //TODO: Check where the user should be redirected
                Task::done(Message::Navigate(Page::Loading(Loading::new())))
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
