#![allow(unused_variables)]
extern crate clap;
use anyhow::Result;
use utils::argument_parsing::{self, Args};
mod utils;

fn main() -> Result<()> {
    let Args {
        file,
        is_verbose,
        url,
        youtube_dl_args,
        metadata_args,
    } = argument_parsing::get_resolved_arguments()?;
    Ok(())
}
