// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let mut guesses = NUM_INCORRECT_GUESSES;
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut word_so_far: Vec<char> = secret_word_chars.iter().map(|_| '-').collect();

    println!("Welcome to CS110L Hangman!");

    while guesses > 0 {
        // messages
        let s: String = word_so_far.iter().collect();
        println!("The word so far is {}", s);
        let s: String = guessed_letters.iter().collect();
        println!("You have guessed the following letters: {}", s);
        println!("You have {} guesses left", guesses);
        print!("Please guess a letter: ");

        // input a character
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let guess = guess.chars().next().expect("Error reading character.");

        guessed_letters.push(guess);
        let mut is_right = false;
        let len = secret_word_chars.len();
        for i in 0..len {
            if secret_word_chars[i] == guess {
                is_right = true;
                word_so_far[i] = guess;
            }
        }

        if !is_right {
            guesses -= 1;
            println!("Sorry, that letter is not in the word");
        }
        println!("");

        if word_so_far == secret_word_chars {
            break;
        }
    }
    if guesses == 0 {
        println!("Sorry, you ran out of guesses!");
    } else {
        println!(
            "Congratulations you guessed the secret word: {}!",
            secret_word
        );
    }
}
