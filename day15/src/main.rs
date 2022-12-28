use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs,
};

const FILE_NAME: &str = "data1.txt";

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    let (mut item_grid, nearest_beacon) = parse(&data);

    part_one(&mut item_grid, &nearest_beacon);
}

fn part_one(item_grid: &mut HashMap<Point, Item>, nearest_beacon: &HashMap<Point, Point>) {
    for (s, b) in nearest_beacon {
        let no_beacon_positions = get_no_beacon_positions(s, b);

        for point in no_beacon_positions {
            match item_grid.get(&point) {
                Some(_) => (),
                None => {
                    item_grid.insert(point, Item::NoBeacon);
                }
            }
        }
    }

    let mut no_beacon_count = 0;
    for point in item_grid.keys() {
        // TODO make CONSTANT
        if point.1 == 10 && *item_grid.get(point).unwrap() == Item::NoBeacon {
            no_beacon_count += 1;
        }
    }

    println!("Part one: {}", no_beacon_count);
}

fn parse(data: &str) -> (HashMap<Point, Item>, HashMap<Point, Point>) {
    let lines = data.lines();
    let mut item_grid = HashMap::new();
    let mut nearest_beacon = HashMap::new();

    for line in lines {
        let mut line = line.replace("Sensor at ", "");
        line = line.replace(": closest beacon is at ", ", ");
        line = line.replace("x=", "");
        line = line.replace("y=", "");

        let tokens = line.split(", ").collect::<Vec<&str>>();
        let mut tokens = tokens.iter();
        let sensor_position = Point(
            tokens.next().unwrap().parse::<i32>().unwrap(),
            tokens.next().unwrap().parse::<i32>().unwrap(),
        );
        let beacon_position = Point(
            tokens.next().unwrap().parse::<i32>().unwrap(),
            tokens.next().unwrap().parse::<i32>().unwrap(),
        );

        item_grid.insert(sensor_position.clone(), Item::Sensor);
        item_grid.insert(beacon_position.clone(), Item::Beacon);
        nearest_beacon.insert(sensor_position, beacon_position);
    }

    (item_grid, nearest_beacon)
}

#[derive(Debug, Eq, PartialEq)]
enum Item {
    Beacon,
    Sensor,
    NoBeacon,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    U,
    D,
    R,
    L,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn travel(direction: &Direction, point: &Point) -> Point {
        match direction {
            Direction::U => Point(point.0, point.1 + 1),
            Direction::D => Point(point.0, point.1 - 1),
            Direction::L => Point(point.0 - 1, point.1),
            Direction::R => Point(point.0 + 1, point.1),
        }
    }

    fn taxicab_distance(p1: &Point, p2: &Point) -> i32 {
        max(p1.0, p2.0) - min(p1.0, p2.0) + max(p1.1, p2.1) - min(p1.1, p2.1)
    }
}

/// TODO
fn get_no_beacon_positions(sensor: &Point, nearest_beacon: &Point) -> HashSet<Point> {
    let taxicab_distance = Point::taxicab_distance(sensor, nearest_beacon);
    let mut no_beacon_positions = HashSet::new();
    let mut point = sensor.clone();

    let mut right_count = 0;
    while right_count < taxicab_distance {
        right_count += 1;
        point = Point::travel(&Direction::R, &point);
    }

    point = travel_grid(
        &point,
        taxicab_distance,
        &Direction::L,
        &Direction::U,
        &mut no_beacon_positions,
    );
    point = travel_grid(
        &point,
        taxicab_distance,
        &Direction::D,
        &Direction::L,
        &mut no_beacon_positions,
    );
    point = travel_grid(
        &point,
        taxicab_distance,
        &Direction::R,
        &Direction::D,
        &mut no_beacon_positions,
    );
    travel_grid(
        &point,
        taxicab_distance,
        &Direction::U,
        &Direction::R,
        &mut no_beacon_positions,
    );

    no_beacon_positions
}

/// TOOD
fn travel_grid(
    point: &Point,
    taxicab_distance: i32,
    direction_undo: &Direction,
    new_direction: &Direction,
    no_beacon_positions: &mut HashSet<Point>,
) -> Point {
    let mut point = point.clone();
    let mut prior_travel_count = taxicab_distance;
    loop {
        prior_travel_count -= 1;
        point = Point::travel(direction_undo, &point);

        let mut new_count = 0;
        let mut new_point = point.clone();
        for _ in 0..taxicab_distance - prior_travel_count {
            new_count += 1;
            new_point = Point::travel(new_direction, &new_point);
            no_beacon_positions.insert(new_point.clone());
        }

        if new_count == taxicab_distance {
            return new_point;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;

    #[test]
    fn test_taxicab_distance() {
        assert_eq!(0, Point::taxicab_distance(&Point(0, 0), &Point(0, 0)));

        assert_eq!(1, Point::taxicab_distance(&Point(1, 0), &Point(0, 0)));
        assert_eq!(1, Point::taxicab_distance(&Point(0, 1), &Point(0, 0)));
        assert_eq!(1, Point::taxicab_distance(&Point(0, 0), &Point(1, 0)));
        assert_eq!(1, Point::taxicab_distance(&Point(0, 0), &Point(0, 1)));
        assert_eq!(1, Point::taxicab_distance(&Point(-1, 0), &Point(0, 0)));
        assert_eq!(1, Point::taxicab_distance(&Point(0, -1), &Point(0, 0)));
        assert_eq!(1, Point::taxicab_distance(&Point(0, 0), &Point(-1, 0)));
        assert_eq!(1, Point::taxicab_distance(&Point(0, 0), &Point(0, -1)));

        assert_eq!(2, Point::taxicab_distance(&Point(2, 0), &Point(0, 0)));
        assert_eq!(2, Point::taxicab_distance(&Point(0, 2), &Point(0, 0)));
        assert_eq!(2, Point::taxicab_distance(&Point(0, 0), &Point(2, 0)));
        assert_eq!(2, Point::taxicab_distance(&Point(0, 0), &Point(0, 2)));
        assert_eq!(2, Point::taxicab_distance(&Point(-2, 0), &Point(0, 0)));
        assert_eq!(2, Point::taxicab_distance(&Point(0, -2), &Point(0, 0)));
        assert_eq!(2, Point::taxicab_distance(&Point(0, 0), &Point(-2, 0)));
        assert_eq!(2, Point::taxicab_distance(&Point(0, 0), &Point(0, -2)));

        assert_eq!(2, Point::taxicab_distance(&Point(1, 1), &Point(0, 0)));
        assert_eq!(2, Point::taxicab_distance(&Point(1, 0), &Point(0, 1)));
        assert_eq!(2, Point::taxicab_distance(&Point(0, 1), &Point(1, 0)));
        assert_eq!(2, Point::taxicab_distance(&Point(0, 0), &Point(1, 1)));
        assert_eq!(2, Point::taxicab_distance(&Point(-1, -1), &Point(0, 0)));
        assert_eq!(2, Point::taxicab_distance(&Point(-1, 0), &Point(0, -1)));
        assert_eq!(2, Point::taxicab_distance(&Point(0, -1), &Point(-1, 0)));
        assert_eq!(2, Point::taxicab_distance(&Point(0, 0), &Point(-1, -1)));
    }
}
