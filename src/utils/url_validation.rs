use super::user_input;
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

/// resolves and returns validated URL.
/// if URL isn't a valid youtube URL, prompts user to ask whether
/// a youtube search with the URL as keyword should be performed
/// if yes "ytsearch:<keyword>" is returned where <keyword> is substitued with url passed in
pub fn resolve_url(url: &String) -> Result<Option<String>> {
    lazy_static! {
        static ref YOUTUBE_URL_REGEX: Regex =
            Regex::new(r"^(https?://)?(www\.youtube\.com|youtu\.?be)/.+$").unwrap();
    }
    if YOUTUBE_URL_REGEX.is_match(url) {
        return Ok(None);
    }
    let prompt = format!(
        "not a valid youtube url, wanna search \"{}\" on youtube instead? [y/n]: ",
        url
    );
    if user_input::prompt_yes_or_no(&prompt)? {
        return Ok(Some(format!("'ytsearch:{}'", url)));
    }
    Ok(None)
}
