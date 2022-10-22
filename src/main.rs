use std::fs::File;
use std::io::{self, BufReader};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{io::Result, thread};

use crossterm::cursor::{Hide, Show};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use dgg::chat::action::Action;
use dgg::chat::command::parse_command_to_action;
use dgg::chat::state::State;
use dgg::chat::user::UserList;
use dgg::config::Config;
use dgg::network::Network;

use dgg::ui::emotes::EmoteList;
use dgg::ui::render;
use dgg::ui::suggester::Suggestor;
use dgg::ui::window::WindowType;
use tokio::sync::Mutex;
use tui::{backend::CrosstermBackend, Terminal};
use tungstenite::Message;

#[tokio::main]
async fn main() -> Result<()> {
    let config = read_user_data_from_file()?;

    let (chat_msg_sender, chat_msg_recv) = futures::channel::mpsc::channel(1);
    let (io_sender, io_recv) = std::sync::mpsc::channel();
    let io_sender_2 = io_sender.clone();

    let state = Arc::new(Mutex::new(State::new(200, config.name, io_sender)));
    let cloned_state = Arc::clone(&state);

    // Network Thread
    std::thread::spawn(move || {
        let mut network = Network::new(&config.token, &state, chat_msg_sender);
        start_tokio(io_recv, io_sender_2, chat_msg_recv, &mut network);
    });

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, Hide, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let tick_rate = Duration::from_millis(100);
    let last_tick = Instant::now();

    let state = cloned_state.lock().await;
    let emote_list = EmoteList::new();
    let mut suggestor = Suggestor::new(&emote_list);

    // TODO make destiny.gg/api/chat/me work
    // state.dispatch(Action::GetMe);
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
                // match keys with modifiers
                match key {
                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => state.chat_input_history.delete_current_word(),
                    _ => (),
                }

                // match single keys
                match key.code {
                    KeyCode::Esc => {
                        state.dispatch(Action::QuitApp);
                        break;
                    }
                    KeyCode::Char(c) => {
                        state.chat_input_history.current_message.push(c);
                        suggestor.update(&state.ul, state.chat_input_history.get_current_word());
                        state.dispatch(Action::Err(suggestor.suggestions.join(",")));
                    }
                    KeyCode::Backspace => {
                        state.chat_input_history.current_message.pop();
                        suggestor.update(&state.ul, state.chat_input_history.get_current_word());
                    }
                    KeyCode::Tab => {
                        // Autocomplete: delete the current word and add the suggestion
                        if suggestor.suggestions.len() > 0 {
                            state.chat_input_history.delete_current_word();
                            state
                                .chat_input_history
                                .current_message
                                .push_str(suggestor.get())
                        }
                    }
                    KeyCode::Enter => {
                        if state.chat_input_history.current_message.starts_with("/") {
                            match parse_command_to_action(&state.chat_input_history.current_message)
                            {
                                Ok(action) => state.dispatch(action),
                                Err(err) => state.add_error(err.to_string()),
                            }
                            state.chat_input_history.add();
                        } else {
                            state.dispatch(Action::SendMsg);
                        }
                    }
                    KeyCode::Up => {
                        state.chat_input_history.next();
                    }
                    KeyCode::Down => {
                        state.chat_input_history.prev();
                    }
                    KeyCode::F(1) => state.windows.get_mut(WindowType::Debug).flip(),
                    KeyCode::F(2) => state.windows.get_mut(WindowType::UserList).flip(),
                    KeyCode::PageUp => state.dispatch(Action::ScrollUp),

                    KeyCode::PageDown => state.dispatch(Action::ScrollDown),

                    _ => (),
                }
            }
        }
        // should this be tokio sleep?
        thread::sleep(Duration::from_millis(30)); // run at roughly 30 fps
    }

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

fn read_user_data_from_file() -> Result<Config> {
    let file = File::open("config.json")?;
    let reader = BufReader::new(file);

    let config: Config = serde_json::from_reader(reader)?;

    Ok(config)
}
