use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

const FILE_NAME: &str = "data1.txt";
const ANSWER_LINE: i32 = 10;

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    let (mut item_grid, sensor_nearest_beacon) = parse(&data);

    part_one(&mut item_grid, &sensor_nearest_beacon);
}

fn part_one(
    item_grid: &mut HashMap<Point, Vec<Item>>,
    sensor_nearest_beacon: &HashMap<Point, Point>,
) {
    // fill the item grid with the no beacon border. Only the border is filled instead of the entire
    // grid due to computation cost. The grid can be found if needed due to relating the border with
    // the sensor that produced it.
    for (sensor, nearest_beacon) in sensor_nearest_beacon {
        let no_beacon_positions = get_no_beacon_positions(sensor, nearest_beacon);

        for (point, sensor) in no_beacon_positions {
            match item_grid.get(&point) {
                Some(items) => {
                    // TODO Does a new vector need allocated?
                    let mut items = items.iter().cloned().collect::<Vec<Item>>();
                    items.push(Item::NoHiddenBeacon(sensor));
                    item_grid.insert(point, items.to_vec());
                }
                None => {
                    item_grid.insert(point, vec![Item::NoHiddenBeacon(sensor)]);
                }
            }
        }
    }

    // filter item_grid to the relevant item_line to reduce computation cost
    let mut item_line = HashMap::new();
    for (point, item) in item_grid.iter() {
        if point.1 == ANSWER_LINE {
            item_line.insert(point.clone(), item.clone());
        }
    }

    // if the item line has 2 beacons from the same sensor, then no additional
    // beacons exist between these 2 beacons. If the item line has only 1 beacon
    // from a sensor, then only that position does not have a beacon.
    let mut sensor_beacon_count: HashMap<Point, usize> = HashMap::new();
    for (_point, items) in &item_line {
        for item in items.iter() {
            match item {
                Item::NoHiddenBeacon(sensor) => match sensor_beacon_count.get(sensor) {
                    Some(sensor_count) => {
                        sensor_beacon_count.insert(sensor.clone(), sensor_count + 1);
                    }
                    None => {
                        sensor_beacon_count.insert(sensor.clone(), 1);
                    }
                },
                _ => (),
            }
        }
    }

    let mut no_beacon_count = 0;
    let mut beacon_count = 0;
    let mut related_sensor: Option<Point> = None;

    let min_x = item_line.keys().map(|point| point.0).min().unwrap();
    let max_x = item_line.keys().map(|point| point.0).max().unwrap();
    let x = (min_x..=max_x).collect::<Vec<i32>>();
    let mut x_iter = x.iter().peekable();

    while x_iter.peek().is_some() {
        let mut increment_no_beacon_count= false;

        if related_sensor.is_some() {
            increment_no_beacon_count = true;
        }

        let point = Point(*x_iter.next().unwrap(), ANSWER_LINE);
        match item_line.get(&point) {
            Some(items) => {
                for item in items.iter() {
                    match item {
                        Item::Beacon => {
                            // Any point with a beacon will have both one Item::Beacon and at least one 
                            // Item::NoHiddenBeacon so remove this point from the final no beacon count
                            beacon_count += 1;
                        }
                        Item::Sensor => (),
                        Item::NoHiddenBeacon(sensor) => {
                            increment_no_beacon_count = true;
                            let sensor_beacon_count = *sensor_beacon_count.get(sensor).unwrap();

                            // TODO start tracking a new
                            // BUG right now this goes off sensor x but should go off which grid 
                            // will last longer
                            if related_sensor.is_some()
                                && related_sensor.as_ref().unwrap().0 < sensor.0
                                && sensor_beacon_count == 2
                            {
                                related_sensor = Some(sensor.clone());
                            }

                            // TODO stop tracking a no beacon grid
                            // BUG

                            // Start tracking a no beacon grid
                            if related_sensor.is_none() && sensor_beacon_count == 2 {
                                related_sensor = Some(sensor.clone());
                            }
                        }
                    }
                }
            }
            None => (),
        }

        if increment_no_beacon_count {
            no_beacon_count += 1;
        }
    }

    println!("part one: {}", no_beacon_count - beacon_count);
}

fn parse(data: &str) -> (HashMap<Point, Vec<Item>>, HashMap<Point, Point>) {
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

        item_grid.insert(sensor_position.clone(), vec![Item::Sensor]);
        item_grid.insert(beacon_position.clone(), vec![Item::Beacon]);
        nearest_beacon.insert(sensor_position, beacon_position);
    }

    (item_grid, nearest_beacon)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Item {
    Beacon,
    Sensor,
    NoHiddenBeacon(Point),
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
fn get_no_beacon_positions(sensor: &Point, nearest_beacon: &Point) -> HashMap<Point, Point> {
    let taxicab_distance = Point::taxicab_distance(sensor, nearest_beacon);
    let mut no_beacon_positions = HashMap::new();
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
        sensor,
        &mut no_beacon_positions,
    );
    point = travel_grid(
        &point,
        taxicab_distance,
        &Direction::D,
        &Direction::L,
        sensor,
        &mut no_beacon_positions,
    );
    point = travel_grid(
        &point,
        taxicab_distance,
        &Direction::R,
        &Direction::D,
        sensor,
        &mut no_beacon_positions,
    );
    travel_grid(
        &point,
        taxicab_distance,
        &Direction::U,
        &Direction::R,
        sensor,
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
    sensor: &Point,
    no_beacon_positions: &mut HashMap<Point, Point>,
) -> Point {
    let mut point = point.clone();
    let mut new_count = 0;
    loop {
        new_count += 1;

        point = Point::travel(direction_undo, &point);
        point = Point::travel(new_direction, &point);
        let new_point = point.clone();

        no_beacon_positions.insert(new_point.clone(), sensor.clone());

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
