use arguably::ArgParser;
mod hangman;
mod utils;

fn main() {
    let mut args_parser = ArgParser::new()
        .helptext("Hangman! Please input word to guess in 'secret_word.txt' and pass number of guesses with option")
        .version("0.1")
        .option("num_guesses n", "4");

    if let Err(err) = args_parser.parse() {
        err.exit();
    }

    let num_guesses: u8 = args_parser
        .value("num_guesses")
        .parse()
        .expect("Could not parse num_guesses from input");

    let secret_word = utils::read_secret_word("secret_word.txt").unwrap();
    let mut hangman = hangman::Hangman::new(&secret_word, num_guesses);

    hangman.play();
}
