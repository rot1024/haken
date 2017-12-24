use super::super::model::{AnimeList, AnimeSelect};
use super::checkboxes::Checkboxes;

pub fn show(a: &AnimeList) -> AnimeSelect {
    let list: Vec<_> = a.list.iter().map(|b| format!("{}", b.title)).collect();
    let l = Checkboxes::new(list, true).interact().unwrap();
    AnimeSelect::select(a, &l)
}
