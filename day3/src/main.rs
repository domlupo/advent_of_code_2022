use std::{
    collections::{HashMap, HashSet},
    fs,
};

const FILE_NAME: &str = "data1.txt";
const PART_TWO_SACK_COUNT: usize = 2;

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    part_one(&data);
    part_two(&data);
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

fn part_two(data: &str) {
    let mut total = 0;
    let mut first_sack: HashSet<char> = HashSet::new();
    let mut second_sack: HashSet<char> = HashSet::new();
    let mut third_sack: HashSet<char> = HashSet::new();
    let mut char_counts: HashMap<char, usize> = HashMap::new();

    data.lines().for_each(|line| {
        let chars = line.chars().collect::<Vec<_>>();
        if first_sack.is_empty() {
            first_sack = HashSet::from_iter(chars.into_iter());
        } else if second_sack.is_empty() {
            second_sack = HashSet::from_iter(chars.into_iter());
        } else if third_sack.is_empty() {
            third_sack = HashSet::from_iter(chars.into_iter());

            for c in &first_sack {
                count_chars(*c, &mut char_counts)
            }
            for c in &second_sack {
                count_chars(*c, &mut char_counts)
            }
            for c in &third_sack {
                count_chars(*c, &mut char_counts)
            }

            for (c, count) in &char_counts {
                if count == &PART_TWO_SACK_COUNT {
                    total += get_priority(*c);
                }
            }

            first_sack.clear();
            second_sack.clear();
            third_sack.clear();
            char_counts.clear();
        }
    });

    println!("Part two: {}", total);
}

fn count_chars(c: char, char_counts: &mut HashMap<char, usize>) {
    match char_counts.get(&c) {
        Some(count) => char_counts.insert(c, count + 1),
        None => char_counts.insert(c, 1),
    };
}

fn get_priority(c: char) -> u32 {
    match c.is_lowercase() {
        true => c as u32 - 96,
        false => c as u32 - 38,
    }
}
