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
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use tui_textarea::TextArea;

use crate::chat::{
    features::Feature,
    state::{State, WindowList, WindowType},
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
    let debug_active = state.windows.get(WindowType::Debug).active;
    let userlist_active = state.windows.get(WindowType::UserList).active;
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

    text_area.set_block(
        Block::default()
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .borders(Borders::ALL)
            .title("Send"),
    );
    f.render_widget(text_area.widget(), chunks[1]);

    Ok(())
}

fn render_debug<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &MutexGuard<State>) {
    let (height, start) = get_height_and_start(chunk, state.debugs.len());

    let mut items: Vec<ListItem> = state.debugs[start..]
        .iter()
        .map(|msg| {
            let lines = textwrap::wrap(&msg, (chunk.width - 2) as usize);
            let line = Spans::from(Span::styled(msg, Style::default().fg(Color::White)));

            if lines.len() > 1 {
                let mut spans = Vec::with_capacity(lines.len());
                let mut extra_lines: Vec<ListItem> = lines
                    .iter()
                    .skip(1)
                    .map(|l| {
                        ListItem::new(Span::styled(
                            format!("{}", l),
                            Style::default().fg(Color::White),
                        ))
                    })
                    .collect();

                spans.push(ListItem::new(line));
                spans.append(&mut extra_lines);
                spans
            } else {
                vec![ListItem::new(line)]
            }
        })
        .flatten()
        .collect();

    // Scroll to bottom
    scroll_to_bottom(&mut items, height);

    let debug_messages = List::new(items).block(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .title("Debug"),
    );
    f.render_widget(debug_messages, chunk);
}

fn render_users<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &MutexGuard<State>) {
    let (height, start) = get_height_and_start(chunk, state.ul.users.len());
    let mut items: Vec<ListItem> = state.ul.users[start..]
        .iter()
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

    scroll_to_bottom(&mut items, height);

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
    fn render_chat_line<'a>(
        name: &str,
        pm: &str,
        timestamp: &str,
        bg_color: Color,
        name_style: &Style,
        message_style: &Style,
    ) -> Spans<'a> {
        Spans::from(vec![
            Span::styled(format!("[{}] ", timestamp), Style::default()),
            Span::styled(
                format!("{}", name),
                name_style.bg(bg_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                ": ",
                Style::default()
                    .bg(bg_color)
                    .remove_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("{}", pm), message_style.bg(bg_color)),
        ])
    }
    // this is the absolute max of messages we can render
    //  we need to update this later because of line wraps!
    let (height, start) = get_height_and_start(chunk, state.messages.len());

    let mut items: Vec<ListItem> = state.messages[start..] // only render messages in view
        .iter()
        .map(|m| {
            let name = m.name.to_string();
            let message_style = Style::default().fg(Color::White);
            let ts = m.get_timestamp_str();

            // Replace Emote Strings in Message
            let pm = parse_emotes(m.message.to_string(), emotes);
            // let pm = m.message.to_string();

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

            // Handle Line Wraps
            let full_line = format!("[{}] {}: {}", ts, name, pm);
            let lines = textwrap::wrap(&full_line, (chunk.width - 2) as usize);
            let first_line_length = lines[0].len() - 8 - name.len() - 2; // "[11:11] name: ";
            let line = render_chat_line(
                &name,
                &pm[..first_line_length],
                &ts,
                bg_color,
                &name_style,
                &message_style,
            );

            if lines.len() > 1 {
                let mut spans = Vec::with_capacity(lines.len());
                let mut extra_lines: Vec<ListItem> = lines
                    .iter()
                    .skip(1)
                    .map(|l| {
                        ListItem::new(Span::styled(format!("{}", l), message_style.bg(bg_color)))
                    })
                    .collect();

                spans.push(ListItem::new(line));
                spans.append(&mut extra_lines);

                spans
            } else {
                vec![ListItem::new(line)]
            }
        })
        .flatten()
        .collect();

    // Scroll to bottom
    scroll_to_bottom(&mut items, height);

    let chat_messages = List::new(items).block(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .title("DGG-Chat"),
    );
    f.render_widget(chat_messages, chunk);
}

// Deals with splitting the chunks into the right size for the different windows
fn get_chunks(size: &Rect, windows: &WindowList) -> Vec<Rect> {
    fn get_chat_chunks(chunk: Rect) -> (Rect, Rect) {
        let windows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(3), Constraint::Length(3)])
            .split(chunk);
        (windows[0], windows[1])
    }
    fn get_userlist_chunks(chunk: Rect) -> (Rect, Rect) {
        let windows = Layout::default()
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .direction(Direction::Horizontal)
            .split(chunk);
        (windows[0], windows[1])
    }
    fn get_debug_chunks(chunk: Rect) -> (Rect, Rect) {
        let windows = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .direction(Direction::Vertical)
            .split(chunk);
        (windows[0], windows[1])
    }

    let area = *size;
    let debug_active = windows.get(WindowType::Debug).active;
    let userlist_active = windows.get(WindowType::UserList).active;

    // chat, userlist and debug
    if debug_active == true && userlist_active == true {
        let (debug, rest_window) = get_debug_chunks(area);
        let (rest_window, user_list) = get_userlist_chunks(rest_window);
        let (chat, chat_input) = get_chat_chunks(rest_window);
        return vec![chat, chat_input, debug, user_list];
    // only chat and debug
    } else if debug_active {
        let (debug, rest_window) = get_debug_chunks(area);
        let (chat, chat_input) = get_chat_chunks(rest_window);
        return vec![chat, chat_input, debug];
    // only chat and userlist
    } else if userlist_active {
        let (rest_window, user_list) = get_userlist_chunks(area);
        let (chat, chat_input) = get_chat_chunks(rest_window);
        return vec![chat, chat_input, user_list];
    }
    // only chat
    let (chat, chat_input) = get_chat_chunks(area);
    return vec![chat, chat_input];
}

fn scroll_to_bottom(items: &mut Vec<ListItem>, height: usize) {
    if items.len() > height {
        let diff = items.len() - height + 2;
        items.drain(0..diff);
    }
}

fn get_height_and_start(chunk: Rect, list_len: usize) -> (usize, usize) {
    let height = (chunk.height) as usize;
    let start = if list_len > height {
        list_len - height - 2 // - 2 because of borders
    } else {
        0
    };

    (height, start)
}
