use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

pub fn read_input() -> String {

    // List of words from https://www.ef.com/wwen/english-resources/english-vocabulary/top-1000-words/
    let file_name: String = String::from("words.txt");
    let mut file = File::open(&file_name).expect("Could not open file.");

    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read file.");

    let possible_words: Vec<&str> = input.trim().split(",").collect();

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0, possible_words.len());

    return String::from(possible_words[random_index]);
}