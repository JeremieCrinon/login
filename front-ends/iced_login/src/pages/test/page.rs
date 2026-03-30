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

    pub fn view<'a>(&'a self, state: &'a AppState) -> Element<'a, Message> {
        let email = &state.user_email;
        let roles = &state.user_roles;
        let token_from_memory = &state.token;

        let msg_button = button("Send msg")
                .on_press(TestMessage::TestMsg { msg: "Hello, World !".to_string() }.into());

        let token_keychain: Option<Text> = match &self.token {
            Some(t) => Some(text(format!("Token from keychain: {}", t.as_str()))),
            None => None
        };

        let token_memory: Option<Text> = match token_from_memory {
            Some(t) => Some(text(format!("Token from memory: {}", t.as_str()))),
            None => None
        };

        let email: Option<Text> = match email {
            Some(e) => Some(text(format!("Email: {}", e.as_str()))),
            None => None
        };

        let roles: Option<Text> = match roles {
            Some(r) => Some(text(format!("Roles: {}", r.join(", ")))),
            None => None
        };

        let logout_button = button("logout")
            .on_press(Message::Logout);

        column![
            text("Test"),
            msg_button,
            token_keychain,
            token_memory,
            email,
            roles,
            logout_button
        ].into()
    }
}

impl From<TestMessage> for Message {
    fn from(message: TestMessage) -> Self {
        Self::Test(message)
    }
}
