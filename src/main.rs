use std::{
    error::Error,
    io::{self},
    vec,
};

use clap::{Parser, Subcommand};
use ratatui::{
    Frame, Terminal,
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Line},
    style::{Color, Style},
    text::Span,
    widgets::Paragraph,
};
use rss::Channel;

#[derive(Parser)]
#[command(name = "rsscli")]
#[command(about = "rsscli")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get { url: String },
}

enum TextStyle {
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

async fn get(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;

    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

async fn draw_tui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    channel: Channel,
) -> Result<(), Box<dyn Error>> {
    terminal.draw(|frame: &mut Frame| {
        let area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Min(0)])
            .split(frame.area())[0];
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

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);
    })?;

    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Get { url } => {
            let channel = get(&url).await?;

            enable_raw_mode()?;
            execute!(io::stdout(), EnterAlternateScreen)?;

            let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
            draw_tui(&mut terminal, channel).await?;

            disable_raw_mode()?;
            execute!(io::stdout(), LeaveAlternateScreen)?;
        }
    }

    Ok(())
}
