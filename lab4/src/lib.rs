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
        String::from("A")
    } else if final_point >= 82.5 {
        String::from("B")
    } else if final_point >= 72.5 {
        String::from("C")
    } else if final_point >= 62.5 {
        String::from("D")
    } else {
        String::from("F, Sorry you fail")
    }
}
