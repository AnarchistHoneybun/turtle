use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_random_word() -> String {
    let file = File::open("words.json").expect("Failed to open words.json");
    let reader = BufReader::new(file);
    let words: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let mut rng = thread_rng();
    let word = words
        .as_array()
        .expect("JSON should be an array")
        .choose(&mut rng)
        .expect("Failed to choose random word")
        .as_str()
        .expect("Word should be a string")
        .to_uppercase();

    word
}

pub fn get_debug_word() -> String {
    "AUDIO".to_string()
}


pub fn check_word(word: &str) -> bool {
    let file = File::open("words.json").expect("Failed to open words.json");
    let reader = BufReader::new(file);
    let words: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    words
        .as_array()
        .expect("JSON should be an array")
        .iter()
        .any(|w| w.as_str().expect("Word should be a string") == word)
}