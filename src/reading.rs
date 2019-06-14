use std::fmt;
use std::fs;

use crate::config::SEPARATOR;

pub struct Reading {
    tag: String,
    value: String,
}

impl Reading {
    pub fn from(tag: &str, path: &str, mapping: &dyn Fn(String) -> String) -> Reading {
        let tag = String::from(tag);
        let reading = fs::read_to_string(path).expect(&format!("error reading file: {}", path));
        let value = mapping(String::from(reading.trim()));

        Reading { tag, value }
    }
}

impl fmt::Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.tag, SEPARATOR, self.value)
    }
}