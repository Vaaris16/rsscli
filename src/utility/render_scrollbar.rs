use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::Color,
    widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState},
};

pub fn render_vertical_scrollbar(frame: &mut Frame, area: Rect, vertical: &mut ScrollbarState) {
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        vertical,
    );
}

pub fn render_horizontal_scrollbar(frame: &mut Frame, area: Rect, horizontal: &mut ScrollbarState) {
    let scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
        .symbols(ratatui::symbols::scrollbar::Set {
            track: "-",
            thumb: "▮",
            begin: "<",
            end: ">",
        })
        .track_style(Color::Yellow)
        .begin_style(Color::Green)
        .end_style(Color::Red);

    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        }),
        horizontal,
    );
}
