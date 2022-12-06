use std::fs;

const FILE_NAME: &str = "data1.txt";

const MOVE_QUANTITY_INDEX: usize = 1;
const MOVE_START_STACK_INDEX: usize = 3;
const MOVE_END_STACK_INDEX: usize = 5;

const CHARS_BETWEEN_CRATES: usize = 4;
const FIRST_CRATE_CHAR_INDEX: usize = 1;

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    let mut stacks = parse_stacks(data);
    let moves = parse_moves(data);
    process_moves(&moves, &mut stacks, CrateMover::First);
    let message = get_message(&stacks);
    println!("Part one: {}", message);
}

fn part_two(data: &str) {
    let mut stacks = parse_stacks(data);
    let moves = parse_moves(data);
    process_moves(&moves, &mut stacks, CrateMover::Second);
    let message = get_message(&stacks);
    println!("Part two: {}", message);
}

enum CrateMover {
    First,
    Second,
}

/// Processes the passed moves by moving crates between the passed stacks.
/// The method to move crates depends on CrateMover. CrateMover::First picks
/// up a single crate at a time which reverses the crate order on during moves
/// where as CrateMover::Second picks up all crates at once which retains the
/// crate order during moves.
fn process_moves(moves: &[Move], stacks: &mut [Stack], crate_mover: CrateMover) {
    moves.iter().for_each(|m| {
        let start_stack = &mut stacks[m.start_stack - 1].crates;
        let crates_to_move = (start_stack.len() - m.quantity)..=(start_stack.len() - 1);

        let moved_crates = match crate_mover {
            // TODO is initializing another String wasteful?
            CrateMover::First => start_stack.drain(crates_to_move).rev().collect::<String>(),
            CrateMover::Second => start_stack.drain(crates_to_move).collect::<String>(),
        };

        let end_stack = &mut stacks[m.end_stack - 1].crates;
        end_stack.push_str(moved_crates.as_str());
    });
}

#[derive(Clone, Default)] // TODO is this clone removeable?
struct Stack {
    crates: String,
}

struct Move {
    quantity: usize,
    start_stack: usize,
    end_stack: usize,
}

fn parse_moves(data: &str) -> Vec<Move> {
    let mut moves = vec![];

    let mut parse_moves = false;
    for line in data.lines() {
        if parse_moves {
            moves.push(parse_move(line));
        }

        if line.is_empty() {
            parse_moves = true;
        }
    }

    moves
}

fn parse_move(input: &str) -> Move {
    let tokens: Vec<&str> = input.split(' ').collect();

    Move {
        quantity: tokens[MOVE_QUANTITY_INDEX].parse::<usize>().unwrap(),
        start_stack: tokens[MOVE_START_STACK_INDEX].parse::<usize>().unwrap(),
        end_stack: tokens[MOVE_END_STACK_INDEX].parse::<usize>().unwrap(),
    }
}

fn parse_stacks(data: &str) -> Vec<Stack> {
    let mut stacks = vec![];

    let mut parse_stacks = false;
    for line in data.lines().rev() {
        if parse_stacks && stacks.is_empty() {
            stacks = parse_stack(line);
        } else if parse_stacks {
            parse_crate(line, &mut stacks);
        }

        if line.is_empty() {
            parse_stacks = true;
        }
    }

    stacks
}

fn parse_stack(input: &str) -> Vec<Stack> {
    let tokens_count = input.split(' ').filter(|&t| !t.is_empty()).count();
    return vec![Stack::default(); tokens_count];
}

fn parse_crate(input: &str, stacks: &mut [Stack]) {
    for (i, char) in input.chars().enumerate() {
        if i % CHARS_BETWEEN_CRATES == FIRST_CRATE_CHAR_INDEX && !char.eq(&' ') {
            stacks[(i - 1) / CHARS_BETWEEN_CRATES].crates.push(char);
        }
    }
}

/// The message is the crate character of the crate on top of every stack
/// from left to right. For example, a left stack with a top crate A, middle stack
/// with top crate B and right stack with top crate C, would have message "ABC"
fn get_message(stacks: &[Stack]) -> String {
    stacks
        .iter()
        .map(|stack| stack.crates.as_bytes()[stack.crates.len() - 1] as char)
        .collect()
}
