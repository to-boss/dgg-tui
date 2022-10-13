use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self, ErrorKind, Result},
    sync::{Arc, Mutex},
    time::Duration,
};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::chat::state::State;

pub fn init() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, Hide, EnterAlternateScreen)?;
    Ok(())
}

pub fn close() -> Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

pub fn get_key() -> crossterm::Result<KeyCode> {
    if let Ok(bool) = event::poll(Duration::default()) {
        if bool {
            if let event::Event::Key(key) = event::read().unwrap() {
                match key {
                    _ => return Ok(key.code),
                }
            }
        }
    }
    Ok(KeyCode::Null)
}

pub fn draw<B>(f: &mut Frame<B>, app: &Arc<Mutex<State>>) -> Result<()>
where
    B: tui::backend::Backend,
{
    let app = app.lock().unwrap();

    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .margin(1)
        .split(size);

    let items: Vec<ListItem> = app
        .inc_messages
        .iter()
        .map(|pm| {
            let name = pm.name.to_string();
            let message = pm.message.to_string();

            // Handle Name
            let name_style = Style::default().fg(Color::White);

            // Handle Greentext
            let message_style = if message.starts_with(">") {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };

            let line = Spans::from(vec![
                Span::styled(format!("{}", name), name_style),
                Span::raw(": "),
                Span::styled(format!("{} ", message), message_style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let chat_messages = List::new(items).block(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .title("DGG-Chat"),
    );

    let input_window = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL)
        .title(format!("{:?}", chunks[0].width));

    f.render_widget(chat_messages, chunks[0]);
    f.render_widget(input_window, chunks[1]);
    Ok(())
}
