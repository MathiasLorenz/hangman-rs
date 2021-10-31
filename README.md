# hangman-rs

Small implementation of the 'hangman' game to practice Rust. Write the secret word (word to guess) in the `secret_word.txt` file and run the executable.

## Rules

You need to guess the secret word in less than `num_guesses` guesses. Play by inputting a single letter to guess (in each turn only the first char will be read). If you hit a letter, it is revealed and you do not use a guess (of the `num_guesses`). If you miss (i.e. the provided letter is not part of the secret word) then you lose a `num_guesses`. If you guess the word before getting to 0 guesses left you win - otherwise you lose.

## Todo

- Make `num_guesses` optional
- Make number of guesses input to the program.
  - In `secret_word.txt` file?
  - As program input (use `clap/arguably` or similar?)