use std::panic;
use std::time::Duration;
use std::{io::stdout, io::Result, thread};

use crossterm::event::{self, KeyCode};
use dgg::chat::user::{User, UserList};
use dgg::chat::{dgg::DGG, event::Action, message::Message};
use dgg::ui::emotes::EmoteList;
use dgg::ui::render::{close, draw, get_key, init};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders};
use tui::{backend::CrosstermBackend, Terminal};
use tui_textarea::{Input, Key, TextArea};

fn main() -> Result<()> {
    custom_panic();

    let (mut dgg, dgg_sender) = DGG::new(99);
    let dgg_state = dgg.get_state_ref();
    dgg.debug_on();

    let _ = thread::spawn(move || dgg.work());

    init()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    let emotes = EmoteList::new();

    let mut text_area = TextArea::default();
    text_area.set_style(Style::default().bg(Color::Black));
    text_area.set_cursor_line_style(Style::default());
    text_area.set_block(Block::default().borders(Borders::ALL).title("Send"));

    loop {
        match terminal.draw(|f| draw(f, &dgg_state, &emotes, &mut text_area).unwrap()) {
            Ok(_) => (),
            Err(_) => break,
        }

        // if let Ok(val) = get_key() {
        //     match val {
        //         KeyCode::Esc => break,
        //         KeyCode::F(1) => dgg_sender.send(0).unwrap(),
        //         KeyCode::Enter => dgg_sender.send(1).unwrap(),
        //         _ => (),
        //     }
        // }

        if let Ok(bool) = event::poll(Duration::default()) {
            if bool {
                match crossterm::event::read()?.into() {
                    Input { key: Key::Esc, .. } => break,
                    Input { key: Key::F(1), .. } => dgg_sender.send(0).unwrap(),
                    Input {
                        key: Key::Enter, ..
                    } => {
                        dgg_sender.send(1).unwrap();
                    }
                    input => {
                        text_area.input(input);
                    }
                }
            }
        }

        if let Ok(mut state) = dgg_state.try_lock() {
            while let Some(event) = state.pop_event() {
                match event.action {
                    Action::RecvMsg => {
                        let msg = Message::from_json(&event.body).unwrap();
                        state.add_message(msg);
                    }
                    Action::SendMsg => {
                        state.message_to_send = event.body.clone();
                        state.send_message = true;
                    }
                    Action::UserJoin => state.ul.add(User::from_json(&event.body).unwrap()),
                    Action::UserQuit => state.ul.remove(User::from_json(&event.body).unwrap()),
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
                }
            }
        }
    }

    // let _ = dgg_handle.join();
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
