use iced::{widget::container, Border};

pub fn card(theme: &iced::Theme) -> container::Style {
    let palette = theme.palette();
    let bg = palette.background;
    let border = palette.primary;

    container::Style {
        background: Some(iced::Background::Color(bg)),
        border: Border {
            color: border,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}
