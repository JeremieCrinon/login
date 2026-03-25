mod pages;

use iced::{
    Element, Task
};
use pages::login::{Login, LoginMessage};
use pages::test::{Test, TestMessage};

#[derive(Debug, Clone)]
pub enum Page {
    Login(Login),
    Test(Test),
}

pub struct UI {
    page: Page,
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
            Page::Login(login) => login.view(),
            Page::Test(test) => test.view(),
        }

    }
}

fn main() -> iced::Result {
    iced::application(UI::new, UI::update, UI::view)
        .title("Iced login")
        .run()
}
