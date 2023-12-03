use std::error::Error;

use crate::advent::*;

pub fn update(advent: &mut Advent) -> Result<(), Box<dyn Error>> {
    let input = include_str!("./input/puzzle.txt");
    let output = solution(input)?;

    let day = Day::Day01; // update this with the correct day

    let _ = advent.solve(day, Part::Part1, Some(output));

    Ok(())
}

fn solution(input: &str) -> Result<String, Box<dyn Error>> {
    todo!() // your solution goes here. good luck! :)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        let input = include_str!("./input/test_1.txt");
        let output = solution(input).unwrap();

        let expected_answer = ""; // update this with the expected answer from the example

        assert_eq!(output, expected_answer);
    }
}
