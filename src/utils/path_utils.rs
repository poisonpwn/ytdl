use anyhow::{bail, Context, Result};
use colored::*;
use getset::Getters;
use std::ffi::{OsStr, OsString};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

macro_rules! map_arr {
    //  macro for mapping each value in array to a diff type
    ($func:path,$($e:expr),*) => {[$($func($e),)*]}
}

pub enum Format {
    Video(OsString),
    Audio(OsString),
}

impl Format {
    const AUDIO_VAR: &'static str = "MUSIC";
    const VIDEO_VAR: &'static str = "VIDEOS";

    /// returns Format enum based on the file extension passed in
    /// video if extension is `"mp4", "flv", "mkv", or "avi"`
    /// and audio if `"mp3", "aac", or "m4a"`.
    pub fn new(extension: &OsStr) -> Result<Format> {
        let env_var = Format::get_media_dir_env_var(extension)?; // will return error if extension is invalid.
        let extension = extension.to_owned();
        let return_val = if env_var == Format::AUDIO_VAR {
            Format::Audio(extension)
        } else {
            Format::Video(extension)
        };
        Ok(return_val)
    }

    fn get_media_dir_env_var(extension: &OsStr) -> Result<&str> {
        let video_extensions = map_arr!(OsStr::new, "mp4", "flv", "mkv", "avi"); // video extensions supported
        let music_extensions = map_arr!(OsStr::new, "mp3", "aac", "m4a"); // audio extensions supported
        return if video_extensions.contains(&extension) {
            // is supported video format
            Ok(Format::VIDEO_VAR)
        } else if music_extensions.contains(&extension) {
            // is supported audio format
            Ok(Format::AUDIO_VAR)
        } else {
            // unsupported format!, bail with error message
            bail!("unsupported format {:?}", extension);
        };
    }
}

#[derive(Getters)]
pub struct MediaFile {
    #[get = "pub"]
    filepath: PathBuf,
    #[get = "pub"]
    format: Format,
}

impl MediaFile {
    /// Constructs a MediaFile struct from resolved filename and format
    /// files not having parent directory (i.e if `Path.parent() == ""`) and ending in a recognized media format
    /// parent will be resolved to `std::env::vars_os(ENV_VAR)` where `ENV_VAR` is decided based on file extension
    /// if `ENV_VAR` is unset, current directory is returned as parent.
    pub fn new(filepath: &Path) -> Result<MediaFile> {
        let resolved_filepath = MediaFile::get_resolved_filepath(&filepath)?;
        Ok(MediaFile {
            filepath: match resolved_filepath {
                Some(new_path) => new_path,
                None => filepath.to_owned(),
            },
            format: Format::new(filepath.extension().unwrap())?,
        })
    }

    fn get_resolved_filepath(filepath: &Path) -> Result<Option<PathBuf>> {
        let mut parent_dir = match filepath.parent() {
            Some(parent_dir) => PathBuf::from(parent_dir),
            // means that filepath's parent is root or some prefix (i.e no need to resolve)
            None => return Ok(None),
        };

        let extension = filepath.extension().context("invalid filename!".red())?;
        if parent_dir == Path::new("") {
            // filepath only given (resolve to media dir if possible)
            // get parent dir from env var ("MUSIC" or "VIDEOS"), depending on file extension
            parent_dir = MediaFile::get_resolved_parent_dir(extension)?;
        }
        // creating all intermediate parent directories of filepath
        create_dir_all(&parent_dir).context("could not create parent directories!".red())?;
        let resolved_filepath = parent_dir
            .canonicalize()? // resolves all intermediate symlinks and .. or . dirs
            .join(filepath.file_name().unwrap());
        Ok(Some(resolved_filepath))
    }

    fn get_resolved_parent_dir(extension: &OsStr) -> Result<PathBuf> {
        let media_dir_env_var = Format::get_media_dir_env_var(extension)?;
        let parent_dir = match std::env::var_os(media_dir_env_var) {
            Some(media_dir) => PathBuf::from(media_dir),
            // if envvar is invalid or unspecified, return current dir
            None => std::env::current_dir()?,
        };
        Ok(parent_dir)
    }
}
