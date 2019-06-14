use std::fs;
use std::fmt;

pub struct Data {
    entries: Vec<Entry>,
}

impl Data {
    pub fn get(raw: &Vec<(&str, &str)>) -> Data {
        let mut entries: Vec<Entry> = vec![];

        for (name, path) in raw {
            entries.push(Entry::from(name, path));
        }

        Data {
            entries
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result: Vec<String> = self.entries.iter().map(|entry| format!("{}", entry)).collect();
        
        write!(f, "{}", result.join(""))
    }
}

pub struct Entry {
    name: String,
    val: Val,
}

impl Entry {
    fn from(name: &str, path: &str) -> Entry {
        let val = fs::read_to_string(path).expect(&format!("error reading: {}", path));
        
        Entry {
            name: name.to_string(),
            val: Val::from(val),
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.val)
    }
}

enum Val {
    n(f64),
    s(String),
}

impl Val {
    fn from(s: String) -> Val {
        let result= s.parse::<f64>();

        match result {
            Ok(num) => Val::n(num),
            _ => Val::s(s),
        }
    } 
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Val::n(n) => write!(f, "{}", n),
            Val::s(s) => write!(f, "{}", s),
        }
    }
}