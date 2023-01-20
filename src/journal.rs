use crate::entry::{entries, Entry};
use std::io::Read;

#[derive(Debug)]
pub struct Journal {
    entries: Vec<Entry>,
    name: String,
}

impl Journal {
    fn sort(&mut self) {
        self.entries.sort_by_key(|entry| entry.time)
    }

    pub fn from_file<R: Read>(name: &str, reader: &mut R) -> Self {
        let name = name.to_owned();
        let mut raw = String::new();
        reader.read_to_string(&mut raw).expect("read failed");
        let entries = entries(raw.lines()).collect::<Vec<_>>();

        Self { entries, name }
    }
}
