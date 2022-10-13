use std::{io::stdin, thread};

use dgg::chat::dgg::DGG;

fn main() {
    let (mut dgg, dgg_sender) = DGG::new(99);
    dgg.debug_on();

    thread::spawn(move || dgg.work());
    println!("Write a message: ");

    loop {
        let mut buff = String::new();
        stdin()
            .read_line(&mut buff)
            .expect("Reading from stdin failed!");

        let msg = buff.trim().to_string();

        if msg == ":q" || dgg_sender.send(msg).is_err() {
            break;
        }
    }
}
