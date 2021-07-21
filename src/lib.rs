pub struct Hangman {
    word: Vec<LetterStatus>,
}

struct LetterStatus {
  letter: char,
  status: GuessedStatus
}

enum GuessedStatus {
  Guessed,
  NotGuessed
}

impl Hangman {
  pub fn new(word: String) -> Self {
    let w: Vec<_> = word.chars().map(|c| LetterStatus {
      letter: c,
      status: GuessedStatus::NotGuessed
    }).collect();

    Hangman {
      word: w
    }
  }

  pub fn print_word(self) {
    for l in self.word.iter() {
      match l.status {
        GuessedStatus::Guessed => println!("{}", format!("Guessed, letter is: {}", l.letter)),
        GuessedStatus::NotGuessed => println!("{}", format!("Not guessed, letter is: {}", l.letter)),
      }
    }

    ()
  }
}

pub fn play_hangman(hangman: Hangman) {
  println!("We are about to play hangman!");
  hangman.print_word();
    //println!("{}", format!("The word is: {}", hangman.word));
}
