use std::{
    cell::{Cell, RefCell},
    fs,
    str::Lines,
};

const FILE_NAME: &str = "data1.txt";
const PART_ONE_ROUNDS: u128 = 20;
const PART_ONE_WORRY_MODIFIER_VALUE: u128 = 3;
const PART_TWO_ROUNDS: u128 = 10000;
const PART_TWO_WORRY_MODIFIER_VALUE: u128 = 9699690;

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    let mut monkeys = parse_monkeys(&data);
    let part_one = solve(&mut monkeys, PART_ONE_ROUNDS, WorryModifier::PartOne);
    println!("part one: {}", part_one);

    let mut monkeys = parse_monkeys(&data);
    let part_two = solve(&mut monkeys, PART_TWO_ROUNDS, WorryModifier::PartTwo);
    println!("part two: {}", part_two);
}

fn solve(monkeys: &mut Vec<Monkey>, rounds: u128, worry_modifier: WorryModifier) -> usize {
    for _ in 0..rounds {
        for monkey in monkeys.iter() {
            for item in monkey.items.borrow().iter() {
                let operation_value = match monkey.operation_value {
                    OperationValue::Old => *item,
                    OperationValue::Number(number) => number as u128,
                };

                let worry_level = match monkey.operation {
                    Operation::Add => item + operation_value,
                    Operation::Multiply => item * operation_value,
                };

                let worry_level = modify_worry_level(worry_level, &worry_modifier);

                match (worry_level % monkey.test.divisor) == 0 {
                    true => monkeys[monkey.test.true_monkey_id.0 as usize]
                        .items
                        .borrow_mut()
                        .push(worry_level),
                    false => monkeys[monkey.test.false_monkey_id.0 as usize]
                        .items
                        .borrow_mut()
                        .push(worry_level),
                }

                monkey
                    .inspection_count
                    .set(monkey.inspection_count.get() + 1);
            }
            monkey.items.borrow_mut().clear();
        }
    }

    let mut inspection_counts = vec![];
    for monkey in monkeys {
        inspection_counts.push(monkey.inspection_count.get())
    }
    inspection_counts.sort();
    inspection_counts.reverse();
    inspection_counts[0] * inspection_counts[1]
}

enum WorryModifier {
    PartOne,
    PartTwo,
}

fn modify_worry_level(worry: u128, worry_modifier: &WorryModifier) -> u128 {
    match worry_modifier {
        WorryModifier::PartOne => worry / PART_ONE_WORRY_MODIFIER_VALUE,
        WorryModifier::PartTwo => worry % PART_TWO_WORRY_MODIFIER_VALUE,
    }
}

fn parse_monkeys(data: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let mut iter = data.lines();

    while let Some(monkey) = parse_monkey(&mut iter) {
        monkeys.push(monkey);
        iter.next(); // skip unneeded line
    }

    monkeys
}

fn parse_monkey(lines: &mut Lines) -> Option<Monkey> {
    lines.next()?; // skip unneeded line

    // parse items
    let line = lines.next()?;
    let line = &line[18..]; // TODO make number a constant
    let tokens: Vec<&str> = line.split(", ").collect();
    let items: Vec<u128> = tokens.iter().map(|t| t.parse::<u128>().unwrap()).collect();
    let items = RefCell::new(items);

    // parse operation and operation value
    let line = lines.next()?;
    let tokens: Vec<&str> = line.split(' ').collect();
    let operation = Operation::new(tokens[tokens.len() - 2].to_string()); // TODO
    let operation_value = tokens[tokens.len() - 1]; // TODO
    let operation_value = match operation_value.parse::<usize>() {
        Ok(number) => OperationValue::Number(number),
        Err(_) => OperationValue::Old,
    };

    // parse test
    let line = lines.next()?;
    let tokens: Vec<&str> = line.split(' ').collect();
    let divisor = tokens[tokens.len() - 1].parse::<u128>().unwrap(); // TODO

    let line = lines.next()?;
    let tokens: Vec<&str> = line.split(' ').collect();
    let true_monkey_id = tokens[tokens.len() - 1].parse::<u128>().unwrap();
    let true_monkey_id = MonkeyID::new(true_monkey_id);

    let line = lines.next()?;
    let tokens: Vec<&str> = line.split(' ').collect();
    let false_monkey_id = tokens[tokens.len() - 1].parse::<u128>().unwrap();
    let false_monkey_id = MonkeyID::new(false_monkey_id);

    Some(Monkey::new(
        items,
        operation,
        operation_value,
        Test::new(divisor, true_monkey_id, false_monkey_id),
    ))
}

struct Monkey {
    items: RefCell<Vec<u128>>,
    operation: Operation,
    operation_value: OperationValue,
    test: Test,
    inspection_count: Cell<usize>,
}

impl Monkey {
    fn new(
        items: RefCell<Vec<u128>>,
        operation: Operation,
        operation_value: OperationValue,
        test: Test,
    ) -> Self {
        Monkey {
            items,
            operation,
            operation_value,
            test,
            inspection_count: Cell::new(0),
        }
    }
}

struct MonkeyID(u128);

impl MonkeyID {
    fn new(id: u128) -> Self {
        MonkeyID(id)
    }
}

struct Test {
    divisor: u128,
    true_monkey_id: MonkeyID,
    false_monkey_id: MonkeyID,
}

impl Test {
    fn new(divisor: u128, true_monkey_id: MonkeyID, false_monkey_id: MonkeyID) -> Self {
        Test {
            divisor,
            true_monkey_id,
            false_monkey_id,
        }
    }
}

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn new(string: String) -> Self {
        if string == "*" {
            Operation::Multiply
        } else if string == "+" {
            Operation::Add
        } else {
            panic!("Can only parse * and +.");
        }
    }
}

enum OperationValue {
    Old,
    Number(usize),
}
