use iced::{
    Element, Task, widget::{
        button, column, text, Text
    }
};
use crate::{AppState, Message};
use crate::helpers::get_token_from_keychain;

#[derive(Debug, Clone)]
pub struct Test {
    token: Option<String>
}

#[derive(Debug, Clone)]
pub enum TestMessage {
    TestMsg {
        msg: String
    }
}

impl Test {
    pub fn new() -> (Self, Task<TestMessage>) {
        let token = get_token_from_keychain();

        (
            Test {
                token: token
            },
            Task::none(),
        )
    }

    pub(crate) fn update(&mut self, message: TestMessage) -> Task<Message> {
        match message {
            TestMessage::TestMsg { msg } => {
                println!("Message: {}", msg)
            }
        }
        Task::none()
    }

    pub fn view<'a>(&'a self, _state: &'a AppState) -> Element<'a, Message> {
        let msg_button = button("Send msg")
                .on_press(TestMessage::TestMsg { msg: "Hello, World !".to_string() }.into());

        let token_text: Option<Text> = match &self.token {
            Some(t) => Some(text(t.as_str())),
            None => None
        };

        let logout_button = button("logout")
            .on_press(Message::Logout);

        column![
            text("Test"),
            msg_button,
            token_text,
            logout_button
        ].into()
    }
}

impl From<TestMessage> for Message {
    fn from(message: TestMessage) -> Self {
        Self::Test(message)
    }
}
