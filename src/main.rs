use std::collections::VecDeque;
use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{io, panic};
use std::{io::stdout, io::Result, thread};

use crossterm::cursor::{Hide, Show};
use crossterm::event::KeyCode;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use dgg::chat::api::ApiCaller;
use dgg::chat::event::Action;
use dgg::chat::message::ChatMessage;
use dgg::chat::state::State;
use dgg::chat::user::{User, UserList};
use dgg::network::Network;
use dgg::ui::emotes::EmoteList;
use dgg::ui::render::{self, close, draw, init};
use dgg::ui::suggester::Suggestor;
use dgg::ui::window::WindowType;
use dgg::{chat, network};
use tokio::sync::Mutex;
use tui::{backend::CrosstermBackend, Terminal};
use tungstenite::Message;

#[tokio::main]
async fn main() -> Result<()> {
    // custom_panic();

    // let (mut dgg, sender) = DGG::new(200);
    // let dgg_state = dgg.get_state_ref();
    // let _ = thread::spawn(move || dgg.work());

    let (chat_msg_sender, chat_msg_recv) = futures::channel::mpsc::channel(1);
    let (io_sender, io_recv) = std::sync::mpsc::channel();
    let io_sender_2 = io_sender.clone();

    let state = Arc::new(Mutex::new(State::new(
        200,
        "onlyclose".to_string(),
        io_sender,
    )));
    let cloned_state = Arc::clone(&state);

    std::thread::spawn(move || {
        let mut network = Network::new(&state, chat_msg_sender);
        start_tokio(io_recv, io_sender_2, chat_msg_recv, &mut network);
    });

    // init()?;
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, Hide, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let emote_list = EmoteList::new();
    // let mut ui_events: VecDeque<Event> = VecDeque::new();
    // ui_events.push_back(Event::new(Action::GetChatHistory, String::new()));

    // let mut suggestor = Suggestor::new(&emote_list);

    let tick_rate = Duration::from_millis(100);
    let last_tick = Instant::now();

    let state = cloned_state.lock().await;
    state.dispatch(Action::GetChatHistory);
    drop(state);

    loop {
        let mut state = cloned_state.lock().await;
        match terminal.draw(|f| render::draw(f, &state, &emote_list).unwrap()) {
            Ok(_) => (),
            Err(_) => break,
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        state.dispatch(Action::QuitApp);
                        break;
                    }
                    KeyCode::Char(c) => state.chat_input.push(c),
                    KeyCode::Backspace => {
                        state.chat_input.pop();
                    }
                    KeyCode::Enter => {
                        if state.chat_input.starts_with("/") {
                            let whitespace = state.chat_input.find(" ").unwrap();
                            let command = &state.chat_input[1..whitespace];
                            if command == "stalk" {
                                state.dispatch(Action::Stalk("Destiny".to_string(), 20));
                            }
                        } else {
                            state.dispatch(Action::SendMsg);
                        }
                    }
                    KeyCode::F(1) => state.windows.get_mut(WindowType::Debug).flip(),
                    KeyCode::F(2) => state.windows.get_mut(WindowType::UserList).flip(),
                    _ => (),
                }
            }
        }

        // Handle Input
        // if let Ok(true) = event::poll(Duration::default()) {
        //     if let event::Event::Key(key) = event::read()? {
        //         match key.code {
        //             KeyCode::Esc => ui_events.push_back(Event::new(Action::QuitApp, String::new())),
        //             KeyCode::F(1) => {
        //                 ui_events.push_back(Event::new(Action::ChangeUserList, String::new()))
        //             }
        //             KeyCode::F(2) => {
        //                 ui_events.push_back(Event::new(Action::ChangeDebug, String::new()))
        //             }
        //             KeyCode::F(3) => {
        //                 ui_events.push_back(Event::new(Action::GetEmbeds, String::new()))
        //             }
        //             KeyCode::F(4) => ui_events.push_back(Event::new(
        //                 Action::Stalk(String::from("Destiny")),
        //                 String::new(),
        //             )),
        //             KeyCode::PageUp => {
        //                 ui_events.push_back(Event::new(Action::ScrollUp, String::new()))
        //             }
        //             KeyCode::PageDown => {
        //                 ui_events.push_back(Event::new(Action::ScrollDown, String::new()))
        //             }
        //             KeyCode::Enter => {
        //                 ui_events.push_back(Event::new(Action::SendMsg, String::new()))
        //             }
        //             KeyCode::Tab => {
        //                 // text_area.insert_str(suggestor.consume());
        //                 ()
        //             }
        //             KeyCode::Backspace => (),

        //             KeyCode::Char(' ') => {
        //                 suggestor.clear_word();
        //                 ui_events.push_back(Event::new(Action::Key(' '), String::new()));
        //             }
        //             KeyCode::Char(c) => {
        //                 suggestor.push(c);
        //                 ui_events.push_back(Event::new(Action::Key(c), String::new()));
        //             }
        //             _ => (),
        //         }
        //     }
        // }

        // Handle Events
        // if let Ok(mut state) = dgg_state.try_lock() {
        //     state.push_ui_events(&mut ui_events);

        //     while let Some(event) = state.pop_event() {
        //         let scroll = state.windows.get(WindowType::Chat).scroll;
        //         //state.add_debug(format!("{}: {}", event, scroll));
        //         // state.add_debug(format!("{:?}", suggestor.suggestions));
        //         match event.action {
        //             Action::SendMsg => {
        //                 let msg: String = state.chat_input.drain(..).collect();
        //                 let message_to_send = format!(r#"MSG {{"data":"{}"}}"#, msg);
        //                 sender.send(message_to_send).unwrap();
        //             }
        //             Action::QuitApp => break 'main,
        //             Action::Key(c) => state.chat_input.push(c),
        //             Action::RecvMsg => {
        //                 let msg = Message::from_json(&event.body).unwrap();
        //                 state.add_message(msg);
        //             }
        //             Action::UserJoin => state.ul.add(User::from_json(&event.body).unwrap()),
        //             Action::UserQuit => state.ul.remove(User::from_json(&event.body).unwrap()),
        //             Action::ScrollUp => state.windows.get_mut(WindowType::Chat).scroll(-1),
        //             Action::ScrollDown => state.windows.get_mut(WindowType::Chat).scroll(1),
        //             Action::GetChatHistory => {
        //                 let messages = api_caller.get_chat_history().unwrap();
        //                 messages
        //                     .iter()
        //                     .for_each(|msg| DGG::parse_ws_message(msg, &mut state))
        //             }
        //             Action::GetEmbeds => {
        //                 let embeds = api_caller.get_last_embeds().unwrap();
        //                 state.add_debug(embeds[0].to_string());
        //                 state.add_debug(embeds[1].to_string());
        //                 state.add_debug(embeds[2].to_string());
        //                 state.add_debug(embeds[3].to_string());
        //                 state.add_debug(embeds[4].to_string());
        //             }
        //             Action::Stalk(name) => {
        //                 let messages = api_caller.stalk(name).unwrap();
        //                 state.add_debug(messages[0].to_string());
        //                 state.add_debug(messages[1].to_string());
        //                 state.add_debug(messages[2].to_string());
        //                 state.add_debug(messages[3].to_string());
        //                 state.add_debug(messages[4].to_string());
        //             }
        //             Action::UsersInit => {
        //                 let mut ul = UserList::from_json(&event.body).unwrap();
        //                 let msg = Message::from(
        //                     "CONNECTED".to_string(),
        //                     format!(
        //                         "There a currently {} connections {} and users online.",
        //                         ul.conn_count,
        //                         ul.users.len()
        //                     ),
        //                 );
        //                 state.add_message(msg);
        //                 state.ul.append(&mut ul);
        //             }
        //             Action::Mute => (),
        //             Action::Unmute => (),
        //             Action::Ban => (),
        //             Action::Unban => (),
        //             Action::Subonly => (),
        //             Action::Broadcast => (),
        //             Action::PrivMsg => (),
        //             Action::Ping => (),
        //             Action::Pong => (),
        //             Action::Err => (),
        //             Action::Refresh => (),
        //             Action::Binary => (),
        //             Action::ChangeUserList => state.windows.get_mut(WindowType::UserList).flip(),
        //             Action::ChangeDebug => state.windows.get_mut(WindowType::Debug).flip(),
        //         }
        //     }
        // };/

        // thread::sleep(Duration::from_millis(30)); // run at roughly 30 fps
    }

    // close()?;
    let mut stdout = io::stdout();
    execute!(stdout, Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

#[tokio::main]
async fn start_tokio(
    io_recv: Receiver<Action>,
    io_sender: Sender<Action>,
    chat_msg_recv: futures::channel::mpsc::Receiver<Message>,
    network: &mut Network,
) {
    network.start_websocket(io_sender, chat_msg_recv).await;
    while let Ok(action) = io_recv.recv() {
        network.handle_io(action).await;
    }
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
