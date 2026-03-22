use iced::{
    Element, Task, widget::{
        Container, button, column, container, text
    }
};

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

    pub fn update(&mut self, message: TestMessage) -> Task<TestMessage> {
        match message {
            TestMessage::TestMsg { msg } => {
                println!("Message: {}", msg)
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, TestMessage> {
        let msg_button: Container<TestMessage> = container(
            button("Send msg")
                .on_press(TestMessage::TestMsg { msg: "Hello, World !".to_string() })   
        );

        column![
            text("Test"),
            msg_button
        ].into()
    }
}
