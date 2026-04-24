use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    text::{Line, Span},
    widgets::ScrollbarState,
};

use crate::utility::render_scrollbar::render_vertical_scrollbar;

pub fn render(frame: &mut Frame, vertical: &mut ScrollbarState) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);

    let [top, main] = frame.area().layout(&layout);

    let title = Line::from_iter([
        Span::from("Scrollbar Widget").bold(),
        Span::from(" (Press 'q' to quit, arrow keys to scroll)"),
    ]);

    frame.render_widget(
        title.centered(),
        top.inner(ratatui::layout::Margin {
            horizontal: 3,
            vertical: 3,
        }),
    );

    render_vertical_scrollbar(frame, main, vertical);
}
