use std::cmp::min;

pub struct Hangman {
    word: String,
    guesses_remaining: usize,
    letters: Vec<char>,
    progress: Vec<Letter>,
    letters_guessed: Vec<char>
}

struct Letter {
    character: char,
    guessed: bool
}

pub enum Status {
    Won,
    Lost,
    InProgress
}

impl Hangman {
    pub fn new(chosen_word: &str) -> Hangman {

        let mut progress: Vec<Letter> = Vec::new();

        for l in chosen_word.chars() {
            progress.push(Letter {
                character: l,
                guessed: false
            });
        }

        Hangman {
            word: chosen_word.to_string(),
            guesses_remaining: match chosen_word.len() {
                1..=6 => 6,
                x => min(x + 2, 15) 
            },
            letters: vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ],
            progress: progress,
            letters_guessed: Vec::new()
        }
    }

    pub fn make_guess(&mut self, c: &char) -> Status {

        let c: char = c.to_ascii_lowercase();
        let mut decrement: bool = true;
        
        if !self.letters.contains(&c) {
            println!("{} is not a letter.", c);
            self.print_status();
            return Status::InProgress;

        } else if self.letters_guessed.contains(&c) {
            decrement = false;
            
        } else if self.word.contains(&c.to_string()) {
            self.update_progress(c);
            decrement = false;
        }

        self.update_guesses(c);    
        return self.handle_guess(decrement);
    }

    pub fn handle_guess(&mut self, decrement: bool) -> Status {

        if decrement { self.guesses_remaining -= 1; }

        self.print_status();
        return self.has_ended();
    }

    pub fn update_guesses(&mut self, guess: char) {
        if !self.letters_guessed.contains(&guess) { self.letters_guessed.push(guess); }
    }

    pub fn update_progress(&mut self, guess: char) {
        for l in self.progress.iter_mut() {
            if l.character == guess { l.guessed = true; }
        }
    }

    pub fn has_ended(&self) -> Status {
        if self.guesses_remaining <= 0 {
            return Status::Lost;
        } else if self.progress.iter().filter(|x| x.guessed == false).count() == 0 {
            return Status::Won;
        }
        return Status::InProgress;
    }

    pub fn print_status(&mut self) {
        println!("Letters guessed so far: ");
        self.letters_guessed.sort();
        for l in self.letters_guessed.iter() { print!("{} ", l ); }
        print!("\nProgess:");
        
        for l in self.progress.iter() {
            if l.guessed {
                print!(" {}", l.character);
            } else {
                print!(" _");
            }
        }

        println!("\nYou have {} guesses left.", self.guesses_remaining);
    }
}
