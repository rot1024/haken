use super::{Anime, Cource};

#[derive(Clone, Debug)]
pub struct AnimeList {
    pub cource: Cource,
    pub list: Vec<Anime>,
}
