use iced::{
    Element, Task, widget::{
        text, column
    }
};

pub struct UI {}

#[derive(Debug, Clone)]
pub enum Message {}

impl UI {
    pub fn new() -> (Self, Task<Message>) {
        (
            UI {},
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![text("Hello, world !")].into()
    }
}

fn main() -> iced::Result {
    iced::application(UI::new, UI::update, UI::view)
        .title("Iced login")
        .run()
}
