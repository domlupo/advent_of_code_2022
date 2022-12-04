use std::fs;

const FILE_NAME: &str = "data1.txt";

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    let mut total = 0;
    let mut max_total = 0;

    for line in data.lines() {
        if total > max_total {
            max_total = total;
        }

        if line.is_empty() {
            total = 0;
        } else {
            total += line.parse::<i32>().unwrap();
        }
    }

    println!("Part one: {}", max_total);
}

fn part_two(data: &str) {
    let mut total = 0;
    let mut max_totals = vec![];

    for line in data.lines() {
        if line.is_empty() {
            max_totals.push(total);
            max_totals.sort_by(|a, b| a.cmp(b).reverse());
            max_totals.truncate(3);
            total = 0;
        } else {
            total += line.parse::<i32>().unwrap();
        }
    }

    println!("Part two: {}", max_totals.iter().sum::<i32>());
}
