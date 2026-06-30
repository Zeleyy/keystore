use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{layout::{Alignment, Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, Borders, Paragraph}};

pub struct App {}

impl App {
    fn new() -> Self {
        Self {}
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new(); 

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(3),
                    Constraint::Length(1),
                ])
                .split(frame.area());

            let main_block = Block::default()
                .title(" Keystore CLI ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));

            let welcome_text = Paragraph::new("Welcome to Keystore TUI!\n\nPress 'q' to exit.")
                .alignment(Alignment::Center)
                .block(main_block);

            let status_bar = Paragraph::new(" q: Quit ")
                .style(Style::default().bg(Color::DarkGray).fg(Color::White));

            frame.render_widget(welcome_text, chunks[0]);
            frame.render_widget(status_bar, chunks[1]);
        })?;

        if handle_input(&mut app)? {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}

fn handle_input(_app: &mut App) -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    return Ok(true);
                }
                
                if key.code == KeyCode::Char('q') {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}
