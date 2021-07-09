extern crate clap;
use clap::{load_yaml, App, Arg};
use std::{fs, io};
// use std::{fs, io};
fn main() {
    let load_yaml = load_yaml!("../args.yaml");
    let matches = App::from(load_yaml)
        .arg(
            Arg::with_name("youtube-dl_args")
                .help("args passed to youtube-dl")
                .allow_hyphen_values(true)
                .last(true) //  didn't work in .yaml file
                .multiple(true)
                .use_delimiter(false),
        )
        .get_matches();
}
