use iced::{
    Element, Task, widget::{
        button, column, text
    }
};

use crate::{Page, pages::test::Test, Message};

#[derive(Debug, Clone)]
pub struct Login {}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    TestMsg {
        msg: String
    },
}

impl Login {
    pub fn new() -> (Self, Task<LoginMessage>) {
        (
            Login {},
            Task::none(),
        )
    }

    pub(crate) fn update(&mut self, message: LoginMessage) -> Task<Message> {
        match message {
            LoginMessage::TestMsg { msg } => {
                println!("Message: {}", msg);
                Task::done(Message::Navigate(Page::Test(Test::new().0)))
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let msg_button = button("Send msg and navigate to test")
                .on_press(LoginMessage::TestMsg { msg: "Hello, World !".to_string() }.into());

        let switch_button = button("Switch to test page")
                .on_press(Message::Navigate(Page::Test(Test::new().0)));
        

        column![
            text("Login"),
            msg_button,
            switch_button
        ].into()
    }
}

impl From<LoginMessage> for Message {
    fn from(message: LoginMessage) -> Self {
        Self::Login(message)
    }
}
