use hangman_rs::*;

fn main() {
    let hangman = Hangman::new(String::from("Wordyword"));

    play_hangman(hangman);
}
