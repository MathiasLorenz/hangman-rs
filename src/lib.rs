use std::io;

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

        self.print_word();
    }

    fn is_dead(&self) -> bool {
        self.num_guesses == 0
    }

    fn did_win(&self) -> bool {
        self.word.iter().all(|l| l.status == GuessedStatus::Guessed)
    }

    fn print_word(&self) {
        let mut word = String::new();
        for l in self.word.iter() {
            match l.status {
                GuessedStatus::Guessed => word.push(l.letter),
                GuessedStatus::NotGuessed => word.push('*'),
            }
        }
        println!("{}", word);
    }
}

pub fn play_hangman(hangman: &mut Hangman) {
    println!("We are about to play hangman!");
    hangman.print_word();
    let mut read_char: Option<char>;
    while !hangman.did_win() && !hangman.is_dead() {
        println!("Please guess a letter: ");
        read_char = None;
        while read_char.is_none() {
            read_char = read_char_from_stdin();
        }

        hangman.guess(read_char.unwrap());
    }

    if hangman.did_win() {
        println!("Weee you won!");
    } else {
        println!("You lost :((((");
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
