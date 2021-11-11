use std::error;
use std::fmt;
use std::fs;
use std::io;

pub fn read_char_from_stdin() -> Option<char> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    if stdin.read_line(&mut buffer).is_err() {
        println!("You inputted something wrong, try again!");
        return None;
    }
    let c = buffer.chars().next()?;
    Some(c)
}

#[derive(Debug)]
struct NoWordFound;

impl fmt::Display for NoWordFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not get first word for secret word")
    }
}

impl error::Error for NoWordFound {}

pub fn read_secret_word(filename: &str) -> Result<String, Box<dyn error::Error>> {
    let contents = fs::read_to_string(filename)?;

    let word = contents
        .split_ascii_whitespace()
        .next()
        .ok_or(NoWordFound)?;
    Ok(word.to_owned())
}
