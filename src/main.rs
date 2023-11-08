use std::error::Error;
use std::io;
use std::io::Write;
use reqwest;
use serde;
use serde::Deserialize;
use tokio;

#[derive(Deserialize, Debug)]
struct Word {
    word: String,

    #[serde(skip_deserializing)]
    guessed_chars: Vec<char>
}

const API_URL: &str = "https://random-word-api.herokuapp.com/word?lang=en";

impl Word {
    fn contains(&self, c: char) -> bool {
        self.word.chars().any(|a| a == c)
    }

    fn add_guessed_char(&mut self, c: char) {
        self.guessed_chars.push(c);
    }

    fn display_word(&self) -> String {
        self.word
            .chars()
            .map(|c| if self.guessed_chars.contains(&c) { c } else { '_' })
            .collect()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut word = get_word().await?;

    println!(
        "Random word acquired! (hint: it is {} characters long). Now you have to guess it!",
        word.word.chars().count()
    );

    loop {
        println!("{}", word.display_word());
        print!("Guess character (or type '!exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();

        if input == word.word {
            println!("Congratulations! You guessed the word: {}.", word.word);
            break;
        }

        if input == "!exit" {
            println!("The word was: {}.", word.word);
            break;
        }

        if input == "!list" {
            println!("{}", word.guessed_chars.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(","));
            break;
        }

        if input.chars().count() != 1 {
            println!("Please enter a single character.");
            continue;
        }

        let single_char = input.chars().next().unwrap();
        word.add_guessed_char(single_char);

        if word.contains(single_char) {
            println!("{} is in the word.", single_char);
        } else {
            println!("{} is NOT in the word.", single_char);
        }

        if word.display_word() == word.word {
            println!("Congratulations! You guessed the word: {}.", word.word);
            break;
        }
    }

    Ok(())
}

async fn get_word() -> Result<Word, Box<dyn Error>> {
    let body = reqwest::get(API_URL)
        .await?
        .text()
        .await?;

    serde_json::from_str(&body).map_err(|err| Box::new(err) as Box<dyn Error>)
}
