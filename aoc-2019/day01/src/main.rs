use common::read_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input(1).unwrap();

    let mut fuel_req = 0;
    for line in input {
        let mass = u32::from_str_radix(&line, 10).unwrap();
        fuel_req += mass / 3 - 2
    }

    println!("Part 1: fuel required {}", fuel_req)
}

fn part2() {
    let input = read_input(1).unwrap();

    let mut fuel_req = 0;
    for line in input {
        let mass = u32::from_str_radix(&line, 10).unwrap();
        fuel_req += recursive_fuel_calculation(mass as i32) as u32;
    }

    println!("Part 2: fuel required {}", fuel_req)
}

fn recursive_fuel_calculation(initial_fuel: i32) -> i32 {
    let curr_fuel_req = initial_fuel / 3 - 2;

    if curr_fuel_req <= 0 {
        return 0;
    }

    curr_fuel_req + recursive_fuel_calculation(curr_fuel_req)
}

#[test]
fn test_additional_fuel_14() {
    assert_eq!(recursive_fuel_calculation(14), 2)
}

#[test]
fn test_additional_fuel_1969() {
    assert_eq!(recursive_fuel_calculation(1969), 966)
}

#[test]
fn test_additional_fuel_100756() {
    assert_eq!(recursive_fuel_calculation(100756), 50346)
}
