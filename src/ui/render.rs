use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::chat::{features::Feature, state::State};

use super::{
    emotes::EmoteList,
    parser::{parse_emotes, parse_flair},
    window::{WindowList, WindowType},
};

pub fn draw<B: Backend>(f: &mut Frame<B>, state: &State, emote_list: &EmoteList) -> Result<()> {
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
    render_chat(f, chunks[0], &state, &emote_list);
    render_chat_input(f, chunks[1], &state);

    Ok(())
}

fn render_chat_input<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &State) {
    let input = Paragraph::new(state.chat_input.as_ref())
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Send"));
    f.set_cursor(chunk.x + state.chat_input.len() as u16 + 1, chunk.y + 1);

    f.render_widget(input, chunk);
}

fn render_debug<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &State) {
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

    scroll_to_bottom(&mut items, height);

    let chatter_names = List::new(items).block(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .title(format!("{} Users", state.ul.users.len())),
    );

    f.render_widget(chatter_names, chunk);
}

fn render_chat<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &State, emote_list: &EmoteList) {
    fn render_chat_line<'a>(
        name: &str,
        pm: &str,
        timestamp: &str,
        bg_color: Color,
        message_color: Color,
        name_color: Color,
    ) -> Spans<'a> {
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
                Style::default().fg(message_color).bg(bg_color),
            ),
        ])
    }
    // this is the absolute max of messages we can render
    //  we need to update this later because of line wraps!
    let (height, start) = get_height_and_start(chunk, state.messages.len());

    let mut items: Vec<ListItem> = state.messages[start..] // only render messages in view
        .iter()
        .map(|m| {
            let name = m.name.to_string();
            let ts = m.get_timestamp_str();

            // Replace Emote Strings in Message
            let pm = parse_emotes(m.message.to_string(), emote_list);
            // let pm = m.message.to_string();

            // Handle Name
            let mut name_color = get_name_color_from_flair(&m.features);

            // Handle Greentext
            let mut message_color = Color::White;
            if pm.starts_with(">") {
                message_color = Color::Green;
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
            let lines = textwrap::wrap(&full_line, (chunk.width - 2) as usize);

            // TODO
            let first_line_length = lines[0].len() - 8 - name.len() - 2;

            let line = render_chat_line(
                &name,
                &pm[..first_line_length],
                &ts,
                bg_color,
                message_color,
                name_color,
            );

            if lines.len() > 1 {
                let mut spans = Vec::with_capacity(lines.len());
                let mut extra_lines: Vec<ListItem> = lines
                    .iter()
                    .skip(1)
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

#[cfg(test)]
mod tests {
    use std::io::stdout;

    use time::OffsetDateTime;
    use tui::{backend::CrosstermBackend, Terminal};

    use crate::chat::message::ChatMessage;

    use super::*;

    #[test]
    fn really_long_message_no_whitespace() {
        let msg = String::from("testsadfwqrekqweoriuwqerpoiwequropiqwuroipwquropiwqeuropwiqeruwoipqruoqpiwruqpwoiruopqwiuropiqwuropqiwurqowpiruqowpiru");
        let message = ChatMessage {
            message: msg,
            features: vec![],
            name: "COCK".to_string(),
            timestamp: OffsetDateTime::now_utc(),
        };
        let emote_list = EmoteList::new();
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend).unwrap();
        // let state = Arc::new(Mutex::new(State::new(10, "COCK".to_string())));
        // state.lock().unwrap().add_message(message);
        // let rect = Rect {
        //     x: 10,
        //     y: 10,
        //     width: 30,
        //     height: 10,
        // };
        // terminal
        //     .draw(|f| render_chat(f, rect, &state.lock().unwrap(), &emote_list))
        //     .unwrap();
    }
}
