use std::{fs, io};

const INPUT_PATHS: &str = "../aoc-inputs/2019";

pub fn read_input(day: u32) -> Result<Vec<String>, io::Error> {
    Ok(
        fs::read_to_string(INPUT_PATHS.to_string() + &format!("/day{:02}", day as u32))?
            .lines()
            .map(|val| String::from(val))
            .collect(),
    )
}
