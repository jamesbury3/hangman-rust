use std;
mod hangman;
mod file_reader;

fn main() {

    print_welcome();
    let word: String = file_reader::read_input();
    let mut h = hangman::Hangman::new(&word);
    h.print_status();
    
    loop {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)
            .expect("Please enter a letter");

        if input.trim() == "quit" {
            break;
        }

        let characters: Vec<char> = input.chars().collect();
        let game: hangman::Game = h.make_guess(&characters[0]);

        match game.status {
            hangman::Status::Won => {
                println!("Congratulations, you won!");
                break;
            },
            hangman::Status::Lost => {
                println!("Sorry, you lost! The word was \"{}\".", word);
                break;
            },
            hangman::Status::InProgress => ()
        }
    }

    fn print_welcome() {
        println!("Welcome to Hangman! Please type a letter to make a guess, or type \"quit\" to exit the game.");
    }
}


