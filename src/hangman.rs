use crate::utils::{read_char_from_stdin, LowercaseAscii};

pub struct Hangman {
    word: Vec<LetterStatus>,
    num_guesses: u8,
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
        let word = word.to_ascii_lowercase();
        // Did create a solution with .map() and .scan() in the below but that was super ugly
        if !word.chars().all(|c| matches!(c, 'a'..='z')) {
            panic!("Cannot play hangman with characters that are not letters :(");
        }

        let w: Vec<_> = word
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

            let read_char = read_char.unwrap();
            match LowercaseAscii::try_from(read_char) {
                Ok(read_char) => self.guess(&read_char),
                Err(err) => println!("{}", err),
            }
        }

        if self.did_win() {
            println!("Weee you won!");
        } else {
            println!("You lost :((((");
        }
    }

    fn guess(&mut self, guess: &LowercaseAscii) {
        let mut did_guess = false;
        for c in &mut self.word {
            if c.letter == guess.get_value() {
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
    fn hangman_new_with_digit_panics() {
        let word = "2";
        let _ = Hangman::new(word, 2);
    }

    #[test]
    #[should_panic]
    fn hangman_new_strange_unicode_alphabetic_panics() {
        let word = "こ";
        Hangman::new(word, 2);
    }

    fn extract_word_from_letterstatus(letter_status: Vec<LetterStatus>) -> String {
        letter_status.into_iter().map(|x| x.letter).collect()
    }

    #[test]
    fn hangman_guess_correct_num_guesses_unchanged() {
        let word = "abc";
        let num_guesses = 2;
        let mut hangman = Hangman::new(word, 2);
        let guess = LowercaseAscii::try_from('a').unwrap();

        hangman.guess(&guess);

        assert_eq!(hangman.num_guesses, num_guesses);
    }

    #[test]
    fn hangman_guess_incorrect_num_guesses_minus_1() {
        let word = "abc";
        let num_guesses = 2;
        let mut hangman = Hangman::new(word, 2);
        let guess = LowercaseAscii::try_from('d').unwrap();

        hangman.guess(&guess);
        assert_eq!(hangman.num_guesses, num_guesses - 1);
    }
}
