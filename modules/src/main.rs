mod sound;

fn hello() {
    println!("HELLO");
}

use crate::sound::{instrument};

fn main() {
    println!("Hello, world!");
    instrument::clarinet();
    sound::instrument::clarinet();
}
