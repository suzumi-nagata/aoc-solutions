use std::{collections::HashMap, str::FromStr};

use common::{read_example_input, read_input};
use Direction::{Down, Left, Right, Up};

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input(3).unwrap();
    // let input = read_example_input(3, 2).unwrap();

    let (path_a, path_b) = parse_input(input);

    let mut filled_positions_map: HashMap<Coord, u8> = HashMap::new();

    let mut initial_pos = Coord { x: 0, y: 0 };
    for instruction in path_a {
        instruction.fill_positions_map(
            &mut filled_positions_map,
            &mut initial_pos,
            SpaceType::WireA,
        );
    }

    let mut initial_pos = Coord { x: 0, y: 0 };
    for instruction in path_b {
        instruction.fill_positions_map(
            &mut filled_positions_map,
            &mut initial_pos,
            SpaceType::WireB,
        );
    }

    let mut cross_positions: Vec<Coord> = filled_positions_map
        .iter()
        .filter_map(|(key, val)| (*val == u8::from(SpaceType::Both)).then_some(*key))
        .collect();

    cross_positions.sort_by(|a, b| ((a.x).abs() + (a.y).abs()).cmp(&((b.x).abs() + (b.y).abs())));

    let cross_positions_dists: Vec<i32> = cross_positions
        .iter()
        .map(|coord| (coord.x).abs() + (coord.y).abs())
        .filter(|dist| *dist > 0)
        .collect();

    let least_dist = cross_positions_dists.first().unwrap();

    println!("Part 1: the minimum cross distance is {}", least_dist)
}

fn part2() {}

#[derive(Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Right),
            "U" => Ok(Up),
            "L" => Ok(Left),
            "D" => Ok(Down),
            _ => Err(String::from("invalid direction")),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: u32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum SpaceType {
    None = 0x00,
    WireA = 0x01,
    WireB = 0x02,
    Both = 0x03,
}

impl From<SpaceType> for u8 {
    fn from(source: SpaceType) -> Self {
        match source {
            SpaceType::None => 0x00,
            SpaceType::WireA => 0x01,
            SpaceType::WireB => 0x02,
            SpaceType::Both => 0x03,
        }
    }
}

fn parse_input(input: Vec<String>) -> (Vec<Instruction>, Vec<Instruction>) {
    let mut input = input.iter();

    let path_a = input
        .next()
        .unwrap()
        .to_owned()
        .split(",")
        .map(|inst_str| parse_instruction(inst_str))
        .collect();
    let path_b = input
        .next()
        .unwrap()
        .to_owned()
        .split(",")
        .map(|inst_str| parse_instruction(inst_str))
        .collect();

    (path_a, path_b)
}

fn parse_instruction(instruction_str: &str) -> Instruction {
    Instruction {
        direction: Direction::from_str(&instruction_str[0..1]).unwrap(),
        amount: u32::from_str_radix(&instruction_str[1..], 10).unwrap(),
    }
}

impl Instruction {
    fn fill_positions_map(
        &self,
        map: &mut HashMap<Coord, u8>,
        curr_pos: &mut Coord,
        space_type: SpaceType,
    ) {
        map.entry(*curr_pos)
            .and_modify(|value| *value |= u8::from(space_type))
            .or_insert(space_type.into());

        let amount = self.amount;

        let mut new_coord = *curr_pos;

        for _ in 1..=amount {
            match self.direction {
                Right => new_coord.x += 1 as i32,
                Up => new_coord.y += 1 as i32,
                Left => new_coord.x -= 1 as i32,
                Down => new_coord.y -= 1 as i32,
            }

            map.entry(new_coord)
                .and_modify(|value| *value |= u8::from(space_type))
                .or_insert(space_type.into());
        }

        *curr_pos = new_coord;
    }
}
