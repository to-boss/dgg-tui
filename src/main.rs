use std::collections::VecDeque;
use std::panic;
use std::thread::current;
use std::time::Duration;
use std::{io::stdout, io::Result, thread};

use crossterm::event::{self, KeyCode, MouseEventKind};
use dgg::chat::api::ApiCaller;
use dgg::chat::event::Event;
use dgg::chat::state::WindowType;
use dgg::chat::user::{User, UserList};
use dgg::chat::{dgg::DGG, event::Action, message::Message};
use dgg::ui::emotes::EmoteList;
use dgg::ui::render::{close, draw, init};
use dgg::ui::suggester::Suggestor;
use tui::style::Style;
use tui::{backend::CrosstermBackend, Terminal};
use tui_textarea::TextArea;

fn main() -> Result<()> {
    custom_panic();

    let (mut dgg, sender) = DGG::new(200);
    let dgg_state = dgg.get_state_ref();

    let _ = thread::spawn(move || dgg.work());

    init()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    let api_caller = ApiCaller::new();
    let emote_list = EmoteList::new();

    let mut text_area = TextArea::default();
    text_area.set_cursor_line_style(Style::default());

    let mut ui_events: VecDeque<Event> = VecDeque::new();
    ui_events.push_back(Event::new(Action::GetChatHistory, String::new()));

    let mut suggestor = Suggestor::new(&emote_list);

    'main: loop {
        match terminal.draw(|f| draw(f, &dgg_state, &emote_list, &mut text_area).unwrap()) {
            Ok(_) => (),
            Err(_) => break,
        }

        // Handle Input
        if let Ok(true) = event::poll(Duration::default()) {
            match event::read()? {
                event::Event::Key(key_event) => {
                    let res = match key_event.code {
                        KeyCode::Esc => {
                            ui_events.push_back(Event::new(Action::QuitApp, String::new()))
                        }
                        KeyCode::F(1) => {
                            ui_events.push_back(Event::new(Action::ChangeUserList, String::new()))
                        }
                        KeyCode::F(2) => {
                            ui_events.push_back(Event::new(Action::ChangeDebug, String::new()))
                        }
                        KeyCode::F(3) => {
                            ui_events.push_back(Event::new(Action::GetEmbeds, String::new()))
                        }
                        KeyCode::F(4) => ui_events.push_back(Event::new(
                            Action::Stalk(String::from("Destiny")),
                            String::new(),
                        )),
                        KeyCode::PageUp => {
                            ui_events.push_back(Event::new(Action::ScrollUp, String::new()))
                        }
                        KeyCode::PageDown => {
                            ui_events.push_back(Event::new(Action::ScrollDown, String::new()))
                        }
                        KeyCode::Enter => {
                            let msg = text_area.lines()[0].to_string();
                            let message_to_send = format!(r#"MSG {{"data":"{}"}}"#, msg);
                            sender.send(message_to_send).unwrap();
                            text_area.delete_line_by_head();
                        }
                        KeyCode::Tab => {
                            text_area.insert_str(suggestor.consume());
                            ()
                        }
                        KeyCode::BackTab => suggestor.pop(),

                        _ => (),
                    };
                    // If we dont use any of these KeyCodes, send them to the text_area
                    if res == () {
                        text_area.input_without_shortcuts(key_event);
                        match key_event.code {
                            KeyCode::Char(' ') => suggestor.clear_word(),
                            KeyCode::Char(c) => suggestor.push(c),
                            _ => (),
                        }
                    }
                }
                event::Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::ScrollUp => {
                        ui_events.push_back(Event::new(Action::ScrollUp, String::new()))
                    }
                    MouseEventKind::ScrollDown => {
                        ui_events.push_back(Event::new(Action::ScrollDown, String::new()))
                    }
                    _ => (),
                },

                _ => (),
            }
        }

        // Handle Events
        if let Ok(mut state) = dgg_state.try_lock() {
            state.push_ui_events(&mut ui_events);

            while let Some(event) = state.pop_event() {
                let scroll = state.windows.get(WindowType::Chat).scroll;
                //state.add_debug(format!("{}: {}", event, scroll));
                // state.add_debug(format!("{:?}", suggestor.suggestions));
                match event.action {
                    Action::QuitApp => break 'main,
                    Action::RecvMsg => {
                        let msg = Message::from_json(&event.body).unwrap();
                        state.add_message(msg);
                    }
                    Action::UserJoin => state.ul.add(User::from_json(&event.body).unwrap()),
                    Action::UserQuit => state.ul.remove(User::from_json(&event.body).unwrap()),
                    Action::ScrollUp => state.windows.get_mut(WindowType::Chat).scroll(-1),
                    Action::ScrollDown => state.windows.get_mut(WindowType::Chat).scroll(1),
                    Action::GetChatHistory => {
                        let messages = api_caller.get_chat_history().unwrap();
                        messages
                            .iter()
                            .for_each(|msg| DGG::parse_ws_message(msg, &mut state))
                    }
                    Action::GetEmbeds => {
                        let embeds = api_caller.get_last_embeds().unwrap();
                        state.add_debug(embeds[0].to_string());
                        state.add_debug(embeds[1].to_string());
                        state.add_debug(embeds[2].to_string());
                        state.add_debug(embeds[3].to_string());
                        state.add_debug(embeds[4].to_string());
                    }
                    Action::Stalk(name) => {
                        let messages = api_caller.stalk(name).unwrap();
                        state.add_debug(messages[0].to_string());
                        state.add_debug(messages[1].to_string());
                        state.add_debug(messages[2].to_string());
                        state.add_debug(messages[3].to_string());
                        state.add_debug(messages[4].to_string());
                    }
                    Action::UsersInit => {
                        let mut ul = UserList::from_json(&event.body).unwrap();
                        let msg = Message::from(
                            "CONNECTED".to_string(),
                            format!(
                                "There a currently {} connections {} and users online.",
                                ul.conn_count,
                                ul.users.len()
                            ),
                        );
                        state.add_message(msg);
                        state.ul.append(&mut ul);
                    }
                    Action::Mute => (),
                    Action::Unmute => (),
                    Action::Ban => (),
                    Action::Unban => (),
                    Action::Subonly => (),
                    Action::Broadcast => (),
                    Action::PrivMsg => (),
                    Action::Ping => (),
                    Action::Pong => (),
                    Action::Err => (),
                    Action::Refresh => (),
                    Action::Binary => (),
                    Action::ChangeUserList => state.windows.get_mut(WindowType::UserList).flip(),
                    Action::ChangeDebug => state.windows.get_mut(WindowType::Debug).flip(),
                }
            }
        };

        thread::sleep(Duration::from_millis(30)); // run at roughly 30 fps
    }

    close()?;
    Ok(())
}

fn custom_panic() {
    #[cfg(not(debug_assertions))]
    panic::set_hook(Box::new(|_| {
        println!("");
    }));

    #[cfg(debug_assertions)]
    panic::set_hook(Box::new(|panic_info| {
        let _ = close();
        println!("{}", panic_info);
    }))
}
