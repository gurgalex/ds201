use std::io::{self, Write};
use std::ascii::AsciiExt;

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
            Err(error) => {
                println!("Non-number entered {:?}", error);
                continue;
            }
        };

        let letter: String = letter_grade(score);
        println!("Your points {} Your grade: {}", score, letter);
        io::stdout().flush().unwrap();

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

/// Returns a letter grade given a point score out of 100
///
/// * `final_point` - A score that needs a grade
///
/// # Example
///
/// ```
/// let score = letter_grade(82.5);
/// ```

pub fn letter_grade(final_point: f32) -> String {
    if final_point >= 92.5 {
        return String::from("A");
    } else if final_point >= 82.5 {
        return String::from("B");
    } else if final_point >= 72.5 {
        return String::from("C");
    } else if final_point >= 62.5 {
        return String::from("D");
    } else {
        return String::from("F, Sorry you fail");
    }
}
