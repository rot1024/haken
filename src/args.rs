use std::str::FromStr;
use clap::{App, Arg};
use model::{Cource, Season};

pub fn parse() -> super::param::Param {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .name(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("YEAR")
                .required(true)
                .help("Sets the year of anime you want to select such as 2018")
                .index(1)
                .requires("SEASON")
                .validator(|y| match y.parse::<u32>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("not a valid numer".to_string()),
                }),
        )
        .get_matches();

    let year = matches.value_of("YEAR").unwrap();
    let season = matches.value_of("SEASON").unwrap();
    let formatter = matches.value_of("formatter").unwrap();
    let source = matches.value_of("source").unwrap();

    super::param::Param {
        cource: Cource {
            year: year.parse::<u32>().unwrap(),
            season: Season::from_str(season).unwrap(),
        },
        formatter: formatter.to_string(),
        source: source.to_string(),
    }
}
