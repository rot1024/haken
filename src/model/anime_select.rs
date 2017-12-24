use super::{Anime, AnimeList, Cource};

#[derive(Clone, Debug)]
pub struct AnimeSelect {
    pub cource: Cource,
    pub selection: Vec<Anime>,
}

impl AnimeSelect {
    pub fn select(a: &AnimeList, i: &[usize]) -> AnimeSelect {
        AnimeSelect {
            cource: a.cource.clone(),
            selection: a.list
                .iter()
                .enumerate()
                .filter_map(|(idx, x)| {
                    if i.contains(&idx) {
                        Some(x.clone())
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}
