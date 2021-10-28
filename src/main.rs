use hangman_rs::*;

fn main() {
    let mut hangman = Hangman::new("oo", 8);

    play_hangman(&mut hangman);
}
