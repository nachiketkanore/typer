#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::Rng;
use std::time::{Duration, Instant};
use std::{
    io::{self, Write},
    thread::sleep,
};

fn generate_sentence(length: usize) -> String {
    let mut rng = rand::rng();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz ".chars().collect();
    (0..length)
        .map(|_| chars[rng.random_range(0..chars.len())])
        .collect()
}

fn print_stats(typed_text: String, original_text: String) {
    assert_eq!(typed_text.len(), original_text.len());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?; // Enter alternate screen for better display
    terminal::enable_raw_mode()?; // Enable raw mode for character-by-character input

    let sentence_length = 50;
    let sentence = generate_sentence(sentence_length);
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

    println!("\nGame Over!");
    println!("Final WPM: {:.2}", wpm);

    sleep(Duration::from_secs(5));
    terminal::disable_raw_mode()?; // Disable raw mode
    stdout.execute(LeaveAlternateScreen)?; // Leave alternate screen
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::generate_sentence;
    use crossterm::event::{Event, KeyCode};
    use std::io::Write;
    use std::time::Duration;
    use std::{io, thread};

    #[test]
    fn test_sentence_generation() {
        let sentence = generate_sentence(20);
        assert_eq!(sentence.len(), 20);
        for c in sentence.chars() {
            assert!(c.is_ascii_alphabetic() || c == ' '); // Check for valid characters
        }

        let sentence2 = generate_sentence(0);
        assert_eq!(sentence2.len(), 0);

        let sentence3 = generate_sentence(100);
        assert_eq!(sentence3.len(), 100);
        for c in sentence3.chars() {
            assert!(c.is_ascii_alphabetic() || c == ' '); // Check for valid characters
        }
    }

    #[test]
    fn test_backspace() {
        let mut typed_string = String::from("hello");
        typed_string.pop();
        assert_eq!(typed_string, "hell");

        typed_string.pop();
        typed_string.pop();
        assert_eq!(typed_string, "he");
    }
}
