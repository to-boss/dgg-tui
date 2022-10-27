use std::io::{self};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::cursor::{Hide, Show};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle};
use crossterm::{execute, terminal};
use dgg::chat::action::Action;
use dgg::chat::command::parse_command_to_action;
use dgg::chat::state::State;
use dgg::config::Config;
use dgg::network::Network;

use dgg::ui::emotes::EmoteList;
use dgg::ui::render;
use dgg::ui::suggester::Suggestor;
use dgg::ui::window::{WindowList, WindowType};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use tui::{backend::CrosstermBackend, Terminal};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO Read user input
    let mut config = Config::default();
    config.get_or_build_paths()?;
    config = config.read_user_data_from_file()?;
    config.askers()?;
    config.save_to_config_file()?;

    let emote_list = EmoteList::new();
    let mut suggestor = Suggestor::new(&emote_list);
    let mut windows = WindowList::new();

    let (chat_msg_sender, chat_msg_recv) = futures::channel::mpsc::channel(1);
    let (io_sender, io_recv) = std::sync::mpsc::channel();
    let io_sender_2 = io_sender.clone();

    let state = Arc::new(Mutex::new(State::new(config.name, io_sender)));
    let cloned_state = Arc::clone(&state);

    // Network Thread
    std::thread::spawn(move || {
        let mut network = Network::new(&config.token, &state, chat_msg_sender);
        start_tokio(io_recv, io_sender_2, chat_msg_recv, &mut network);
    });

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        Hide,
        EnterAlternateScreen,
        SetTitle("DGG - Terminally Online")
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let tick_rate = Duration::from_millis(16);
    let last_tick = Instant::now();

    // TODO make destiny.gg/api/chat/me work
    // state.dispatch(Action::GetMe);
    let state = cloned_state.lock().await;
    state.dispatch(Action::GetChatHistory);
    drop(state);

    loop {
        let mut state = cloned_state.lock().await;
        match terminal
            .draw(|f| render::draw(f, &state, &emote_list, &suggestor, &mut windows).unwrap())
        {
            Ok(_) => (),
            Err(_) => break,
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                // match keys with modifiers
                match key {
                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }
                    | KeyEvent {
                        // Gnome Terminal uses Control+h as Backspace
                        code: KeyCode::Char('h'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        state.chat_input.delete_current_word();
                        suggestor.suggestions.clear();
                    }
                    // match keys without modifiers
                    _ => match key.code {
                        KeyCode::Esc => {
                            state.dispatch(Action::QuitApp);
                            break;
                        }
                        KeyCode::Char(c) => {
                            state.chat_input.current_message.push(c);
                            suggestor.update(&state.ul, state.chat_input.get_current_word());
                        }
                        KeyCode::Backspace => {
                            state.chat_input.current_message.pop();
                            suggestor.update(&state.ul, state.chat_input.get_current_word());
                        }
                        KeyCode::Tab => {
                            // Autocomplete: delete the current word and add the suggestion
                            if suggestor.suggestions.len() > 0 {
                                state.chat_input.delete_current_word();
                                state.chat_input.current_message.push_str(&suggestor.get())
                            }
                        }
                        KeyCode::Enter => {
                            if state.chat_input.current_message.starts_with("/") {
                                match parse_command_to_action(&state.chat_input.current_message) {
                                    Ok(action) => state.dispatch(action),
                                    Err(err) => state.add_error(err.to_string()),
                                }
                                state.chat_input.add();
                            } else if state.chat_input.current_message.starts_with(":q") {
                                break;
                            } else {
                                state.dispatch(Action::SendMsg);
                            }
                            suggestor.suggestions.clear();
                        }
                        KeyCode::Up => {
                            state.chat_input.next();
                        }
                        KeyCode::Down => {
                            state.chat_input.prev();
                        }
                        KeyCode::F(1) => windows.get_mut(WindowType::Debug).flip(),
                        KeyCode::F(2) => windows.get_mut(WindowType::UserList).flip(),
                        KeyCode::F(3) => windows.get_mut(WindowType::Chat).auto_scroll = true,
                        KeyCode::PageUp => {
                            windows.get_mut(WindowType::Chat).scroll(-2);
                        }
                        KeyCode::PageDown => {
                            windows.get_mut(WindowType::Chat).scroll(2);
                        }
                        _ => (),
                    },
                }
            }
        }
        // sleep for lower CPU usage
        thread::sleep(tick_rate);
    }

    let mut stdout = io::stdout();
    execute!(stdout, Show, LeaveAlternateScreen,)?;
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
