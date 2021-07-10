extern crate clap;
mod validation;
use anyhow::Result;
use clap::{load_yaml, App, Arg};
use std::path::PathBuf;

fn main() -> Result<()> {
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
    let mut filename = PathBuf::from(matches.value_of("FILEPATH").unwrap());
    validation::resolve_filename(&mut filename)?; //  directly mutates and returns the resolved path
    println!("{}", filename.to_str().unwrap());
    Ok(())
}
