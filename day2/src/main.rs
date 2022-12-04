use std::fs;

const FILE_NAME: &str = "data1.txt";
const SHAPE_INDEX: usize = 2;

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    let mut total_score = 0;

    for line in data.lines() {
        let opponent_shape = line.chars().next().unwrap();
        let shape = line.chars().nth(SHAPE_INDEX).unwrap();

        let opponent_shape = Shape::from_char_to_shape(opponent_shape);
        let shape = Shape::from_char_to_shape(shape);

        let outcome = get_outcome(&shape, &opponent_shape);
        total_score += get_outcome_score(&outcome);
        total_score += get_shape_score(&shape);
    }

    println!("Part one: {}", total_score);
}

fn part_two(data: &str) {
    let mut total_score = 0;

    for line in data.lines() {
        let opponent_shape = line.chars().next().unwrap();
        let shape = line.chars().nth(SHAPE_INDEX).unwrap();

        let opponent_shape = Shape::from_char_to_shape(opponent_shape);
        let outcome = Shape::from_char_to_outcome(shape);

        let shape = get_shape(&opponent_shape, &outcome);
        total_score += get_outcome_score(&outcome);
        total_score += get_shape_score(&shape);
    }

    println!("Part two: {}", total_score);
}

enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn from_char_to_shape(value: char) -> Shape {
        match value {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissor,
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissor,
            _ => panic!("Unknown value: {}", value),
        }
    }

    fn from_char_to_outcome(value: char) -> Outcome {
        match value {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

fn get_outcome(shape: &Shape, opponent_shape: &Shape) -> Outcome {
    let shapes = (shape, opponent_shape);
    match shapes {
        (Shape::Rock, Shape::Rock) => Outcome::Draw,
        (Shape::Rock, Shape::Paper) => Outcome::Lose,
        (Shape::Rock, Shape::Scissor) => Outcome::Win,
        (Shape::Paper, Shape::Rock) => Outcome::Win,
        (Shape::Paper, Shape::Paper) => Outcome::Draw,
        (Shape::Paper, Shape::Scissor) => Outcome::Lose,
        (Shape::Scissor, Shape::Rock) => Outcome::Lose,
        (Shape::Scissor, Shape::Paper) => Outcome::Win,
        (Shape::Scissor, Shape::Scissor) => Outcome::Draw,
    }
}

fn get_shape(opponent_shape: &Shape, outcome: &Outcome) -> Shape {
    let scenarios = (opponent_shape, outcome);
    match scenarios {
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Rock, Outcome::Draw) => Shape::Rock,
        (Shape::Rock, Outcome::Lose) => Shape::Scissor,
        (Shape::Paper, Outcome::Win) => Shape::Scissor,
        (Shape::Paper, Outcome::Draw) => Shape::Paper,
        (Shape::Paper, Outcome::Lose) => Shape::Rock,
        (Shape::Scissor, Outcome::Win) => Shape::Rock,
        (Shape::Scissor, Outcome::Draw) => Shape::Scissor,
        (Shape::Scissor, Outcome::Lose) => Shape::Paper,
    }
}

fn get_shape_score(shape: &Shape) -> usize {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    }
}

fn get_outcome_score(outcome: &Outcome) -> usize {
    match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    }
}
