#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::Rng;
use random_word::Lang;
use std::{
    io::{self, Write},
    thread::sleep,
};
use std::{
    process::exit,
    time::{Duration, Instant},
};

fn generate_sentence(mut words: usize) -> String {
    let mut rng = rand::rng();
    let mut sentence = String::new();
    while words > 0 {
        let word_length = rng.random_range(2..=10);
        if let Some(word) = random_word::gen_len(word_length, Lang::En) {
            words -= 1;
            sentence.push_str(word);
            if words > 0 {
                sentence.push_str(" ");
            }
        }
    }
    sentence
}

fn print_stats(typed_text: String, original_text: String) {
    assert_eq!(typed_text.len(), original_text.len());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?; // Enter alternate screen for better display
    terminal::enable_raw_mode()?; // Enable raw mode for character-by-character input

    let words_count = 1;
    let sentence = generate_sentence(words_count);
    let mut typed_string = String::new();
    let mut start_time: Option<Instant> = None;

    loop {
        print!("\x1B[2J\x1B[H"); // Clear the console

        // Display WPM
        if let Some(start) = start_time {
            let elapsed = start.elapsed();
            let minutes = elapsed.as_secs_f64() / 60.0;
            let wpm = if minutes > 0.0 {
                (typed_string.len() as f64 / 5.0) / minutes
            } else {
                0.0
            };
            println!("WPM: {:.2}", wpm);
        } else {
            println!("WPM: 0.00");
        }

        // Display the sentence with markers
        for (i, c) in sentence.chars().enumerate() {
            if i < typed_string.len() {
                if typed_string.chars().nth(i) == Some(c) {
                    print!("\x1B[32m{}\x1B[0m", c); // Green
                } else {
                    print!("\x1B[31m{}\x1B[0m", c); // Red
                }
            } else {
                print!("{}", c);
            }
        }
        println!();

        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Esc | KeyCode::Enter => break, // Exit on Esc or Enter
                KeyCode::Backspace => {
                    typed_string.pop(); // Handle backspace
                }
                KeyCode::Char(c) => {
                    if start_time.is_none() {
                        start_time = Some(Instant::now());
                    }
                    typed_string.push(c);
                    if typed_string.len() > sentence.len() {
                        typed_string.pop(); // prevent overflow
                    }
                    if typed_string == sentence || typed_string.len() == sentence.len() {
                        break; // Game finished
                    }
                }
                _ => {} // Ignore other keys
            }
        }
    }

    let elapsed = start_time.unwrap().elapsed();
    let minutes = elapsed.as_secs_f64() / 60.0;
    let wpm = (typed_string.len() as f64 / 5.0) / minutes;

    println!("Game Finished! Final WPM: {:.2}", wpm);
    println!("original text: {}", sentence);
    println!("typed text: {}", typed_string);
    println!("Exiting in {}sec", 5);
    io::stdout().flush().unwrap();
    std::thread::sleep(Duration::from_secs(5));

    terminal::disable_raw_mode()?; // Disable raw mode
    stdout.execute(LeaveAlternateScreen)?; // Leave alternate screen
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::generate_sentence;
    use crossterm::event::{Event, KeyCode};
    use random_word::Lang;
    use std::io::Write;
    use std::time::Duration;
    use std::{io, thread};

    #[test]
    fn test_sentence_generation_word_count() {
        assert_eq!(generate_sentence(0).len(), 0);
        assert_eq!(generate_sentence(20).split(' ').count(), 20);
        assert_eq!(generate_sentence(100).split(' ').count(), 100);
    }

    #[test]
    fn test_sentence_generation_valid_characters() {
        for c in generate_sentence(100).chars() {
            assert!(c.is_ascii_alphabetic() || c == ' ');
        }
    }

    #[test]
    fn test_word_generation_upto_word_length_10() {
        for word_length in 2..=10 {
            assert!(random_word::gen_len(word_length, Lang::En).is_some());
        }
    }
}
