use std::{fs, io, path::PathBuf};

const INPUT_PATHS: &str = "../../aoc-inputs/2019";

pub fn read_input(day: u32) -> Result<Vec<String>, io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path.push(INPUT_PATHS.to_string() + &format!("/day{:02}", day));
    println!("reading input {:?}", path);

    Ok(fs::read_to_string(path)?
        .lines()
        .map(|val| String::from(val))
        .collect())
}

pub fn read_example_input(day: u32, example_num: u32) -> Result<Vec<String>, io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path.push(INPUT_PATHS.to_string() + &format!("/day{:02}-ex{:02}.in", day, example_num));
    println!("reading example input {:?}", path);

    Ok(fs::read_to_string(path)?
        .lines()
        .map(|val| String::from(val))
        .collect())
}

pub fn read_example_output(day: u32, example_num: u32) -> Result<Vec<String>, io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path.push(INPUT_PATHS.to_string() + &format!("/day{:02}-ex{:02}.out", day, example_num));
    println!("reading example output {:?}", path);

    Ok(fs::read_to_string(path)?
        .lines()
        .map(|val| String::from(val))
        .collect())
}
