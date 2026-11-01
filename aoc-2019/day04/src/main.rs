use std::slice::Windows;

use common::read_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input(4).unwrap();

    let (initial_range, final_range) = input.first().unwrap().split_once("-").unwrap();
    let initial_range = u32::from_str_radix(initial_range, 10).unwrap();
    let final_range = u32::from_str_radix(final_range, 10).unwrap();

    let mut password_count = 0;
    for password_candidate in initial_range..=final_range {
        let candidate_vec = int_to_vector(password_candidate);

        if !has_double_digit(&candidate_vec) {
            continue;
        }

        if !is_non_decreasing(&candidate_vec) {
            continue;
        }

        password_count += 1;
    }

    println!("Part 1: number of possible passwords {}", password_count)
}

fn part2() {
    let input = read_input(4).unwrap();

    let (initial_range, final_range) = input.first().unwrap().split_once("-").unwrap();
    let initial_range = u32::from_str_radix(initial_range, 10).unwrap();
    let final_range = u32::from_str_radix(final_range, 10).unwrap();

    let mut password_count = 0;
    for password_candidate in initial_range..=final_range {
        let candidate_vec = int_to_vector(password_candidate);

        if !is_non_decreasing(&candidate_vec) {
            continue;
        }

        if !has_a_single_double_digit(&candidate_vec) {
            continue;
        }

        password_count += 1;
    }

    println!("Part 2: number of possible passwords {}", password_count)
}

fn int_to_vector(candidate: u32) -> Vec<u32> {
    candidate
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn has_double_digit(candidate: &[u32]) -> bool {
    candidate.windows(2).any(|w| w[0] == w[1])
}

fn has_a_single_double_digit(candidate: &[u32]) -> bool {
    let mut dup_digit: Vec<u32> = candidate
        .windows(2)
        .filter_map(|w| (w[0] == w[1]).then_some(w[0]))
        .collect();

    dup_digit.dedup();

    // Try to find any double digit that isn't a triple digit
    dup_digit
        .iter()
        .any(|&digit| !has_this_as_a_triple_digit(candidate, digit))
}

fn has_this_as_a_triple_digit(candidate: &[u32], digit: u32) -> bool {
    candidate
        .windows(3)
        .any(|w| digit == w[0] && w[0] == w[1] && w[1] == w[2])
}

fn is_non_decreasing(candidate: &[u32]) -> bool {
    candidate.windows(2).all(|w| w[0] <= w[1])
}
