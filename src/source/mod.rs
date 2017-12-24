mod http;
mod animemo;

use model::Source;
use std::str::FromStr;

pub enum SourceType {
    Animemo,
}

impl FromStr for SourceType {
    type Err = ();

    fn from_str(s: &str) -> Result<SourceType, ()> {
        match s {
            "animemo" => Ok(SourceType::Animemo),
            _ => Err(()),
        }
    }
}

pub fn get_source(t: SourceType) -> Option<Box<Source>> {
    match t {
        SourceType::Animemo => Some(animemo::Animemo::new()),
        // _ => None,
    }
}
