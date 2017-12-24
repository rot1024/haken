#[macro_use]
extern crate clap;
extern crate console;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate select;
extern crate tokio_core;

mod model;
mod source;
mod args;
mod param;
mod cli;

fn main() {
    let param = args::parse();
    let d = source::get_source(param.source.parse().unwrap()).unwrap();
    let f = cli::formatter::get_formatter(param.formatter.parse().unwrap()).unwrap();

    match d.get_anime_list(&param.cource) {
        Ok(l) => {
            let s = cli::selector::show(&l);
            let result = f.format(&s);
            println!("{}", result);
        }
        Err(e) => {
            println!("Failed to fetch data: {}", e.description());
        }
    }
}
