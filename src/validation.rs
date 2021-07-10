use anyhow::{bail, Context, Result};
use colored::*;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

macro_rules! map_arr {
    //  macro for mapping each value in array to a diff type
    ($func:path,$($e:expr),*) => {[$($func($e),)*]}
}

pub fn resolve_filename(filename: &mut PathBuf) -> Result<()> {
    let video_extensions = map_arr!(OsStr::new, "mp4", "flv", "mkv", "avi"); // video extensions supported
    let music_extensions = map_arr!(OsStr::new, "mp3", "aac", "m4a");
    let extension = filename.extension().context("invalid filename!".red())?;

    let dir_env_var = if video_extensions.contains(&extension) {
        "VIDEOS"
    } else if music_extensions.contains(&extension) {
        "MUSIC"
    } else {
        bail!("unsupported format {:?}", extension);
    };

    let mut parent_dir = match filename.parent() {
        Some(parent_dir) => PathBuf::from(parent_dir),
        None => return Ok(()),
    };

    if parent_dir == Path::new("") {
        parent_dir = match std::env::var_os(dir_env_var) {
            Some(media_dir) => PathBuf::from(media_dir),
            None => std::env::current_dir()?,
        };
    }

    // creating all intermediate parent directories of filename
    create_dir_all(&parent_dir).context("could not create parent directories!".red())?;
    *filename = parent_dir
        .canonicalize()? // resolves all intermediate symlinks and .. or . dirs
        .join(filename.file_name().unwrap());
    Ok(())
}
