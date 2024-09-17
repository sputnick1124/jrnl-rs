use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use std::iter::Peekable;

use crate::error::{JrnlError, JrnlErrorKind};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Entries<I>
where
    I: Iterator,
{
    iter: Peekable<I>,
}

impl<'a, I> Iterator for Entries<I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry_lines: Vec<&str> = vec![];
        match self.iter.next() {
            None => None,
            Some(elt) => {
                entry_lines.push(elt);
                loop {
                    match self.iter.peek() {
                        Some(&elt) if !elt.starts_with("[") => {
                            entry_lines.push(self.iter.next().unwrap())
                        }
                        _ => break,
                    }
                }
                Entry::parse(&entry_lines).ok()
            }
        }
    }
}

pub fn entries<I>(iter: I) -> Entries<I>
where
    I: Iterator,
{
    Entries {
        iter: iter.peekable(),
    }
}

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub time: NaiveDateTime,
    title: String,
    text: String,
    tags: Vec<String>,
    starred: bool,
}

impl Entry {
    pub fn parse(raw_text: &[&str]) -> Result<Self> {
        lazy_static! {
            static ref TITLE_RE: Regex =
                Regex::new(r"^[[:blank:]]*\[(?P<time>[^\]]+)\]\s*(?P<title>.*$)").unwrap();
        }
        let time_title = raw_text
            .get(0)
            .ok_or(JrnlError(JrnlErrorKind::EmptyEntry))?;
        let caps = TITLE_RE
            .captures(time_title)
            .ok_or(JrnlError(JrnlErrorKind::InvalidTitleLine))?;
        let time_str = &caps["time"].to_owned();
        let title = caps["title"].to_owned();
        let starred = title.contains("*");
        let text = raw_text
            .iter()
            .skip(1)
            .map(|&s| format!("{}\n", s))
            .collect::<String>()
            .trim()
            .to_owned();
        let tags = text
            .split_whitespace()
            // TODO: get tag chars from config
            .filter(|word| word.starts_with(&['#', '@']))
            .map(|word| word.to_owned())
            .collect::<Vec<String>>();
        // TODO: get strftime fmt from config
        let time = NaiveDateTime::parse_from_str(time_str, "%F %r")?.into();
        Ok(Entry {
            time,
            title,
            text,
            tags,
            starred,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn test_parse() {
        let lines = [
            "[2023-01-12 08:51:57 AM] Test entry.",
            "This is a test entry",
        ];
        let assert_date = NaiveDate::from_ymd_opt(2023, 01, 12).unwrap();
        let assert_time = NaiveTime::from_hms_opt(8, 51, 57).unwrap();
        assert_eq!(
            Entry {
                time: NaiveDateTime::new(assert_date, assert_time),
                title: "Test entry.".to_owned(),
                text: "This is a test entry".to_owned(),
                tags: vec![],
                starred: false,
            },
            Entry::parse(&lines).unwrap()
        );
    }
}
