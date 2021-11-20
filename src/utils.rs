use anyhow::Result;
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

pub fn read_secret_word(filename: &str) -> Result<String, anyhow::Error> {
    let contents = fs::read_to_string(filename)?;

    let word = contents
        .split_ascii_whitespace()
        .next()
        .ok_or(NoWordFound)?;
    Ok(word.to_owned())
}

pub struct LowercaseAscii {
    value: char,
}

impl TryFrom<char> for LowercaseAscii {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let value = value.to_ascii_lowercase();
        if matches!(value, 'a'..='z') {
            Ok(Self { value })
        } else {
            Err(format!(
                "Input was {} but has to be valid char a through z",
                value
            ))
        }
    }
}

impl LowercaseAscii {
    pub fn get_value(&self) -> char {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::LowercaseAscii;

    #[test]
    fn lowercase_ascii_a_is_accepted() {
        let result = LowercaseAscii::try_from('a');
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn lowercase_ascii_bracket_is_not_accepted() {
        let result = LowercaseAscii::try_from('[');
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn lowercase_ascii_strange_unicode_is_not_accepted() {
        let result = LowercaseAscii::try_from('こ');
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn lowercase_ascii_uppercase_a_is_lowercased_and_accepted() {
        let result = LowercaseAscii::try_from('A');
        assert_eq!(result.is_ok(), true);
    }
}
