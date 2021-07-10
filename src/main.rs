extern crate clap;
use anyhow::{bail, Context, Result};
use clap::{load_yaml, App, Arg};
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

macro_rules! map_arr {
    ($func:path,$($e:expr),*) => {[$($func($e),)*]}
}

fn resolve_filename(filename: &mut PathBuf) -> Result<()> {
    let video_extensions = map_arr!(OsStr::new, "mp4", "flv", "mkv", "avi"); // video extensions supported
    let music_extensions = map_arr!(OsStr::new, "mp3", "aac", "m4a");
    let extension = filename.extension().context("invalid filename!")?;

    let dir_env_var = if video_extensions.contains(&extension) {
        "VIDEOS"
    } else if music_extensions.contains(&extension) {
        "MUSIC"
    } else {
        bail!("unsupported format {:?}", extension);
    };

    let mut parent_dir: PathBuf;
    match filename.parent() {
        Some(var) => parent_dir = PathBuf::from(var),
        None => return Ok(()),
    }
    if parent_dir == Path::new("") {
        parent_dir = match std::env::var_os(dir_env_var) {
            Some(media_dir) => PathBuf::from(media_dir),
            None => std::env::current_dir()?,
        };
    }

    // creating all intermediate parent directories of filename
    create_dir_all(&parent_dir).context("could not create parent directories!")?;
    *filename = parent_dir
        .canonicalize()? // resolves all intermediate symlinks and .. or . dirs
        .join(filename.file_name().unwrap());
    Ok(())
}

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
    resolve_filename(&mut filename)?; //  directly mutates and returns the resolved path
    println!("{}", filename.to_str().unwrap());
    Ok(())
}
