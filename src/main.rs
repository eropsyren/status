extern crate status;

use status::{DATA, Data, Entry};
use std::{thread, time};
use std::io::{Write, stdout};

fn main() {
    let raw = DATA.to_vec();

    loop {
        refresh(Data::get(&raw));
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn refresh(data: Data) {
    print!("{}[2J", 27 as char);
    print!("{}[H", 27 as char);
    println!("{}", data);
}
