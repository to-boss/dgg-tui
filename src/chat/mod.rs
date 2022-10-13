use std::io::{self, stdin, Write};

mod dgg;
mod features;
mod message;
mod state;
mod user;

pub fn run() {
    let mut dgg = dgg::DGG::new(100);
    dgg.debug_on();
    dgg.listen();

    let state = dgg.get_state_ref();

    // Network Thread
    //let network_handle = std::thread::spawn(move || dgg.listen());

    // IO Loop
    let stdin = stdin();
    let mut buffer = String::new();
    // loop {
    //     stdin.read_line(&mut buffer).unwrap();
    //     let res = match buffer.trim_end() {
    //         "q" => break,
    //         _ => "",
    //     };
    //     println!("> command: [{}]", res);
    // }
}
