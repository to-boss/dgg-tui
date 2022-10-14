use crossterm::{
    cursor::{Hide, Show},
    event::{self, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Result},
    sync::{Arc, Mutex, MutexGuard},
    time::Duration,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use tui_textarea::TextArea;

use crate::chat::{features::Feature, state::State};

use super::{
    emotes::EmoteList,
    parser::{parse_emotes, parse_flair},
};

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

pub fn draw<B: Backend>(
    f: &mut Frame<B>,
    state: &Arc<Mutex<State>>,
    emotes: &EmoteList,
    text_area: &mut TextArea,
) -> Result<()> {
    let mut state = state.lock().unwrap();

    let size = f.size();
    let chunks = get_chunks(&size, &state.users_window);

    let max_items = (chunks[0].height - 2) as usize;
    state.max_messages = max_items;

    render_chat(f, chunks[0], &state, &emotes);
    f.render_widget(text_area.widget(), chunks[1]);

    if state.users_window {
        render_users(f, chunks[2], &state);
    }

    Ok(())
}

fn render_users<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &MutexGuard<State>) {
    let max_items = (chunk.height - 2) as usize;
    let items: Vec<ListItem> = state
        .ul
        .users
        .iter()
        .take(max_items)
        .map(|m| {
            let name = m.name.to_string();
            // Handle Name
            let pf = parse_flair(&m.features);
            let name_style = match pf {
                Feature::Tier1 => Style::default().fg(Color::Cyan),
                Feature::Tier2 => Style::default().fg(Color::LightCyan),
                Feature::Tier3 => Style::default().fg(Color::Green),
                Feature::Tier4 => Style::default().fg(Color::Magenta),
                Feature::Vip => Style::default().fg(Color::Rgb(231, 144, 21)),
                Feature::Mod => Style::default().fg(Color::Yellow),
                Feature::Broadcaster => Style::default().fg(Color::Rgb(231, 144, 21)),
                Feature::Admin => Style::default().fg(Color::Red),
                _ => Style::default().fg(Color::White),
            };

            let line = Spans::from(vec![Span::styled(format!("{}", name), name_style)]);
            ListItem::new(line)
        })
        .collect();

    let chatter_names = List::new(items).block(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .title(format!("{} Users", state.ul.users.len())),
    );
    f.render_widget(chatter_names, chunk);
}

fn render_chat<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    state: &MutexGuard<State>,
    emotes: &EmoteList,
) {
    let max_items = (chunk.height - 2) as usize;
    let items: Vec<ListItem> = state
        .messages
        .iter()
        .take(max_items)
        .map(|m| {
            let name = m.name.to_string();
            let message_style = Style::default().fg(Color::White);

            // Replace Emote Strings in Message
            let pm = parse_emotes(m.message.to_string(), emotes);

            // Handle Name
            let pf = parse_flair(&m.features);
            let name_style = match pf {
                Feature::Tier1 => Style::default().fg(Color::Cyan),
                Feature::Tier2 => Style::default().fg(Color::LightCyan),
                Feature::Tier3 => Style::default().fg(Color::Green),
                Feature::Tier4 => Style::default().fg(Color::Magenta),
                Feature::Vip => Style::default().fg(Color::Rgb(231, 144, 21)),
                Feature::Mod => Style::default().fg(Color::Yellow),
                Feature::Broadcaster => Style::default().fg(Color::Rgb(231, 144, 21)),
                Feature::Admin => Style::default().fg(Color::Red),
                _ => Style::default().fg(Color::White),
            };

            // Handle Greentext
            if pm.starts_with(">") {
                message_style.fg(Color::Green);
            }

            // Handle Name Hightlight own Message
            let mut background = Style::default().bg(Color::Black);
            if name == state.username {
                background = Style::default().bg(Color::Rgb(21, 21, 21));
            }

            // Handle Highlight other Message
            if pm.contains(&state.username) {
                background = Style::default().bg(Color::Rgb(6, 38, 62));
            }

            let line = Spans::from(vec![
                Span::styled(format!("{}", name), name_style),
                Span::raw(": "),
                Span::styled(format!("{} ", pm), message_style),
            ]);

            ListItem::new(line).style(background)
        })
        .collect();

    let chat_messages = List::new(items).block(
        Block::default()
            .style(Style::default().bg(Color::Rgb(8, 8, 8)))
            .borders(Borders::ALL)
            .title("DGG-Chat"),
    );

    f.render_widget(chat_messages, chunk);
}

fn get_chunks(size: &Rect, users_window: &bool) -> Vec<Rect> {
    if *users_window == false {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
            .split(*size)
    } else {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
            .split(*size);
        let left = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
            .split(chunks[0]);
        vec![left[0], left[1], chunks[1]]
    }
}
