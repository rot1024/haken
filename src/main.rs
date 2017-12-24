extern crate haken;
#[macro_use]
extern crate clap;

mod model;
mod args;
mod param;

fn main() {
    let param = args::parse();
}
