use std::thread;

use dgg::chat::{dgg::DGG, event::Action, message::Message};

fn main() {
    let (mut dgg, dgg_sender) = DGG::new(99);
    let dgg_state = dgg.get_state_ref();
    dgg.debug_on();

    let dgg_handle = thread::spawn(move || dgg.work());

    loop {
        if let Ok(mut state) = dgg_state.try_lock() {
            while let Some(event) = state.pop_event() {
                match event.action {
                    Action::RecvMsg => {
                        let msg = Message::from_json(&event.body).unwrap();
                        println!("{}", msg);
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

    dgg_handle.join();
}
