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

fn get_media_dir_from_ext(extension: &OsStr) -> Result<&str> {
    let video_extensions = map_arr!(OsStr::new, "mp4", "flv", "mkv", "avi"); // video extensions supported
    let music_extensions = map_arr!(OsStr::new, "mp3", "aac", "m4a"); // audio extensions supported
    let media_var = if video_extensions.contains(&extension) {
        // is supported video format
        "VIDEOS"
    } else if music_extensions.contains(&extension) {
        // is supported audio format
        "MUSIC"
    } else {
        // unsupported format!, bail with error message
        bail!("unsupported format {:?}", extension);
    };
    Ok(media_var)
}

fn get_resolved_parent_dir(extension: &OsStr) -> Result<PathBuf> {
    let media_var = get_media_dir_from_ext(extension)?; // find media dir envvar based on file extension
    let parent_dir = match std::env::var_os(media_var) {
        Some(media_dir) => PathBuf::from(media_dir),
        // if envvar is invalid or unspecified, return current dir
        None => std::env::current_dir()?,
    };
    Ok(parent_dir)
}

/// directly mutates filepath to resolve all ambiguity
/// files not having parent directory (i.e if `Path.parent() == ""`) and ending in a recognized video format
/// parent will be resolved to `std::env::vars_os(ENV_VAR)` where `ENV_VAR` is decided based on file extension
/// if `ENV_VAR` is unset, current directory is returned as parent.
pub fn resolve_filepath(filepath: &mut PathBuf) -> Result<()> {
    let mut parent_dir = match filepath.parent() {
        Some(parent_dir) => PathBuf::from(parent_dir),
        // means that filepath's parent is root or some prefix (i.e no need to resolve)
        None => return Ok(()),
    };

    if parent_dir == Path::new("") {
        // filepath only given (resolve to media dir if possible)
        let extension = filepath.extension().context("invalid filename!".red())?;
        // get parent dir from env var ("MUSIC" or "VIDEOS"), depending on file extension
        parent_dir = get_resolved_parent_dir(extension)?;
    }

    // creating all intermediate parent directories of filepath
    create_dir_all(&parent_dir).context("could not create parent directories!".red())?;
    *filepath = parent_dir
        .canonicalize()? // resolves all intermediate symlinks and .. or . dirs
        .join(filepath.file_name().unwrap());
    Ok(())
}

/// directly mutates and validates URL.
/// if URL isn't a valid youtube URL, prompts user to ask whether
/// a youtube search with the URL as keyword should be performed
/// if yes "ytsearch:<keyword>" is returned where <keyword> is substitued with url passed in
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
