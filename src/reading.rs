use std::fmt;
use std::fs;

use crate::config::FNF;
use crate::config::SEPARATOR;
use crate::data::Data;

pub struct Reading {
    tag: &'static str,
    value: String,
}

impl Reading {
    pub fn from(data: &Data) -> Reading {
        let tag = data.get_tag();

        let reading = match data.get_path() {
            Some(path) => match fs::read_to_string(path) {
                Ok(val) => val,
                Err(_) => String::from(FNF),
            },
            None => String::from(""),
        };

        let value = if reading == FNF {
            reading
        } else if reading == "" {
            reading
        } else {
            data.exec_map(reading)
        };

        Reading { tag, value }
    }
}

impl fmt::Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.tag, SEPARATOR, self.value)
    }
}
