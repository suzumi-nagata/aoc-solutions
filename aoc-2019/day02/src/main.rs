use common::read_input;
use OpCode::{Add, Halt, Multiply};

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input(2).unwrap();
    let mut pc = 0;

    let mut program: Vec<i32> = input
        .first()
        .unwrap()
        .split(",")
        .map(|value| i32::from_str_radix(value, 10).unwrap())
        .collect();

    program[1] = 12;
    program[2] = 2;

    let val_pos_0 = exec_program(&mut program, &mut pc);

    println!("Part 1: final program value at pos 0 {}", val_pos_0)
}

fn part2() {
    let target = 19690720;
    let mut result = None;

    'outer_loop: for noun in 0..=99 {
        for verb in 0..=99 {
            result = try_inputs(noun, verb, target);
            if result.is_some() {
                break 'outer_loop;
            }
        }
    }

    println!("Part 2: 100 * noun + verb: {}", result.unwrap())
}

fn try_inputs(noun: i32, verb: i32, target: i32) -> Option<i32> {
    let input = read_input(2).unwrap();
    let mut pc = 0;

    let mut program: Vec<i32> = input
        .first()
        .unwrap()
        .split(",")
        .map(|value| i32::from_str_radix(value, 10).unwrap())
        .collect();

    program[1] = noun;
    program[2] = verb;

    let val_pos_0 = exec_program(&mut program, &mut pc);

    (val_pos_0 == target as u32).then_some(100 * noun + verb)
}

fn exec_program(program: &mut Vec<i32>, pc: &mut u32) -> u32 {
    let mut halt = false;

    while !halt {
        // println!("PC {:?} - {:?}", pc, program);
        let opcode: OpCode = program
            .get(*pc as usize)
            .map(|val| (*val).try_into().unwrap())
            .unwrap();
        halt = opcode.exec(program, pc);
    }

    program[0] as u32
}

enum OpCode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

impl TryFrom<i32> for OpCode {
    type Error = String;

    fn try_from(source: i32) -> Result<Self, Self::Error> {
        match source {
            1 => Ok(Add),
            2 => Ok(Multiply),
            99 => Ok(Halt),
            _ => Err(String::from("Invalid OpCode")),
        }
    }
}

impl OpCode {
    fn exec(&self, program: &mut Vec<i32>, pc: &mut u32) -> bool {
        match self {
            OpCode::Add => {
                let in1: usize = program
                    .get((*pc + 1) as usize)
                    .map(|val| *val as usize)
                    .unwrap();
                let in2 = program
                    .get((*pc + 2) as usize)
                    .map(|val| *val as usize)
                    .unwrap();
                let out3 = program
                    .get((*pc + 3) as usize)
                    .map(|val| *val as usize)
                    .unwrap();
                program[out3] = program[in1] + program[in2];
                *pc = *pc + 4;

                false
            }

            OpCode::Multiply => {
                let in1: usize = program
                    .get((*pc + 1) as usize)
                    .map(|val| *val as usize)
                    .unwrap();
                let in2 = program
                    .get((*pc + 2) as usize)
                    .map(|val| *val as usize)
                    .unwrap();
                let out3 = program
                    .get((*pc + 3) as usize)
                    .map(|val| *val as usize)
                    .unwrap();

                program[out3] = program[in1] * program[in2];
                *pc = *pc + 4;

                false
            }

            OpCode::Halt => true,
        }
    }
}

#[test]
fn test_examples() {
    use common::{read_example_input, read_example_output};

    let ex_nums = [1, 2, 3, 4, 5];

    for ex in ex_nums {
        println!("running example: {ex}");

        let input = read_example_input(2, ex).unwrap();
        let mut pc = 0;

        let mut program: Vec<i32> = input
            .first()
            .unwrap()
            .split(",")
            .map(|value| i32::from_str_radix(value, 10).unwrap())
            .collect();

        exec_program(&mut program, &mut pc);

        let output = read_example_output(2, ex).unwrap();

        let output_program: Vec<i32> = output
            .first()
            .unwrap()
            .split(",")
            .map(|value| i32::from_str_radix(value, 10).unwrap())
            .collect();

        assert_eq!(program, output_program)
    }
}
