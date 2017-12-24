use std::error::Error;
use super::http;
use super::super::model::{Anime, AnimeList, Cource, Source};
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

pub struct Animemo {}

impl Animemo {
    pub fn new() -> Box<Source> {
        Box::new(Animemo {})
    }

    fn parse_html(c: &Cource, html: &str) -> Result<AnimeList, Box<Error>> {
        let doc = Document::from(html);
        let list = doc.find(Name("section").and(Attr("rel", ())))
            .filter_map(|n| {
                let title = n.find(Name("h4").child(Name("a"))).next();
                let url = title.and_then(|n| n.attr("href"));
                if title.and(url).is_some() {
                    Some(Anime {
                        title: title.unwrap().text(),
                        url: url.unwrap().to_string(),
                        rebroadcast: false,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(AnimeList {
            cource: c.clone(),
            list: list,
        })
    }
}

impl Source for Animemo {
    fn get_anime_list(&self, c: &Cource) -> Result<AnimeList, Box<Error>> {
        let (s, e) = c.season.get_month_range();
        match http::get(&format!(
            "http://animemo.jp/post/{}{:02}{:02}",
            c.year, s, e
        )) {
            Ok(res) => Self::parse_html(c, &res),
            Err(err) => Err(err),
        }
    }
}
