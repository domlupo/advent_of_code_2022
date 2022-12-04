use std::{collections::HashSet, fs};

const FILE_NAME: &str = "data1.txt";

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    let mut total = 0;

    data.lines().for_each(|line| {
        let ranges = line.split(',').collect::<Vec<&str>>();
        let first_range = parse_range(ranges[0]);
        let second_range = parse_range(ranges[1]);

        if first_range.is_subset(&second_range) || second_range.is_subset(&first_range) {
            total += 1;
        }
    });

    println!("Part one: {}", total);
}

fn part_two(data: &str) {
    let mut total = 0;

    data.lines().for_each(|line| {
        let ranges = line.split(',').collect::<Vec<&str>>();
        let first_range = parse_range(ranges[0]);
        let second_range = parse_range(ranges[1]);

        if !first_range.is_disjoint(&second_range) {
            total += 1;
        }
    });

    println!("Part two: {}", total);
}

fn parse_range(range: &str) -> HashSet<i32> {
    let bounds = range.split('-').collect::<Vec<&str>>();
    let start: i32 = bounds[0].parse().unwrap();
    let end: i32 = bounds[1].parse().unwrap();
    HashSet::from_iter(start..=end)
}
