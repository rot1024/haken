use std::error::Error;
use super::{AnimeList, Cource};

pub trait Source {
    fn get_anime_list(&self, &Cource) -> Result<AnimeList, Box<Error>>;
}
