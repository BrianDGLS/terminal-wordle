mod words;
use termion::color;

fn get_player_guess() -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("Type a 5 letter word: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    if s.len() != 5 {
        return get_player_guess();
    } else {
        return s;
    }
}

fn mask(word: String, guess: String) -> String {
    let lowercase_word = word.to_lowercase();
    let lowercase_guess = guess.to_lowercase();
    lowercase_guess
        .split("")
        .enumerate()
        .map(|(i, c)| {
            let contains = lowercase_word.contains(c);
            let indices: Vec<usize> = lowercase_word
                .match_indices(c)
                .map(|(x, _)| x + 1)
                .collect();
            let index_match = indices.contains(&i);

            match (contains, index_match) {
                (true, true) => format!(
                    "{}{}{}",
                    color::Bg(color::Green),
                    c.to_string(),
                    color::Bg(color::Reset)
                ),
                (true, false) => format!(
                    "{}{}{}",
                    color::Bg(color::Yellow),
                    c.to_string(),
                    color::Bg(color::Reset)
                ),
                (_, _) => String::from("-"),
            }
        })
        .collect()
}

fn main() {
    let word = words::get_random_word();
    let mut guess_count = 0;
    let mut guessed_correct = false;
    let mut guesses = vec![String::from("-----"); 6];
    while guess_count < 6 && !guessed_correct {
        for guess in &guesses {
            println!("{}{}", guess, color::Bg(color::Reset));
        }
        let current_guess: &str = &get_player_guess();
        guesses[guess_count] = mask(String::from(&word), String::from(current_guess));
        guessed_correct = current_guess.eq_ignore_ascii_case(&word);
        guess_count += 1;
    }

    for guess in &guesses {
        println!("{}{}", guess, color::Bg(color::Reset));
    }

    if guessed_correct {
        println!("Congrats, you guessed correct with '{}'.", word);
    } else {
        println!("Sorry, the correct word was '{}'.", word);
    }
}
