extern crate lab4;
use std::io::{self, Write};
use std::ascii::AsciiExt;

use lab4::letter_grade;

fn main() {
    loop {
        print!("What is your final score? ");
        // flush text to stdout since no newline was given.
        io::stdout().flush().unwrap();
        let mut score: String = String::new();

        io::stdin()
            .read_line(&mut score)
            .expect("Failed to read line");

        let score: f32 = match score.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Non-number entered: {:?}", score.trim());
                continue;
            }
        };

        let letter: String = letter_grade(score);
        println!("Your points {} Your grade: {}", score, letter);

        print!("Do you have more student Final points? ");
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("input could not be read!");

        response = response.trim().to_string();
        if response.eq_ignore_ascii_case("n") || response.eq_ignore_ascii_case("No")
            || response.eq_ignore_ascii_case("I, don't")
        {
            println!(
                "Grade report Complete! Your final letter grade is {}",
                letter
            );
            break;
        }
    }
}
