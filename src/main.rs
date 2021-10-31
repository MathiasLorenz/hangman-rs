use hangman_rs::*;

fn main() {
    let mut hangman = Hangman::new("DALLER", 3);

    hangman.play();
}
