use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
    slice::Iter,
};

// file constants
const FILE_NAME: &str = "data1.txt";
const DIRECTION_INDEX: usize = 0;
const DIRECTION_MULTIPLIER_INDEX: usize = 1;

// point constants
const X_DISTANCE: i32 = 1;
const Y_DISTANCE: i32 = 1;

// head and node constants
const START_X_INDEX: i32 = 0;
const START_Y_INDEX: i32 = 0;
const PART_ONE_NODE_COUNT: usize = 1;
const PART_TWO_NODE_COUNT: usize = 8;

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    let mut head = Head::new(PART_ONE_NODE_COUNT);

    data.lines().for_each(|line| {
        let (direction, multipler) = parse_line(line);
        for _ in 0..multipler {
            head.travel(&direction);

            match Point::within_grid(
                (&head.position, &head.next.position),
                (X_DISTANCE, Y_DISTANCE),
            ) {
                true => continue,
                false => {
                    head.next.travel(&head.position);
                    head.next.update_history();
                }
            }
        }
    });

    let last_node = head.next;

    println!("Part one: {}", last_node.unique_position_count);
}

fn part_two(data: &str) {
    let mut head = Head::new(PART_TWO_NODE_COUNT);

    data.lines().for_each(|line| {
        let (direction, multipler) = parse_line(line);
        for _ in 0..multipler {
            head.travel(&direction);

            match Point::within_grid(
                (&head.position, &head.next.position),
                (X_DISTANCE, Y_DISTANCE),
            ) {
                true => continue,
                false => {
                    head.next.travel(&head.position);
                    head.next.update_history();
                }
            }

            let mut next_node = &mut head.next;

            while next_node.next != None {
                if !Point::within_grid(
                    (
                        &next_node.position,
                        &next_node.next.as_ref().unwrap().position,
                    ),
                    (X_DISTANCE, Y_DISTANCE),
                ) {
                    next_node.next.as_mut().unwrap().travel(&next_node.position);
                    next_node.next.as_mut().unwrap().update_history();
                }

                next_node = next_node.next.as_mut().unwrap();
            }
        }
    });

    let mut last_node = &mut head.next;
    while last_node.next != None {
        last_node = last_node.next.as_mut().unwrap();
    }

    println!("Part two: {}", last_node.unique_position_count);
}

fn parse_line(line: &str) -> (Direction, usize) {
    let tokens: Vec<&str> = line.split(' ').collect();
    (
        Direction::from(tokens[DIRECTION_INDEX]),
        tokens[DIRECTION_MULTIPLIER_INDEX].parse::<usize>().unwrap(),
    )
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    U,
    D,
    R,
    L,
}

impl Direction {
    fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] =
            [Direction::U, Direction::D, Direction::R, Direction::L];
        DIRECTIONS.iter()
    }
}

#[derive(Clone)]
enum DiagonalDirection {
    UR,
    UL,
    DL,
    DR,
}

impl DiagonalDirection {
    fn to_directions(&self) -> HashSet<Direction> {
        let mut directions = HashSet::new();
        match self {
            DiagonalDirection::UL => {
                directions.insert(Direction::U);
                directions.insert(Direction::L);
            }
            DiagonalDirection::UR => {
                directions.insert(Direction::U);
                directions.insert(Direction::R);
            }
            DiagonalDirection::DL => {
                directions.insert(Direction::D);
                directions.insert(Direction::L);
            }
            DiagonalDirection::DR => {
                directions.insert(Direction::D);
                directions.insert(Direction::R);
            }
        }

        directions
    }

    fn iterator() -> Iter<'static, DiagonalDirection> {
        static DIAGONAL_DIRECTIONS: [DiagonalDirection; 4] = [
            DiagonalDirection::UL,
            DiagonalDirection::UR,
            DiagonalDirection::DL,
            DiagonalDirection::DR,
        ];
        DIAGONAL_DIRECTIONS.iter()
    }
}

impl Direction {
    fn from(c: &str) -> Direction {
        match c {
            "U" => Direction::U,
            "D" => Direction::D,
            "R" => Direction::R,
            "L" => Direction::L,
            _ => panic!("Cant only convert chars U, D, R, L to Direction"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn travel(direction: &Direction, point: Point) -> Point {
        match direction {
            Direction::U => Point(point.0, point.1 + 1),
            Direction::L => Point(point.0 - 1, point.1),
            Direction::R => Point(point.0 + 1, point.1),
            Direction::D => Point(point.0, point.1 - 1),
        }
    }

    /// returns true if the two points are within the grid created by the passed distances.
    /// The distances will be used for both x and y directions separately. E.g. Points ((0,0), (1, -2)))
    /// with (1, 2) distances will return true since x moved <= x distance and y moved <= y distance
    fn within_grid(points: (&Point, &Point), distances: (i32, i32)) -> bool {
        Point::reachable(points.0 .0, points.1 .0, distances.0)
            && Point::reachable(points.0 .1, points.1 .1, distances.1)
    }

    fn adjacent(points: (&Point, &Point)) -> bool {
        for direction in Direction::iterator() {
            // TODO remove clone
            let new_point = Point::travel(direction, points.0.clone());

            if &new_point == points.1 {
                return true;
            }
        }

        false
    }

    /// returns true if it is possible to touch or surpasses the second_marker
    /// by either moving left or right the passed distance
    fn reachable(first_marker: i32, second_marker: i32, distance: i32) -> bool {
        let min_marker = min(first_marker, second_marker);
        let max_marker = max(first_marker, second_marker);

        if distance > min_marker {
            // only check max marker since min marker - distance is negative
            if min_marker + distance < max_marker {
                return false;
            }
        } else {
            // check both markers since a range can be constructed
            let range = (min_marker - distance)..=(min_marker + distance);
            if !range.contains(&max_marker) {
                return false;
            }
        }
        true
    }
}

struct Head {
    position: Point,
    next: Node,
}

impl Head {
    fn new(node_count: usize) -> Head {
        Head {
            position: Point(START_X_INDEX, START_Y_INDEX),
            next: Node::new(node_count),
        }
    }

    fn travel(&mut self, direction: &Direction) {
        // TODO remove clone
        self.position = Point::travel(direction, self.position.clone());
    }
}

#[derive(Clone, PartialEq)]
struct Node {
    position: Point,
    history: HashSet<Point>,
    unique_position_count: usize,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(node_count_remainder: usize) -> Node {
        let mut history = HashSet::new();
        history.insert(Point(START_X_INDEX, START_Y_INDEX));

        let mut next_node = None;
        if node_count_remainder > 0 {
            next_node = Some(Box::new(Node::new(node_count_remainder - 1)));
        }

        Node {
            position: Point(START_X_INDEX, START_Y_INDEX),
            history,
            unique_position_count: 1,
            next: next_node,
        }
    }

    fn update_history(&mut self) {
        match self.history.get(&self.position) {
            Some(_) => (),
            None => {
                self.unique_position_count += 1;
                // TODO remove clone
                self.history.insert(self.position.clone());
            }
        };
    }

    /// Travel toward the passed position. The travel has three priority levels.
    /// First priority: Travel up, down, left or right to get adjacent to passed point
    /// Second priority: Travel 1 space diagonally to get adjacent to passed point
    /// Third priority: Travel 1 space diagonally to get diagonal to passed point
    fn travel(&mut self, toward_position: &Point) {
        // First priority: Travel up, down, left or right to get adjacent to passed point
        for direction in Direction::iterator() {
            // TODO remove clone
            let new_position = Point::travel(direction, self.position.clone());

            match Point::adjacent((&new_position, toward_position)) {
                true => {
                    self.position = new_position;
                    return;
                }
                false => continue,
            }
        }

        // Second priority: Travel 1 space diagonally to get adjacent to passed point
        for diagonal_direction in DiagonalDirection::iterator() {
            let mut new_position = self.position.clone();
            // TODO remove clone
            for direction in diagonal_direction.clone().to_directions() {
                new_position = Point::travel(&direction, new_position);
            }

            match Point::adjacent((&new_position, toward_position)) {
                true => {
                    self.position = new_position;
                    return;
                }
                false => continue,
            }
        }

        // Third priority: Travel 1 space diagonally to get diagonal to passed point
        for diagonal_direction in DiagonalDirection::iterator() {
            let mut new_position = self.position.clone();
            // TODO remove clone
            for direction in diagonal_direction.clone().to_directions() {
                new_position = Point::travel(&direction, new_position);
            }

            match Point::within_grid((&new_position, toward_position), (X_DISTANCE, Y_DISTANCE)) {
                true => {
                    self.position = new_position;
                    return;
                }
                false => continue,
            }
        }

        panic!("No direction or diagonal direction travel will get near connected position");
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;

    #[test]
    fn points_within() {
        assert_eq!(
            Point::within_grid((&Point(0, 0), &Point(0, 0)), (0, 0)),
            true
        );

        assert_eq!(
            Point::within_grid((&Point(0, 0), &Point(2, 1)), (3, 1)),
            true
        );
        assert_eq!(
            Point::within_grid((&Point(0, 0), &Point(2, 1)), (2, 2)),
            true
        );
        assert_eq!(
            Point::within_grid((&Point(0, 0), &Point(2, 1)), (2, 1)),
            true
        );
        assert_eq!(
            Point::within_grid((&Point(0, 0), &Point(2, 1)), (1, 1)),
            false
        );
        assert_eq!(
            Point::within_grid((&Point(0, 0), &Point(2, 1)), (2, 0)),
            false
        );

        assert_eq!(
            Point::within_grid((&Point(1, 2), &Point(0, 0)), (1, 3)),
            true
        );
        assert_eq!(
            Point::within_grid((&Point(1, 2), &Point(0, 0)), (2, 2)),
            true
        );
        assert_eq!(
            Point::within_grid((&Point(1, 2), &Point(0, 0)), (1, 2)),
            true
        );
        assert_eq!(
            Point::within_grid((&Point(1, 2), &Point(0, 0)), (1, 1)),
            false
        );
        assert_eq!(
            Point::within_grid((&Point(1, 2), &Point(0, 0)), (0, 2)),
            false
        );
    }

    #[test]
    fn points_connected() {
        assert_eq!(Point::adjacent((&Point(0, 0), &Point(0, 0))), false);

        assert_eq!(Point::adjacent((&Point(0, 0), &Point(1, 0))), true);
        assert_eq!(Point::adjacent((&Point(0, 0), &Point(-1, 0))), true);
        assert_eq!(Point::adjacent((&Point(0, 0), &Point(0, 1))), true);
        assert_eq!(Point::adjacent((&Point(0, 0), &Point(0, -1))), true);
        assert_eq!(Point::adjacent((&Point(1, 0), &Point(0, 0))), true);
        assert_eq!(Point::adjacent((&Point(-1, 0), &Point(0, 0))), true);
        assert_eq!(Point::adjacent((&Point(0, 1), &Point(0, 0))), true);
        assert_eq!(Point::adjacent((&Point(0, -1), &Point(0, 0))), true);

        assert_eq!(Point::adjacent((&Point(0, 0), &Point(1, 1))), false);
        assert_eq!(Point::adjacent((&Point(0, 0), &Point(-1, 1))), false);
        assert_eq!(Point::adjacent((&Point(0, 0), &Point(-1, -1))), false);
        assert_eq!(Point::adjacent((&Point(0, 0), &Point(1, -1))), false);
        assert_eq!(Point::adjacent((&Point(1, 1), &Point(0, 0))), false);
        assert_eq!(Point::adjacent((&Point(-1, 1), &Point(0, 0))), false);
        assert_eq!(Point::adjacent((&Point(-1, -1), &Point(0, 0))), false);
        assert_eq!(Point::adjacent((&Point(1, -1), &Point(0, 0))), false);
    }
}
