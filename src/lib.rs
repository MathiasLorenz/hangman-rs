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
        for c in self.word.iter_mut() {
            if c.letter == guess {
                c.status = GuessedStatus::Guessed
            }
        }

        self.num_guesses -= 1;
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
    while !hangman.did_win() && !hangman.is_dead() {
        print!("Please guess a letter: ");
        hangman.guess('o');
    }

    if hangman.did_win() {
        println!("Weee you won!");
    } else {
        println!("You lost :((((");
    }
}
