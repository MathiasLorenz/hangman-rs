use hangman_rs::*;

fn main() {
    let mut hangman = Hangman::new("man", 3);

    play_hangman(&mut hangman);
}
