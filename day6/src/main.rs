use std::{collections::HashMap, fmt::Error, fs};

const FILE_NAME: &str = "data1.txt";
const FIRST_WINDOW_SIZE: usize = 4;
const SECOND_WINDOW_SIZE: usize = 14;

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    match find_marker(data, FIRST_WINDOW_SIZE) {
        Ok(index) => println!("Part one: {}", index),
        Err(_) => println!("Part one: Error"),
    }
}

fn part_two(data: &str) {
    match find_marker(data, SECOND_WINDOW_SIZE) {
        Ok(index) => println!("Part two: {}", index),
        Err(_) => println!("Part two: Error"),
    }
}

fn find_marker(data: &str, window_size: usize) -> Result<usize, Error> {
    let mut char_counts = CharCounts::default();
    let bytes = data.as_bytes();

    for i in bytes.iter().take(window_size) {
        char_counts.add(*i);
    }

    // check current window for answer or continue with
    // slide window by adding front value and removing back value
    for i in window_size..bytes.len() {
        if char_counts.0.iter().len() == window_size {
            return Ok(i);
        }

        char_counts.add(bytes[i]);
        match char_counts.remove(bytes[i - window_size]) {
            Ok(_) => (),
            Err(_) => panic!("Key does not exist"),
        }
    }

    panic!("Answer not found")
}

#[derive(Default)]
struct CharCounts(HashMap<u8, usize>);

impl CharCounts {
    /// add a new key into the HashMap with count 1
    /// or increment an existing key by 1
    fn add(&mut self, c: u8) {
        self.0
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    /// remove a key with a count of 1
    /// or decrement a count higher than 1
    fn remove(&mut self, c: u8) -> Result<(), &'static str> {
        let count = match self.0.get(&c) {
            Some(count) => count,
            None => return Err("Key does not exist"),
        };

        match *count == 1 {
            true => self.0.remove(&c),
            false => self.0.insert(c, count - 1),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::CharCounts;

    #[test]
    fn char_counts() {
        let mut char_counts = CharCounts::default();

        // test add fn
        char_counts.add(2);
        assert_eq!(*char_counts.0.get(&2).unwrap(), 1 as usize);
        char_counts.add(2);
        char_counts.add(1);
        assert_eq!(*char_counts.0.get(&1).unwrap(), 1 as usize);
        assert_eq!(*char_counts.0.get(&2).unwrap(), 2 as usize);

        // test remove fn
        char_counts.remove(2);
        assert_eq!(*char_counts.0.get(&2).unwrap(), 1 as usize);
        char_counts.remove(2);
        assert_eq!(char_counts.0.get(&2), None);
        assert_eq!(char_counts.remove(2).is_err(), true);
        char_counts.remove(1);
        assert_eq!(char_counts.0.get(&1), None);
    }
}
