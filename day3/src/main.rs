use std::{collections::HashSet, fs};

const FILE_NAME: &str = "data1.txt";

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    part_one(&data);
}

fn part_one(data: &str) {
    let mut total = 0;

    data.lines().for_each(|line| {
        let chars = line.chars().collect::<Vec<_>>();
        let (first_items, second_items) = chars.split_at(line.len() / 2);

        let first_items: HashSet<&char> = HashSet::from_iter(first_items.iter());
        let second_items: HashSet<&char> = HashSet::from_iter(second_items.iter());

        let intersect = first_items.intersection(&second_items);
        let duplicate_char = **intersect.collect::<Vec<_>>()[0];

        total += get_priority(duplicate_char);
    });

    println!("Part one: {}", total);
}

fn get_priority(c: char) -> u32 {
    match c.is_lowercase() {
        true => c as u32 - 96,
        false => c as u32 - 38,
    }
}
