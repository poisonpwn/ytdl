extern crate clap;
use anyhow::Result;
use utils::argument_parsing;
use utils::exec_command;
use utils::path_utils::{Format::Audio, MediaFile};
mod utils;

fn main() -> Result<()> {
    let args = argument_parsing::get_resolved_arguments()?;
    let MediaFile { filepath, format } = &args.file;
    println!("outfile: {}", filepath.to_string_lossy());

    exec_command::run_youtube_dl(&args)?; // run youtube-dl
    if let Audio(_) = format {
        // check if file is audio file
        exec_command::run_eye_d3(filepath, &args.metadata_args)?; // run eyeD3
    }
    Ok(())
}
