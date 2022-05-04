use rand::Rng;
use std::fs;

const WORDS_FILE: &str = "src/words.txt";

fn get_words_file_content() -> String {
  fs::read_to_string(WORDS_FILE).unwrap()
}

fn get_words() -> Vec<String> {
  get_words_file_content()
    .split("\n")
    .map(|x| x.to_string())
    .collect()
}

pub fn get_random_word() -> String {
  let mut rng = rand::thread_rng();
  let words = get_words();
  words[rng.gen_range(0..words.len())].to_string()
}
