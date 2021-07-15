use anyhow::Result;
use std::io::{self,Write};

pub fn prompt_yes_or_no(prompt: &String) -> Result<bool> {
    let mut input_buffer = String::new();
    let (mut stdout, stdin) = (io::stdout(), io::stdin());
    let yay_or_nay = loop {
        print!("{}",prompt);
        stdout.flush()?;
        stdin.read_line(&mut input_buffer)?;
        break match input_buffer.trim() {
            "y" | "Y" => true,
            "n" | "N "=> false,
            invalid_reply => {
                println!("invalid input: {}", invalid_reply);
                input_buffer.clear();
                continue;
            },
        };
    };
    Ok(yay_or_nay)
}
