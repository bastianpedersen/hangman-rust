use std::io;
use std::io::{Read, Write};
use reqwest;
use serde;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Word {
    word: String
}

const API_URL: &str = "https://random-word-api.herokuapp.com/word?lang=es";

impl Word {
    fn is_character_in_word(&self, c: char) -> bool {
        return self.word.chars().any(|a| a == c)
    }

    fn display_word(&self, guessed_chars: &[char]) -> String {
        self.word
            .chars()
            .map(|c| if guessed_chars.contains(&c) { c } else { '_' })
            .collect()
    }
}

fn main() {
    let word = get_word();
    let mut guessed_chars = Vec::new();

    println!(
        "Random word acquired! (hint: it is {} characters long). Now you have to guess it!",
        word.word.len()
    );

    loop {
        println!("{}", word.display_word(&guessed_chars));
        print!("Guess character (or type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();

        if input == "exit" {
            println!("The word was: {}", word.word);
            break;
        }

        if input.len() != 1 {
            println!("Please enter a single character.");
            continue;
        }

        let single_char = input.chars().next().unwrap();
        guessed_chars.push(single_char);

        if word.is_character_in_word(single_char) {
            println!("{} is in the word.", single_char);
        } else {
            println!("{} is NOT in the word.", single_char);
        }

        if word.display_word(&guessed_chars) == word.word {
            println!("Congratulations! You guessed the word: {}", word.word);
            break;
        }
    }
}

fn get_word() -> Word {
    let body = reqwest::blocking::get(API_URL)
        .expect("Failed to fetch word from the API")
        .text()
        .expect("Failed to read response body");
    serde_json::from_str(&body).expect("Failed to parse JSON")
}
