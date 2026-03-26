mod pages;
mod translator;
mod config;

use std::collections::HashMap;
use iced::{
    Element, Task, keyboard, Subscription, Event, 
    widget::operation::{focus_next, focus_previous}
};

use pages::login::{Login, LoginMessage};
use pages::test::{Test, TestMessage};
use translator::translator::Translator;

use config::CONFIG;

pub struct AppState {
    pub translations: HashMap<String, String>,
    pub reqwest_client: reqwest::Client,
}

#[derive(Debug, Clone)]
pub enum Page {
    Login(Login),
    Test(Test),
}

pub struct UI {
    page: Page,
    state: AppState
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    Login(LoginMessage),
    Test(TestMessage),
    FocusNext,
    FocusPrevious,
}

impl UI {
    pub fn new() -> (Self, Task<Message>) {
        let translator = Translator::new();
        let available = translator.available_locales();
        let locale = sys_locale::get_locales()
            .find_map(|sys_locale| {
                let lang = sys_locale.split(&['-', '_'][..]).next().unwrap_or(&sys_locale);
                available.contains(&lang).then_some(lang.to_string())
            })
            .unwrap_or_else(|| "en".to_string());

        let translations = translator.get_translation(&locale);
        let client = reqwest::Client::new();

        let state = AppState {translations, reqwest_client: client};

        (
            UI {
                page: Page::Login(Login::new().0),
                state: state
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
            (Page::Login(page), Message::Login(msg)) => {
                page.update(msg, &self.state)
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
        match &self.page {
            Page::Login(login) => login.view(&self.state),
            Page::Test(test) => test.view(&self.state),
        }
    }

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
