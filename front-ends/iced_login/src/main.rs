mod pages;

use iced::{
    Element, Task, widget::{
        text, column
    }
};
use pages::login::{Login, LoginMessage};

pub struct UI {
    login: Login
}

#[derive(Debug, Clone)]
pub enum Message {
    Login(LoginMessage),
}

impl UI {
    pub fn new() -> (Self, Task<Message>) {
        (
            UI {
                login: Login::new().0
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Login(msg) => {
                let _ = self.login.update(msg);
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("Hello, world !"),
            self.login.view().map(Message::Login),
        ].into()
    }
}

fn main() -> iced::Result {
    iced::application(UI::new, UI::update, UI::view)
        .title("Iced login")
        .run()
}
