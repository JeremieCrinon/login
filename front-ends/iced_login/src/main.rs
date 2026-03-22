mod pages;
mod translator;

use iced::{
    Element, Task, widget::{
        text, column, button, container, Container, row
    }
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
        match message {
            Message::Navigate(page) => {
                self.page = page;
            }
            Message::Login(msg) => {
                if let Page::Login(login) = &mut self.page {
                    login.update(msg);
                }
            }
            Message::Test(msg) => {
                if let Page::Test(test) = &mut self.page {
                    test.update(msg);
                }
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let test_button: Container<Message> = container(
            button("Test")
                .on_press(Message::Navigate(Page::Test(Test::new().0)))
        );

        let login_button: Container<Message> = container(
            button("Login")
                .on_press(Message::Navigate(Page::Login(Login::new().0)))
        );

        let buttons = row![
            test_button,
            login_button
        ];

        let page = match &self.page {
            Page::Login(login) => login.view().map(Message::Login),
            Page::Test(test) => test.view().map(Message::Test),
        };

        column![
            buttons,
            page
        ].into()
    }
}

fn main() -> iced::Result {
    iced::application(UI::new, UI::update, UI::view)
        .title("Iced login")
        .run()
}
