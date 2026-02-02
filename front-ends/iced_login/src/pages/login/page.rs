use iced::{
    Element, Task, widget::{
        Container, button, column, container, text
    }
};

pub struct Login {}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    TestMsg {
        msg: String
    }
}

impl Login {
    pub fn new() -> (Self, Task<LoginMessage>) {
        (
            Login {},
            Task::none(),
        )
    }

    pub fn update(&mut self, message: LoginMessage) -> Task<LoginMessage> {
        match message {
            LoginMessage::TestMsg { msg } => {
                println!("Message: {}", msg)
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, LoginMessage> {
        let msg_button: Container<LoginMessage> = container(
            button("Send msg")
                .on_press(LoginMessage::TestMsg { msg: "Hello, World !".to_string() })   
        );

        column![
            text("Login"),
            msg_button
        ].into()
    }
}
