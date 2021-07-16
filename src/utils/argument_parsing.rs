extern crate clap;
use super::validation;
use anyhow::Result;
use clap::{load_yaml, App, Arg, ArgMatches, Values};
use std::ffi::OsStr;
use std::path::PathBuf;

pub struct Args {
    pub is_verbose: bool,
    pub filepath: PathBuf,
    pub url: String,
    pub mp3_args: Option<Mp3Args>,
    pub youtube_dl_args: Option<Vec<String>>,
}

pub struct Mp3Args {
    pub image_url: Option<String>,
    pub keep_image: bool,
}
impl Mp3Args {
    pub fn from_matches(matches: &ArgMatches, filepath: &PathBuf) -> Option<Mp3Args> {
        if filepath.extension().unwrap() == OsStr::new("mp3") {
            return Some(Mp3Args {
                image_url: matches
                    .value_of("image-url")
                    .and_then(|image_url| Some(image_url.to_owned())),
                keep_image: matches.is_present("keep_image"),
            });
        }
        return None;
    }
}

pub fn get_resolved_arguments() -> Result<Args> {
    let yaml_args = load_yaml!("../../args.yaml");
    let matches = App::from(yaml_args)
        .arg(
            Arg::with_name("youtube-dl_args")
                .help("args passed to youtube-dl")
                .allow_hyphen_values(true)
                .last(true) //  didn't work in .yaml file
                .multiple(true)
                .use_delimiter(false),
        )
        .get_matches();

    let mut filepath = PathBuf::from(matches.value_of("FILEPATH").unwrap());
    validation::resolve_filepath(&mut filepath)?; //  directly mutates filepath to the resolved path
    println!("{}", filepath.to_str().unwrap());

    let mut url = String::from(matches.value_of("URL").unwrap());
    validation::resolve_url(&mut url)?;
    let mp3_args = Mp3Args::from_matches(&matches, &filepath);
    let youtube_dl_args = matches
        .values_of("youtube-dl_args")
        .and_then(|values: Values| {
            Some(
                values
                    .map(|val: &str| val.to_owned())
                    .collect::<Vec<String>>(),
            )
        });
    return Ok(Args {
        youtube_dl_args,
        filepath,
        url,
        mp3_args,
        is_verbose: matches.is_present("verbose"),
    });
}
