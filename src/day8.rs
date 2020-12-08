use std::collections::{HashSet};

pub fn solve_floor() {
    let data = parse_data();
    let (acc, _terminated) = run_program(&data);

    println!("{}", acc);
}

pub fn solve_basement() {
    let data = parse_data();
    let possible_bad_lines: Vec<usize> = data
        .iter()
        .enumerate()
        .filter(|(_, (arg, _))| *arg == "jmp" || *arg == "nop")
        .map(|(i, (_, _))| i)
        .collect();

    for i in possible_bad_lines {
        let mut fixed_program = data.clone();
        fixed_program[i] = match fixed_program[i].0 {
            "jmp" => ("nop", fixed_program[i].1),
            "nop" => ("jmp", fixed_program[i].1),
            _ => panic!("None of the other instructions have problems!")
        };

        let (acc, terminated) = run_program(&fixed_program);
        if terminated {
            println!("{}", acc);
            break;
        }
    }
}

// Returns the accumulator and whether or not the program terminated successfully.
fn run_program(data: &Vec<(&str, isize)>) -> (isize, bool) {
    let mut acc = 0;
    let mut instruction_pointer: isize = 0;
    let mut lines_read: HashSet<isize> = HashSet::new();

    while !lines_read.contains(&instruction_pointer) && (instruction_pointer as usize) < data.len() {
        lines_read.insert(instruction_pointer);
        let (addition, jump_pointer) = run_instruction(data[instruction_pointer as usize]);
        acc += addition;
        if jump_pointer != 0 {
            instruction_pointer += jump_pointer;
            assert!(instruction_pointer >= 0, "Instruction pointer went negative!");
        } else {
            instruction_pointer += 1;
        }
    }
    (acc, instruction_pointer as usize >= data.len())
}

fn run_instruction((instruction, arg): (&str, isize)) -> (isize, isize) {
    match instruction {
        "acc" => (arg, 0),
        "jmp" => (0, arg),
        "nop" => (0, 0),
        _ => (0, 0)
    }
}

fn parse_data() -> Vec<(&'static str, isize)> {
    let data: Vec<(&str, isize)> = include_str!("../resources/8-1.txt")
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let cmd = split.next();
            let arg = split.next().unwrap();
            (cmd.unwrap(), arg.parse().unwrap())
        })
        .collect();
    data
}
