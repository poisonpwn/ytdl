use anyhow::Result;
use std::io::{self, Write};

/// prompt user provide a yay or nay response and returns `true` or `false` respectively.
/// if user inputs something other than y/n (case insensitive),
/// keep asking till valid input is given
pub fn prompt_yes_or_no(prompt: &String) -> Result<bool> {
    // input buffer in which the response is stored in each iteration
    let mut input_buffer = String::new();
    let (mut stdout, stdin) = (io::stdout(), io::stdin());
    let yay_or_nay = loop {
        print!("{}", prompt);
        // flushing is required so that old chars don't pollute the stdout buffer
        stdout.flush()?;
        stdin.read_line(&mut input_buffer)?;

        // trim() is required to strip any trailing whitespace so that
        // response doesn't match whitespace
        break match input_buffer.to_lowercase().trim() {
            "y" => true,
            "n" => false,
            invalid_reply => {
                // user inputted an invalid response!, ask again
                println!("invalid input: {}", invalid_reply);
                input_buffer.clear(); // clear last reply from buffer
                continue;
            }
        };
    };
    Ok(yay_or_nay)
}
