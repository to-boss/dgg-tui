use std::ops::Range;

use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::chat::{features::Feature, message::ChatMessage, state::State};

use super::{
    emotes::EmoteList,
    parser::{parse_emotes, parse_flair},
    suggester::Suggestor,
    window::{WindowList, WindowType},
};

pub fn draw<B: Backend>(
    f: &mut Frame<B>,
    state: &State,
    emote_list: &EmoteList,
    suggestions: &Suggestor,
    windows: &mut WindowList,
) -> Result<()> {
    let debug_active = windows.get(WindowType::Debug).active;
    let userlist_active = windows.get(WindowType::UserList).active;
    let size = f.size();
    let chunks = get_chunks(&size, windows);

    if debug_active == true && userlist_active == true {
        render_debug(f, chunks[2], &state, windows);
        render_users(f, chunks[3], &state);
    } else if debug_active {
        render_debug(f, chunks[2], &state, windows);
    } else if userlist_active {
        render_users(f, chunks[2], &state);
    }

    // Always render chat and chat_input
    render_chat(f, chunks[0], &state, &emote_list, windows)?;
    render_chat_input(f, chunks[1], &state, &suggestions);

    Ok(())
}

fn render_chat_input<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    state: &State,
    suggestions: &Suggestor,
) {
    let title = format!("Send:─{}", suggestions);

    let start_range;
    let mut cursor_x = chunk.x + state.chat_input_history.current_message.len() as u16 + 1;
    if state.chat_input_history.current_message.len() < (chunk.width - 2).into() {
        start_range = 0;
    } else {
        start_range = state.chat_input_history.current_message.len() - ((chunk.width - 2) as usize);
        if start_range > (chunk.width - 2).into() {
            cursor_x -= 1;
        }
    }

    let input = Paragraph::new(&state.chat_input_history.current_message[start_range..])
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title(title));

    f.set_cursor(cursor_x, chunk.y + 1);
    f.render_widget(input, chunk);
}

fn render_chat<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    state: &State,
    emote_list: &EmoteList,
    windows: &mut WindowList,
) -> Result<()> {
    // this is the absolute max of messages we can render
    //  we need to update this later because of line wraps!
    let height = (chunk.height - 2) as usize;
    let width = (chunk.width - 2) as usize;
    let viewport = windows
        .get_mut(WindowType::Chat)
        .compute_viewport(height, state.messages.len());

    let items: Vec<ListItem> = get_chat_items(
        viewport,
        width,
        &state.username,
        &state.messages,
        &emote_list,
    );

    let chat_messages = List::new(items).block(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .title("DGG-Chat"),
    );
    f.render_widget(chat_messages, chunk);

    Ok(())
}

fn render_debug<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    state: &State,
    windows: &mut WindowList,
) {
    let height = (chunk.height - 2) as usize;
    let viewport = windows
        .get_mut(WindowType::Debug)
        .compute_viewport(height, state.debugs.len());

    let items: Vec<ListItem> = state.debugs[viewport]
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

    let debug_messages = List::new(items).block(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .title("Debug"),
    );
    f.render_widget(debug_messages, chunk);
}

fn render_users<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &State) {
    let (height, start) = get_height_and_start(chunk, state.ul.users.len());
    let mut items: Vec<ListItem> = state.ul.users[start..]
        .iter()
        .map(|m| {
            let name = m.name.to_string();

            // Handle Name
            let name_style = get_name_color_from_flair(&m.features);

            let line = Spans::from(vec![Span::styled(
                format!("{}", name),
                Style::default().fg(name_style),
            )]);
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

fn get_height_and_start(chunk: Rect, list_len: usize) -> (usize, usize) {
    let height = (chunk.height) as usize;
    let start = if list_len > height + 2 {
        list_len - height - 2 // - 2 because of borders
    } else {
        0
    };

    (height, start)
}

fn get_name_color_from_flair(features: &Vec<String>) -> Color {
    let pf = parse_flair(features).unwrap();
    match pf {
        Feature::Tier1 => Color::Cyan,
        Feature::Tier2 => Color::LightCyan,
        Feature::Tier3 => Color::LightGreen,
        Feature::Tier4 => Color::Magenta,
        Feature::Vip => Color::Rgb(230, 144, 20),
        Feature::Mod => Color::Yellow,
        Feature::Broadcaster => Color::Rgb(230, 144, 20),
        Feature::Admin => Color::Red,
        _ => Color::White,
    }
}

// Convert a Vec<ChatMessage> to a Vec<ListItem> with proper styling
fn get_chat_items<'a>(
    viewport: Range<usize>,
    width: usize,
    username: &'a str,
    messages: &Vec<ChatMessage>,
    emote_list: &EmoteList,
) -> Vec<ListItem<'a>> {
    fn render_chat_line<'a>(
        name: &str,
        pm: &str,
        timestamp: &str,
        bg_color: Color,
        message_color: Color,
        name_color: Color,
        modifier: Modifier,
    ) -> Spans<'a> {
        // Each line has 4 components which can be styled differently
        // [time] [name ][: ] [rest of message]
        Spans::from(vec![
            Span::styled(format!("[{}] ", timestamp), Style::default()),
            Span::styled(
                format!("{}", name),
                Style::default()
                    .fg(name_color)
                    .bg(bg_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                ": ",
                Style::default()
                    .bg(bg_color)
                    .remove_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{}", pm),
                Style::default()
                    .fg(message_color)
                    .bg(bg_color)
                    .add_modifier(modifier),
            ),
        ])
    }

    messages[viewport] // only render messages in view
        .iter()
        .map(|m| {
            let name = &m.name;
            let ts = m.get_timestamp_str();
            let mut words: Vec<&str> = m.message.split_whitespace().collect();

            // Replace Emote Strings in Message
            let pm = parse_emotes(&mut words, emote_list);

            // Default styles
            let mut message_color = Color::White;
            let mut bg_color = Color::Black;
            let mut modifier = Modifier::empty();

            // Handle Name
            let mut name_color = get_name_color_from_flair(&m.features);

            // Handle Greentext
            if pm.starts_with(">") {
                message_color = Color::Green;
            }

            // Handle Name Hightlight own Message
            if name == username {
                bg_color = Color::Rgb(50, 50, 50);
            }

            // Handle Highlight other Message
            if pm.contains(username) {
                bg_color = Color::Rgb(10, 40, 60);
            }

            // Handle nsfw messages
            // TODO: only mark nsfw when a link is found
            if words
                .iter()
                .any(|word| word.len() == 4 && word.to_lowercase() == "nsfw")
            {
                modifier = Modifier::UNDERLINED;
                bg_color = Color::LightMagenta;
            }

            if name.eq("STALK") || name.eq("EMBED") {
                name_color = Color::Rgb(250, 0, 140);
                bg_color = Color::Rgb(50, 50, 50);
            }

            if name.eq("ERROR") {
                name_color = Color::LightRed;
                bg_color = Color::Rgb(50, 50, 50);
            }

            // Handle Line Wraps
            let full_line = format!("[{}] {}: {}", ts, name, pm);
            let lines = textwrap::wrap(&full_line, width);

            // Text wrapping of really long messages
            // "[12:00] : ".len() is 10, i think the wrapper strips the whitespace after the :
            // to the next line when dealing with really long messages
            let first_line_length;
            if lines[0].len() - name.len() < 10 {
                first_line_length = lines[0].len() - name.len() - 9;
            } else {
                first_line_length = lines[0].len() - name.len() - 10;
            }

            let line = render_chat_line(
                &name,
                &pm[..first_line_length],
                &ts,
                bg_color,
                message_color,
                name_color,
                modifier,
            );

            if lines.len() > 1 {
                let mut spans = Vec::with_capacity(lines.len());
                let mut extra_lines: Vec<ListItem> = lines
                    .iter()
                    .skip(1) // Skip the first line, it's already handled
                    .map(|l| {
                        ListItem::new(Span::styled(
                            format!("{}", l),
                            Style::default().fg(message_color).bg(bg_color),
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
        .collect()
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use super::*;

    #[test]
    fn really_long_message_no_whitespace() {
        let (sender, _) = channel();
        let state = State::new(100, "onlyclose".to_string(), sender);
        let emote_list = EmoteList::new();
        let messages = vec![ChatMessage::from_string(
            state.username.to_string(),
            "x".repeat(100),
        )];
        let _ = get_chat_items(0..20, 20, &state.username, &messages, &emote_list);
        // println!("{:#?}", _);
    }
}
