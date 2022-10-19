use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{io, panic};
use std::{io::stdout, io::Result, thread};

use crossterm::cursor::{Hide, Show};
use crossterm::event::KeyCode;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use dgg::chat::event::Action;
use dgg::chat::state::State;
use dgg::network::Network;
use dgg::ui::emotes::EmoteList;
use dgg::ui::render::{self, close};
use dgg::ui::suggester::Suggestor;
use dgg::ui::window::WindowType;
use tokio::sync::Mutex;
use tui::{backend::CrosstermBackend, Terminal};
use tungstenite::Message;

#[tokio::main]
async fn main() -> Result<()> {
    // custom_panic();

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

        //             Action::UserJoin => state.ul.add(User::from_json(&event.body).unwrap()),
        //             Action::UserQuit => state.ul.remove(User::from_json(&event.body).unwrap()),
        //             Action::ScrollUp => state.windows.get_mut(WindowType::Chat).scroll(-1),
        //             Action::ScrollDown => state.windows.get_mut(WindowType::Chat).scroll(1),
        //
        //             Action::GetEmbeds => {
        //                 let embeds = api_caller.get_last_embeds().unwrap();
        //                 state.add_debug(embeds[0].to_string());
        //                 state.add_debug(embeds[1].to_string());
        //                 state.add_debug(embeds[2].to_string());
        //                 state.add_debug(embeds[3].to_string());
        //                 state.add_debug(embeds[4].to_string());

        thread::sleep(Duration::from_millis(30)); // run at roughly 30 fps
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
