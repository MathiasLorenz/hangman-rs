use anyhow::{Result, bail};
use crate::utils::{read_char_from_stdin, LowercaseAscii};
use std::collections::HashSet;

pub struct Hangman {
    word: Vec<LetterStatus>,
    num_guesses: u8,
    guessed_chars: HashSet<char>,
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

#[derive(PartialEq, Eq, Debug)]
enum GuessOutcome {
    AlreadyGuessed,
    Hit,
    Miss,
}

impl Hangman {
    pub fn new(word: &str, num_guesses: u8) -> Result<Self> {
        let word = word.to_ascii_lowercase();
        // Did create a solution with .map() and .scan() in the below but that was super ugly
        if !word.chars().all(|c| matches!(c, 'a'..='z')) {
            bail!("Cannot play hangman with characters that are not letters :(");
        }

        let w: Vec<_> = word
            .chars()
            .map(|c| LetterStatus {
                letter: c,
                status: GuessedStatus::NotGuessed,
            })
            .collect();

        Ok(Hangman {
            word: w,
            num_guesses,
            guessed_chars: HashSet::new(),
        })
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

            let read_char = read_char.unwrap();
            match LowercaseAscii::try_from(read_char) {
                Ok(guess) => {
                    let guess_outcome = self.apply_guess(&guess);
                    self.print_guess_outcome(&guess_outcome);
                },
                Err(err) => println!("{}", err),
            }
        }

        if self.did_win() {
            println!("Weee you won!");
        } else {
            println!("You lost :((((");
        }
    }

    fn print_guess_outcome(&self, guess_outcome: &GuessOutcome) {
        match guess_outcome {
            GuessOutcome::AlreadyGuessed => println!("You have already guessed that. Try something else."),
            GuessOutcome::Hit => println!("Wuu you guessed a letter! No guess spent!"),
            GuessOutcome::Miss => { println!("Damn, the word does not contain that letter.. Try something else!") },
        }

        println!("You now have {} guesses left", self.num_guesses);
        println!("{}", self.construct_obfuscated_word());
    }

    fn apply_guess(&mut self, guess: &LowercaseAscii) -> GuessOutcome {
        let guess = guess.get_value();
        if self.guessed_chars.contains(&guess) {
            return GuessOutcome::AlreadyGuessed
        }
        self.guessed_chars.insert(guess);

        let mut did_guess = false;
        for c in &mut self.word {
            if c.letter == guess {
                c.status = GuessedStatus::Guessed;
                did_guess = true;
            }
        }

        if did_guess { GuessOutcome::Hit } 
        else {
            self.num_guesses -= 1;
            GuessOutcome::Miss
        }
    }

    fn is_dead(&self) -> bool {
        self.num_guesses == 0
    }

    fn did_win(&self) -> bool {
        self.word.iter().all(|l| l.status == GuessedStatus::Guessed)
    }

    fn construct_obfuscated_word(&self) -> String {
        let mut word = String::new();
        for l in &self.word {
            match l.status {
                GuessedStatus::Guessed => word.push(l.letter),
                GuessedStatus::NotGuessed => word.push('*'),
            }
        }
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hangman_new_simple_word_is_read_properly() {
        let word = "sup";
        let hangman = Hangman::new(word, 2).unwrap();
        let hangman_word = extract_word_from_letterstatus(hangman.word);

        assert_eq!(word, hangman_word);
    }

    #[test]
    fn hangman_new_word_with_uppercase_is_translated_to_lowercase() {
        let word = "hellOwoM";
        let word_lowercase = "hellowom";
        let hangman = Hangman::new(word, 2).unwrap();
        let hangman_word = extract_word_from_letterstatus(hangman.word);

        assert_eq!(word_lowercase, hangman_word);
    }

    #[test]
    fn hangman_new_with_digit_returns_error() {
        let word = "2";
        let hangman = Hangman::new(word, 2);

        assert_eq!(hangman.is_err(), true);
    }

    #[test]
    fn hangman_new_strange_unicode_alphabetic_returns_error() {
        let word = "こ";
        let hangman = Hangman::new(word, 2);

        assert_eq!(hangman.is_err(), true);
    }

    fn extract_word_from_letterstatus(letter_status: Vec<LetterStatus>) -> String {
        letter_status.into_iter().map(|x| x.letter).collect()
    }

    #[test]
    fn hangman_guess_correct_num_guesses_unchanged() {
        let word = "abc";
        let num_guesses = 2;
        let mut hangman = Hangman::new(word, num_guesses).unwrap();
        let guess = LowercaseAscii::try_from('a').unwrap();

        let guess_outcome = hangman.apply_guess(&guess);

        assert_eq!(hangman.num_guesses, num_guesses);
        assert_eq!(guess_outcome, GuessOutcome::Hit);
    }

    #[test]
    fn hangman_guess_incorrect_num_guesses_minus_1() {
        let word = "abc";
        let num_guesses = 2;
        let mut hangman = Hangman::new(word, num_guesses).unwrap();
        let guess = LowercaseAscii::try_from('d').unwrap();

        let guess_outcome = hangman.apply_guess(&guess);

        assert_eq!(hangman.num_guesses, num_guesses - 1);
        assert_eq!(guess_outcome, GuessOutcome::Miss);
    }

    #[test]
    fn hangman_guess_char_with_multiple_occurrences_all_are_noted() {
        let word = "aabc";
        let num_guesses = 2;
        let mut hangman = Hangman::new(word, num_guesses).unwrap();
        let guess = LowercaseAscii::try_from('a').unwrap();

        hangman.apply_guess(&guess);

        let expected_obfuscated_word = "aa**";
        assert_eq!(hangman.num_guesses, num_guesses);
        assert_eq!(
            hangman.construct_obfuscated_word(),
            expected_obfuscated_word
        );
    }

    #[test]
    fn hangman_apply_guess_already_tried_no_guess_used() {
        let word = "abc";
        let num_guesses = 2;
        let mut hangman = Hangman::new(word, num_guesses).unwrap();
        let guess = LowercaseAscii::try_from('a').unwrap();

        hangman.apply_guess(&guess);
        let guess_outcome = hangman.apply_guess(&guess);

        assert_eq!(hangman.num_guesses, num_guesses);
        assert_eq!(guess_outcome, GuessOutcome::AlreadyGuessed);
    }
}
