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

mod utility;

use utility::draw_tui::draw_tui;

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

async fn get(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;

    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
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
