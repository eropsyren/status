extern crate status;

use status::{DATA, Data, Entry};
use std::{thread, time};

fn main() {
    loop {
        let data = Data::get(DATA.to_vec());

        println!("{}", data);

        thread::sleep(time::Duration::from_secs(1));
    }
}