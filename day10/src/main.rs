use std::{collections::HashSet, fs};

const FILE_NAME: &str = "data1.txt";
const ADDX_TOKEN: &str = "addx";
const NOOP_TOKEN: &str = "noop";
const SUM_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];

// part two
const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;
const LIT_PIXEL: &str = "#";
const DARK_PIXEL: &str = ".";

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    let sum_cycles: HashSet<i32> = SUM_CYCLES.into_iter().collect();
    let mut sum = 0;
    let mut cycle = 1;
    let mut x = 1;

    for line in data.lines() {
        let input = parse_line(line);

        match input {
            Input::Addx(value) => {
                cycle += 1;

                if sum_cycles.contains(&cycle) {
                    sum += x * cycle;
                }

                cycle += 1;
                x += value;
            }
            Input::Noop => cycle += 1,
        }

        if sum_cycles.contains(&cycle) {
            sum += x * cycle;
        }
    }

    println!("Part one: {}", sum);
}

fn part_two(data: &str) {
    let mut crt = String::new();
    let mut cycle = 0;
    let mut sprite_position = 1;

    for line in data.lines() {
        let input = parse_line(line);

        match input {
            Input::Addx(value) => {
                cycle += 1;
                crt.push_str(get_pixel(sprite_position, cycle - 1));
                cycle += 1;
                crt.push_str(get_pixel(sprite_position, cycle - 1));
                sprite_position += value;
            }
            Input::Noop => {
                cycle += 1;
                crt.push_str(get_pixel(sprite_position, cycle - 1));
            }
        }
    }
    println!();
    println!("part two:");
    display_crt(crt);
}

fn display_crt(crt: String) {
    let mut left_index = 0;
    let mut right_index = CRT_WIDTH - 1;

    for _ in 0..CRT_HEIGHT {
        println!("{}", &crt[left_index..=right_index]);
        left_index += CRT_WIDTH;
        right_index += CRT_WIDTH;
    }
}

fn get_pixel(sprite_position: i32, pixel_position: i32) -> &'static str {
    let pixel_position = pixel_position % (CRT_WIDTH as i32);
    if sprite_position == pixel_position
        || sprite_position + 1 == pixel_position
        || sprite_position - 1 == pixel_position
    {
        LIT_PIXEL
    } else {
        DARK_PIXEL
    }
}

fn parse_line(line: &str) -> Input {
    let tokens: Vec<&str> = line.split(' ').collect();

    if tokens[0] == ADDX_TOKEN {
        Input::Addx(tokens[1].parse::<i32>().unwrap())
    } else if tokens[0] == NOOP_TOKEN {
        Input::Noop
    } else {
        panic!("Can only parse addx and noop inputs")
    }
}

enum Input {
    Addx(i32),
    Noop,
}
