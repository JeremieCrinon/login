use iced::{
    Element, Task, widget::{
        button, column, text, Text
    }
};
use crate::{AppState, Message, config::CONFIG};

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
        let entry: Option<keyring::Entry> = match keyring::Entry::new(CONFIG.app_name.as_str(), "token") {
            Ok(e) => Some(e),
            Err(e) => {
                println!("Error getting the keyring entry: {}", e);
                None
            }
        };

        let token = match entry {
            Some(e) => {
                match e.get_password() {
                    Ok(t) => Some(t),
                    Err(e) => {
                        println!("Failed to get password from keyring entry: {}", e);
                        None
                    }
                }
            }
            None => None
        };

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

        column![
            text("Test"),
            msg_button,
            token_text
        ].into()
    }
}

impl From<TestMessage> for Message {
    fn from(message: TestMessage) -> Self {
        Self::Test(message)
    }
}
