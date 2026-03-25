mod pages;
mod translator;

use std::collections::HashMap;

use iced::{
    Element, Task
};
use pages::login::{Login, LoginMessage};
use pages::test::{Test, TestMessage};
use translator::translator::Translator;

#[derive(Debug, Clone)]
pub enum Page {
    Login(Login),
    Test(Test),
}

pub struct UI {
    page: Page,
    translations: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    Login(LoginMessage),
    Test(TestMessage)
}

impl UI {
    pub fn new() -> (Self, Task<Message>) {
        (
            UI {
                page: Page::Login(Login::new().0),
                translations: Translator::new().get_translation("en"),
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
            (Page::Login(page), Message::Login(msg)) => {
                page.update(msg)
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
            Page::Login(login) => login.view(&self.translations),
            Page::Test(test) => test.view(&self.translations),
        }
    }
}

fn main() -> iced::Result {
    iced::application(UI::new, UI::update, UI::view)
        .title("Iced login")
        .run()
}
