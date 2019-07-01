extern crate status;

use status::{get_data, Reading};
use std::{thread, time};

fn main() {
    let data = get_data();

    loop {
        clear();

        for datum in &data {
            println!("{}", Reading::from(datum));
        }

        thread::sleep(time::Duration::from_secs(1));
    }
}

fn clear() {
    print!("{}[2J", 27 as char);
    print!("{}[H", 27 as char);
}
