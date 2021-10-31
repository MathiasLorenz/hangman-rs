use hangman_rs::*;

fn main() {
    let secret_word = read_secret_word("secret_word.txt").unwrap();
    let mut hangman = Hangman::new(&secret_word, 3);

    hangman.play();
}
