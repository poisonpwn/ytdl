use super::user_input;
use anyhow::{bail, Context, Result};
use colored::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

macro_rules! map_arr {
    //  macro for mapping each value in array to a diff type
    ($func:path,$($e:expr),*) => {[$($func($e),)*]}
}

fn get_media_env_var(extension: &OsStr) -> Result<&str> {
    let video_extensions = map_arr!(OsStr::new, "mp4", "flv", "mkv", "avi"); // video extensions supported
    let music_extensions = map_arr!(OsStr::new, "mp3", "aac", "m4a"); // audio extensions supported
    if video_extensions.contains(&extension) {
        // is supported video format
        Ok("VIDEOS")
    } else if music_extensions.contains(&extension) {
        // is supported audio format
        Ok("MUSIC")
    } else {
        // unsupported format!, bail with error message
        bail!("unsupported format {:?}", extension);
    }
}

pub fn resolve_filename(filename: &mut PathBuf) -> Result<()> {
    let mut parent_dir = match filename.parent() {
        Some(parent_dir) => PathBuf::from(parent_dir),

        // means that filename's parent is root or some prefix (i.e no need to resolve)
        None => return Ok(()),
    };

    if parent_dir == Path::new("") {
        // filename only given (resolve to media dir if possible)
        let extension = filename.extension().context("invalid filename!".red())?;

        // get parent dir from env var, depending on file extension
        parent_dir = match std::env::var_os(get_media_env_var(extension)?) {
            Some(media_dir) => PathBuf::from(media_dir),
            None => std::env::current_dir()?, // if envvar is invalid or unspecified return current dir
        };
    }

    // creating all intermediate parent directories of filename
    create_dir_all(&parent_dir).context("could not create parent directories!".red())?;
    *filename = parent_dir
        .canonicalize()? // resolves all intermediate symlinks and .. or . dirs
        .join(filename.file_name().unwrap());
    Ok(())
}

pub fn resolve_url(url: &mut String) -> Result<()> {
    lazy_static! {
        static ref YOUTUBE_URL_REGEX: Regex =
            Regex::new(r"^(https?://)?(www\.youtube\.com|youtu\.?be)/.+$").unwrap();
    }
    if YOUTUBE_URL_REGEX.is_match(url) {
        return Ok(());
    }
    let prompt = format!(
        "not a valid youtube url, wanna search \"{}\" on youtube instead? [y/n]: ",
        url
    );
    if user_input::prompt_yes_or_no(&prompt)? {
        *url = format!("'ytsearch:{}'", url);
    }
    Ok(())
}
