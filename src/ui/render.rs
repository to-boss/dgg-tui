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
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use tui_textarea::TextArea;

use crate::chat::{
    self,
    features::Feature,
    state::{State, Window},
    user,
};

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
    let state = state.lock().unwrap();
    let debug_active = state.windows[0].active;
    let userlist_active = state.windows[1].active;

    let size = f.size();
    let chunks = get_chunks(&size, &state.windows);

    if debug_active == true && userlist_active == true {
        render_debug(f, chunks[2], &state);
        render_users(f, chunks[3], &state);
    } else if debug_active {
        render_debug(f, chunks[2], &state);
    } else if userlist_active {
        render_users(f, chunks[2], &state);
    }

    // Always render chat and chat_input
    render_chat(f, chunks[0], &state, &emotes);
    f.render_widget(text_area.widget(), chunks[1]);

    Ok(())
}

fn render_debug<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &MutexGuard<State>) {
    let debug_block = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL)
        .title("Debugs");

    let max_items = (chunk.height - 2) as usize;
    let text: Vec<Spans> = state
        .debugs
        .iter()
        .take(max_items)
        .map(|line| Spans::from(Span::styled(line, Style::default().fg(Color::White))))
        .collect();

    let paragraph = Paragraph::new(text)
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .block(debug_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    f.render_widget(paragraph, chunk);
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
    let text: Vec<Spans> = state
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
                Feature::Vip => Style::default().fg(Color::Rgb(230, 144, 20)),
                Feature::Mod => Style::default().fg(Color::Yellow),
                Feature::Broadcaster => Style::default().fg(Color::Rgb(230, 144, 20)),
                Feature::Admin => Style::default().fg(Color::Red),
                _ => Style::default().fg(Color::White),
            };

            // Handle Greentext
            if pm.starts_with(">") {
                message_style.fg(Color::Green);
            }

            // Handle Name Hightlight own Message
            let mut bg_color = Color::Black;
            if name == state.username {
                bg_color = Color::Rgb(50, 50, 50);
            }

            // Handle Highlight other Message
            if pm.contains(&state.username) {
                bg_color = Color::Rgb(10, 40, 60);
            }

            Spans::from(vec![
                Span::styled(format!("{}", name), name_style.bg(bg_color)),
                Span::styled(": ", Style::default().bg(bg_color)),
                Span::styled(format!("{} ", pm), message_style.bg(bg_color)),
            ])
        })
        .collect();

    let chat_block = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL)
        .title("DGG-Chat");

    let chat_messages = Paragraph::new(text)
        .style(Style::default().bg(Color::Black))
        .block(chat_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((0, 0));

    f.render_widget(chat_messages, chunk);
}

fn get_chunks(size: &Rect, windows: &Vec<Window>) -> Vec<Rect> {
    let area = *size;
    let debug_active = windows[0].active;
    let userlist_active = windows[1].active;

    let mut constraints = vec![Constraint::Percentage(100)];
    let mut chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);

    let mut chat = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(chunks[0]);

    if debug_active == true && userlist_active == true {
        constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
        chunks = Layout::default()
            .constraints(constraints)
            .direction(Direction::Vertical)
            .split(area);
        let users = Layout::default()
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .direction(Direction::Horizontal)
            .split(chunks[1]);
        chat = Layout::default()
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
            .direction(Direction::Vertical)
            .split(users[0]);
        return vec![chat[0], chat[1], chunks[0], users[1]];
    } else if debug_active {
        constraints = vec![Constraint::Percentage(20), Constraint::Percentage(80)];
        chunks = Layout::default()
            .constraints(constraints)
            .direction(Direction::Vertical)
            .split(area);
        chat = Layout::default()
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
            .direction(Direction::Vertical)
            .split(chunks[1]);
        return vec![chat[0], chat[1], chunks[0]];
    } else if userlist_active {
        constraints = vec![Constraint::Percentage(80), Constraint::Percentage(20)];
        chunks = Layout::default()
            .constraints(constraints)
            .direction(Direction::Horizontal)
            .split(area);

        chat = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(chunks[0]);
        return vec![chat[0], chat[1], chunks[1]];
    }

    return vec![chat[0], chat[1]];
}
