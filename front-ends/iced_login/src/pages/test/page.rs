use iced::{
    Element, Task, widget::{
        button, column, text
    }
};
use crate::Message;

#[derive(Debug, Clone)]
pub struct Test {}

#[derive(Debug, Clone)]
pub enum TestMessage {
    TestMsg {
        msg: String
    }
}

impl Test {
    pub fn new() -> (Self, Task<TestMessage>) {
        (
            Test {},
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

    pub fn view(&self) -> Element<'_, Message> {
        let msg_button = button("Send msg")
                .on_press(TestMessage::TestMsg { msg: "Hello, World !".to_string() }.into());

        column![
            text("Test"),
            msg_button
        ].into()
    }
}

impl From<TestMessage> for Message {
    fn from(message: TestMessage) -> Self {
        Self::Test(message)
    }
}
