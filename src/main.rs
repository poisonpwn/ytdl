extern crate clap;
use clap::{load_yaml, App, Arg};
use std::fs::create_dir_all;
use std::io::{self, Stderr, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

fn resolve_filename(filename: &mut PathBuf) {
    let mut err_handle: Stderr = io::stderr();
    const VIDEO_EXTENSIONS: [&str; 4] = ["mp4", "flv", "mkv", "avi"];
    const MUSIC_EXTENSIONS: [&str; 3] = ["mp3", "aac", "m4a"];
    let extension = &filename
        .extension()
        .unwrap_or_else(|| {
            err_handle.write_all(b"invalid file extension\n").unwrap();
            exit(1)
        })
        .to_str()
        .unwrap();

    let dir_env_var = if VIDEO_EXTENSIONS.contains(extension) {
        "VIDEOS"
    } else if MUSIC_EXTENSIONS.contains(extension) {
        "MUSIC"
    } else {
        err_handle
            .write_all(format!("unsupported format {:?}\n", extension).as_bytes())
            .unwrap();
        exit(1);
    };
    let mut parent_dir: PathBuf;
    match filename.parent() {
        Some(var) => parent_dir = PathBuf::from(var),
        None => return,
    }
    if parent_dir == Path::new("") {
        let media_dir_string = std::env::var(dir_env_var).unwrap_or_else(|_| {
            String::from(
                std::env::current_dir()
                    .unwrap_or_else(|_| {
                        err_handle.write_all(b"current dir not found!\n").unwrap();
                        exit(1);
                    })
                    .to_str()
                    .unwrap(),
            )
        });
        parent_dir = PathBuf::from(media_dir_string);
    }

    create_dir_all(&parent_dir).unwrap_or_else(|_| {
        err_handle
            .write_all(b"could not create parent dirs\n")
            .unwrap();
        exit(1);
    });
    *filename = parent_dir
        .canonicalize()
        .unwrap()
        .join(&*(filename.file_name().unwrap()).to_str().unwrap());
}

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
    let mut filename = PathBuf::from(matches.value_of("FILEPATH").unwrap());
    resolve_filename(&mut filename);
    println!("{}", filename.to_str().unwrap());
}
