#![allow(clippy::items_after_test_module)] // you can ignore this

use anyhow::{Error, Result};
use aoc_23_with_rust::*; // you can ignore this too // and also this

/// Update this with the current day
const CURRENT_DAY: Day = Day::Day01;

/// The function is expecting you to return a `Result<String, anyhow::Error>`.
/// This means that you can return an `Ok(String)` or an `Err(anyhow::Error)`.
/// The easiest way to achieve this is to simply wrap your solution in an `Ok`:
/// ```
/// fn answer_to_ultimate_question() -> Result<String, Error> {
///     let result = "42".to_string();
///     Ok(result)
/// }
/// ```
/// and use `?` to propagate any potential errors:
/// ```
/// fn answer_to_ultimate_question() -> Result<i32, Error> {
///     let result = "42".parse::<i32>()?;
///     Ok(result)
/// }
/// ```
///
/// Please don't change the function name nor the signature.
fn solve_part_1(input: &str) -> Result<String, Error> {
    // your solution to part 1 goes here.
    good_luck!(input)
}

/// The function is expecting you to return a `Result<String, anyhow::Error>`.
/// This means that you can return an `Ok(String)` or an `Err(anyhow::Error)`.
/// The easiest way to achieve this is to simply wrap your solution in an `Ok`:
/// ```
/// fn answer_to_ultimate_question() -> Result<String, Error> {
///     let result = "42".to_string();
///     Ok(result)
/// }
/// ```
/// and use `?` to propagate any potential errors:
/// ```
/// fn answer_to_ultimate_question() -> Result<i32, Error> {
///     let result = "42".parse::<i32>()?;
///     Ok(result)
/// }
/// ```
///
/// Please don't change the function name nor the signature.
fn solve_part_2(input: &str) -> Result<String, Error> {
    // your solution to part 2 goes here.
    good_luck!(input)
}

/// Optional: use this to test your solutions.
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_part_1_solution() {
        const EXAMPLE_1: &str = r#"
            example input part 1 goes here
            note: you can use indented
            multiline strings just like this.
            or you can just simply copy-paste.
            do as you wish,
            the elves will handle everything!
        "#;

        const EXPECTED_ANSWER_1: &str = ""; // update this with the expected answer

        // that's it! run `cargo test --bin <day>` and let the elves do the rest

        elf_test_this!(EXAMPLE_1, solve_part_1, EXPECTED_ANSWER_1);
    }

    #[test]
    fn check_part_2_solution() {
        const EXAMPLE_2: &str = r#"
            example input part 2 goes here
            note: you can use indented
            multiline strings just like this.
            or you can just simply copy-paste.
            do as you wish,
            the elves will handle everything!
        "#;

        const EXPECTED_ANSWER_2: &str = ""; // update this with the expected answer

        // that's it! run `cargo test --bin <day>` and let the elves do the rest

        elf_test_this!(EXAMPLE_2, solve_part_2, EXPECTED_ANSWER_2);
    }
}

/// You don't need to do anything in the main function.
/// Simply run `cargo run --bin <day>` and check the answer inside `solution.yaml`
fn main() -> Result<(), Error> {
    let mut advent = Advent::ho_ho_ho()?;

    advent.get_package(elf_magic!())?;

    Ok(())
}

// Puzzle Input
const PUZZLE_INPUT: &str = r#"
    puzzle input goes here
    note: you can use indented
    multiline strings just like this.
    or you can just simply copy-paste.
    do as you wish,
    the elves will handle everything!
"#;
