use std::error;
use std::fmt;
use std::fs;
use std::io;

pub struct Hangman {
    word: Vec<LetterStatus>,
    num_guesses: u8, // Todo: Make this optional. Either directly (Option<T>)
                     //       or derive_builder/typed_builder crate
}

struct LetterStatus {
    letter: char,
    status: GuessedStatus,
}

#[derive(PartialEq, Eq)]
enum GuessedStatus {
    Guessed,
    NotGuessed,
}

impl Hangman {
    pub fn new(word: &str, num_guesses: u8) -> Self {
        // This will make us iterate over word twice but should be more than fine.
        // Did create a solution with .map() and .scan() in the below but that was super ugly
        if word.chars().any(|c| !c.is_alphabetic()) {
            panic!("Cannot play hangman with characters that are not letters :(");
        }

        let w: Vec<_> = word
            .to_ascii_lowercase()
            .chars()
            .map(|c| LetterStatus {
                letter: c,
                status: GuessedStatus::NotGuessed,
            })
            .collect();

        Hangman {
            word: w,
            num_guesses,
        }
    }

    pub fn play(&mut self) {
        println!("We are about to play hangman!");
        println!("Your word to guess has {} letters", self.word.len());
        let mut read_char: Option<char>;
        while !self.did_win() && !self.is_dead() {
            println!("Please guess a letter: ");
            read_char = None;
            while read_char.is_none() {
                read_char = read_char_from_stdin();
            }

            self.guess(read_char.unwrap());
        }

        if self.did_win() {
            println!("Weee you won!");
        } else {
            println!("You lost :((((");
        }
    }

    fn guess(&mut self, guess: char) {
        let mut did_guess = false;
        for c in self.word.iter_mut() {
            if c.letter == guess {
                c.status = GuessedStatus::Guessed;
                did_guess = true;
            }
        }

        if did_guess {
            println!("Nice, you guessed a letter! Therefore you did not use a guess.");
        } else {
            println!("Damn, you did not guess a letter and you lose a guess :(");
            self.num_guesses -= 1;
        }

        println!("You now have {} guesses left", self.num_guesses);

        println!("{}", self.construct_obfuscated_word());
    }

    fn is_dead(&self) -> bool {
        self.num_guesses == 0
    }

    fn did_win(&self) -> bool {
        self.word.iter().all(|l| l.status == GuessedStatus::Guessed)
    }

    fn construct_obfuscated_word(&self) -> String {
        let mut word = String::new();
        for l in self.word.iter() {
            match l.status {
                GuessedStatus::Guessed => word.push(l.letter),
                GuessedStatus::NotGuessed => word.push('*'),
            }
        }
        word
    }
}

fn read_char_from_stdin() -> Option<char> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hangman_new_simple_word_is_read_properly() {
        let word = "sup";
        let hangman = Hangman::new(word, 2);
        let hangman_word = extract_word_from_letterstatus(hangman.word);

        assert_eq!(word, hangman_word);
    }

    #[test]
    fn hangman_new_word_with_uppercase_is_translated_to_lowercase() {
        let word = "hellOwoM";
        let word_lowercase = "hellowom";
        let hangman = Hangman::new(word, 2);
        let hangman_word = extract_word_from_letterstatus(hangman.word);

        assert_eq!(word_lowercase, hangman_word);
    }

    #[test]
    #[should_panic]
    fn hangman_new_with_non_alphabetic_panics() {
        let word = "2";
        let _ = Hangman::new(word, 2);
    }

    fn extract_word_from_letterstatus(letter_status: Vec<LetterStatus>) -> String {
        letter_status.into_iter().map(|x| x.letter).collect()
    }

    #[test]
    fn hangman_guess_correct_num_guesses_unchanged() {
        let word = "abc";
        let num_guesses = 2;
        let mut hangman = Hangman::new(word, 2);

        hangman.guess('a');

        assert_eq!(hangman.num_guesses, num_guesses);
    }

    #[test]
    fn hangman_guess_incorrect_num_guesses_minus_1() {
        let word = "abc";
        let num_guesses = 2;
        let mut hangman = Hangman::new(word, 2);

        hangman.guess('d');

        assert_eq!(hangman.num_guesses, num_guesses - 1);
    }

    #[test]
    fn hangman_construct_obfuscated_word() {
        let word = "abc";
        let mut hangman = Hangman::new(word, 2);

        hangman.guess('a');
        hangman.guess('c');

        assert_eq!(hangman.construct_obfuscated_word(), "a*c");
    }
}
