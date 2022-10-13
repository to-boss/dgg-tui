use std::panic;
use std::{io::stdout, io::Result, thread};

use crossterm::event::{KeyCode, KeyEvent};
use dgg::{
    chat::{dgg::DGG, event::Action, message::Message},
    ui,
};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    #[cfg(not(debug_assertions))]
    panic::set_hook(Box::new(|_| {
        println!("");
    }));

    let (mut dgg, dgg_sender) = DGG::new(99);
    let dgg_state = dgg.get_state_ref();
    dgg.debug_on();

    let _ = thread::spawn(move || dgg.work());

    ui::init()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        match terminal.draw(|f| ui::draw(f, &dgg_state).unwrap()) {
            Ok(_) => (),
            Err(_) => break,
        }

        if let Ok(val) = ui::get_keypresses() {
            if val {
                break;
            }
        }

        if let Ok(mut state) = dgg_state.try_lock() {
            while let Some(event) = state.pop_event() {
                match event.action {
                    Action::RecvMsg => {
                        let msg = Message::from_json(&event.body).unwrap();
                        state.add_message(msg);
                    }
                    Action::SendMsg => (),
                    Action::UserJoin => (),
                    Action::UserQuit => (),
                    Action::UsersInit => (),
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
    ui::close()?;
    Ok(())
}
