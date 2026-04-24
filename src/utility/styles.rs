use ratatui::style::{Color, Style};

pub enum TextStyle {
    Title,
    Description,
    Link,
}

impl TextStyle {
    pub fn styles(self) -> Style {
        match self {
            Self::Title => Style::new().fg(Color::White).bold(),
            Self::Link => Style::new().fg(Color::Cyan).italic().underlined(),
            Self::Description => Style::new().fg(Color::White),
        }
    }
}
