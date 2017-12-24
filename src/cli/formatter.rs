use super::super::model::AnimeSelect;
use std::str::FromStr;

pub trait Formatter {
    fn format(&self, &AnimeSelect) -> String;
}

pub struct SimpleFormatter {}

impl SimpleFormatter {
    pub fn new() -> Box<Formatter> {
        Box::new(SimpleFormatter {})
    }
}

impl Formatter for SimpleFormatter {
    fn format(&self, a: &AnimeSelect) -> String {
        a.selection
            .iter()
            .map(|a| &a.title)
            .fold("".to_string(), |a, b| a + "\n" + b)
    }
}

pub enum FormatterType {
    Simple,
}

impl FromStr for FormatterType {
    type Err = ();

    fn from_str(s: &str) -> Result<FormatterType, ()> {
        match s {
            "simple" => Ok(FormatterType::Simple),
            _ => Err(()),
        }
    }
}

pub fn get_formatter(t: FormatterType) -> Option<Box<Formatter>> {
    match t {
        FormatterType::Simple => Some(SimpleFormatter::new()),
        // _ => None,
    }
}
