extern crate clap;
use super::path_utils::MediaFile;
use super::url_utils::resolve_url;
use anyhow::Result;
use clap::{load_yaml, App, Arg, ArgMatches, Values};
use getset::Getters;
use std::ffi::OsStr;
use std::path::Path;

#[derive(Getters)]
pub struct Args {
    #[get = "pub"]
    is_verbose: bool,
    #[get = "pub"]
    file: MediaFile,
    #[get = "pub"]
    url: String,
    #[get = "pub"]
    quality: Option<String>,
    #[get = "pub"]
    metadata_args: Option<MetadataArgs>,
    #[get = "pub"]
    youtube_dl_args: Option<Vec<String>>,
}

#[derive(Getters)]
pub struct MetadataArgs {
    #[get = "pub"]
    pub artist: Option<String>,
    #[get = "pub"]
    pub album: Option<String>,
}
impl MetadataArgs {
    pub fn from_matches(matches: &ArgMatches, filepath: &Path) -> Option<MetadataArgs> {
        if filepath.extension().unwrap() == OsStr::new("mp3") {
            return Some(MetadataArgs {
                artist: matches
                    .value_of("artist")
                    .and_then(|image_url| Some(image_url.to_owned())),
                album: matches
                    .value_of("album")
                    .and_then(|image_url| Some(image_url.to_owned())),
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

    // filepath from user
    let filepath = Path::new(matches.value_of("FILEPATH").unwrap());
    // resolved and parent substituted filepath
    let file = MediaFile::new(&filepath)?;
    let mut url = String::from(matches.value_of("URL").unwrap());
    if let Some(resolved_url) = resolve_url(&url)? {
        url = resolved_url; // change url into "ytsearch:<keyword>"
    };

    let metadata_args = MetadataArgs::from_matches(&matches, &filepath);

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
        file,
        quality: matches.value_of("quality").and_then(|s| Some(s.to_owned())),
        url,
        metadata_args,
        is_verbose: matches.is_present("verbose"),
    });
}
