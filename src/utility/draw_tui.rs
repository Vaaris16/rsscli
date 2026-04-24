use std::{
    error::Error,
    io::{self},
    vec,
};

use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, KeyCode},
    layout::{self, Alignment, Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Line},
    text::Span,
    widgets::{Paragraph, ScrollbarState},
};

use rss::Channel;

use crate::utility::render::render;
use crate::utility::styles::TextStyle;

pub async fn draw_tui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    channel: Channel,
) -> Result<(), Box<dyn Error>> {
    let mut vertical = ScrollbarState::new(50);

    let mut scroll: u16 = 0;

    let lines: Vec<Line> = channel
        .items()
        .iter()
        .flat_map(|item| {
            vec![
                Line::from(Span::styled(
                    item.title().unwrap_or("no title"),
                    TextStyle::Title.styles(),
                )),
                Line::from(Span::styled(
                    item.description().unwrap_or("no description"),
                    TextStyle::Description.styles(),
                )),
                Line::from(Span::styled(
                    item.link().unwrap_or("no link"),
                    TextStyle::Link.styles(),
                )),
                Line::from(""),
            ]
        })
        .collect();

    loop {
        let lines = lines.clone();
        terminal.draw(|frame: &mut Frame| {
            let content_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(1), Constraint::Length(9)])
                .split(frame.area())[0];

            render(frame, &mut vertical);

            let paragraph = Paragraph::new(lines.clone())
                .alignment(Alignment::Center)
                .scroll((scroll, 0));
            frame.render_widget(
                paragraph,
                content_area.inner(layout::Margin {
                    horizontal: 2,
                    vertical: 2,
                }),
            );
        })?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('j') | KeyCode::Down => {
                    vertical.next();
                    scroll = scroll.saturating_add(1);
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    vertical.prev();
                    scroll = scroll.saturating_sub(1);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
