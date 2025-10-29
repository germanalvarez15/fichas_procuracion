use iced::widget::button;
use iced::widget::container;
use iced::{Border, Color, Shadow, Theme};

pub fn primary_button(theme: &Theme, status: button::Status) -> button::Style {
    let color = match status {
        button::Status::Active => Color::from_rgb(0.0, 0.482, 1.0),
        button::Status::Hovered => Color::from_rgb(0.0, 0.40, 0.85),
        button::Status::Pressed => Color::from_rgb(0.0, 0.35, 0.70),
        button::Status::Disabled => Color::from_rgb(0.5, 0.65, 0.8),
    };

    button::Style {
        background: Some(color.into()),
        border: Border {
            color: Color::from_rgb(0.196, 0.804, 0.196),
            width: 1.0,
            radius: 5.0.into(),
        },
        text_color: Color::from_rgb(1.0, 1.0, 1.0),
        ..Default::default()
    }
}

pub fn secondary_button(theme: &Theme, status: button::Status) -> button::Style {
    let color = match status {
        button::Status::Active => Color::from_rgb(0.17, 0.70, 0.17),
        button::Status::Hovered => Color::from_rgb(0.19, 0.75, 0.19),
        button::Status::Pressed => Color::from_rgb(0.14, 0.60, 0.14),
        button::Status::Disabled => Color::from_rgb(0.3, 0.45, 0.3),
    };

    button::Style {
        background: Some(color.into()),
        border: Border {
            color: Color::from_rgb(0.196, 0.804, 0.196),
            width: 1.0,
            radius: 5.0.into(),
        },
        text_color: Color::from_rgb(1.0, 1.0, 1.0),
        ..Default::default()
    }
}

pub fn cancel_button(theme: &Theme, status: button::Status) -> button::Style {
    let color = match status {
        button::Status::Active => Color::from_rgb(1.0, 0.0, 0.0),
        button::Status::Hovered => Color::from_rgb(0.9, 0.1, 0.1),
        button::Status::Pressed => Color::from_rgb(0.7, 0.0, 0.0),
        button::Status::Disabled => Color::from_rgb(0.6, 0.3, 0.3),
    };

    button::Style {
        background: Some(color.into()),
        border: Border {
            color: Color::from_rgb(1.0, 0.0, 0.0),
            width: 1.0,
            radius: 5.0.into(),
        },
        text_color: Color::from_rgb(1.0, 1.0, 1.0),
        ..Default::default()
    }
}

pub fn card_container(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Color::from_rgb(0.98, 0.98, 0.99).into()),
        border: Border {
            color: Color::from_rgb(0.85, 0.85, 0.87),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Shadow {
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 4.0,
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
        },
        ..Default::default()
    }
}
