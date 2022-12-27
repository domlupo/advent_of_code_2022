use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt, fs,
    ops::Range,
};

const FILE_NAME: &str = "data1.txt";

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    let material_grid = parse_material_grid(&data);
    display_material_grid(&material_grid);
    part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    let mut material_grid = parse_material_grid(data);
    let total_sand_positions = drop_sand_one(Point(500, 0), &mut material_grid);
    println!("Part one: {}", total_sand_positions);
}

fn part_two(data: &str) {
    let mut material_grid = parse_material_grid(data);
    let y_max = get_y_max(&material_grid);
    let total_sand_positions = drop_sand_two(Point(500, 0), y_max + 1, &mut material_grid);
    println!("Part two: {}", total_sand_positions);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(usize, usize);

#[derive(Debug, Eq, PartialEq)]
enum Material {
    Air,
    Rock,
    Sand,
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Material::Air => write!(f, "."),
            Material::Rock => write!(f, "#"),
            Material::Sand => write!(f, "o"),
        }
    }
}

fn display_material_grid(material_grid: &HashMap<Point, Material>) {
    let x_min = get_x_min(material_grid);
    let x_max = get_x_max(material_grid);
    let y_max = get_y_max(material_grid);

    for y in 0..=y_max {
        for x in x_min..=x_max {
            match material_grid.get(&Point(x, y)) {
                Some(material) => {
                    print!("{}", material);
                }
                None => print!("{}", Material::Air),
            }
        }
        println!();
    }
}

fn get_y_max(material_grid: &HashMap<Point, Material>) -> usize {
    let y_vals: Vec<usize> = material_grid.keys().map(|k| k.1).collect();
    *y_vals.iter().max().unwrap()
}

fn get_x_min(material_grid: &HashMap<Point, Material>) -> usize {
    let x_vals: Vec<usize> = material_grid.keys().map(|k| k.0).collect();
    *x_vals.iter().min().unwrap()
}

fn get_x_max(material_grid: &HashMap<Point, Material>) -> usize {
    let x_vals: Vec<usize> = material_grid.keys().map(|k| k.0).collect();
    *x_vals.iter().max().unwrap()
}

fn drop_sand_one(start_position: Point, material_grid: &mut HashMap<Point, Material>) -> i32 {
    let y_max = get_y_max(material_grid);
    let mut total_sand_positions = 0;

    'drop_new_sand: loop {
        let mut current_sand_position = start_position;
        let mut prior_sand_position: Option<Point> = None;
        while current_sand_position.1 < y_max {
            // attempt to drop sand further
            for sand_position in get_sorted_sand_drop(current_sand_position) {
                match material_grid.get(&sand_position) {
                    Some(_) => continue,
                    None => {
                        current_sand_position = sand_position;
                        break;
                    }
                }
            }

            // if sand position attempts to drop further failed, store its position and drop more new sand
            match prior_sand_position {
                Some(prior_sand_position) if prior_sand_position == current_sand_position => {
                    material_grid.insert(current_sand_position, Material::Sand);
                    total_sand_positions += 1;
                    continue 'drop_new_sand;
                }
                _ => (),
            }

            prior_sand_position = Some(current_sand_position);
        }
        return total_sand_positions;
    }
}

fn drop_sand_two(
    start_position: Point,
    max_y: usize,
    material_grid: &mut HashMap<Point, Material>,
) -> i32 {
    let mut total_sand_positions = 0;

    'drop_new_sand: loop {
        let mut current_sand_position = start_position;
        let mut prior_sand_position: Option<Point> = None;

        while material_grid.get(&start_position).is_none() {
            // attempt to drop sand further
            for sand_position in get_sorted_sand_drop(current_sand_position) {
                match material_grid.get(&sand_position) {
                    Some(_) => continue,
                    None => {
                        current_sand_position = sand_position;
                        break;
                    }
                }
            }

            // if sand position attempts to drop further failed, store its position and drop more new sand
            match prior_sand_position {
                Some(prior_sand_position) if prior_sand_position == current_sand_position => {
                    material_grid.insert(current_sand_position, Material::Sand);
                    total_sand_positions += 1;
                    continue 'drop_new_sand;
                }
                _ => (),
            }

            // if sand position is at maximum depth, store its position and drop more new sand
            if current_sand_position.1 == max_y {
                material_grid.insert(current_sand_position, Material::Sand);
                total_sand_positions += 1;
                continue 'drop_new_sand;
            }

            prior_sand_position = Some(current_sand_position);
        }
        return total_sand_positions;
    }
}

fn get_sorted_sand_drop(p: Point) -> Vec<Point> {
    if p.0 == 0 {
        vec![Point(p.0, p.1 + 1), Point(p.0 + 1, p.1 + 1)]
    } else {
        vec![
            Point(p.0, p.1 + 1),
            Point(p.0 - 1, p.1 + 1),
            Point(p.0 + 1, p.1 + 1),
        ]
    }
}

fn parse_material_grid(data: &str) -> HashMap<Point, Material> {
    let mut material_grid = HashMap::new();
    let lines = data.lines();

    for line in lines {
        let comma_separated_points = line
            .split_whitespace()
            .collect::<String>()
            .replace("->", ",");
        let comma_separated_points = comma_separated_points.split(',').collect::<Vec<&str>>();
        let mut points_iter = comma_separated_points.iter().peekable();

        let mut prior_point = None;
        loop {
            let x = points_iter.next().unwrap().parse::<usize>().unwrap();
            let y = points_iter.next().unwrap().parse::<usize>().unwrap();
            let point = Point(x, y);
            material_grid.insert(point, Material::Rock);

            match prior_point {
                Some(prior_point) => {
                    for point in points_between(&prior_point, &point).iter() {
                        material_grid.insert(*point, Material::Rock);
                    }
                }
                None => (),
            }

            match points_iter.peek() {
                Some(_) => prior_point = Some(point),
                None => break,
            }
        }
    }

    material_grid
}

fn points_between(p1: &Point, p2: &Point) -> HashSet<Point> {
    if p1 == p2 {
        HashSet::new()
    } else if p1.0 != p2.0 && p1.1 != p2.1 {
        panic!("points must share an axis to find inner points");
    } else if p1.0 != p2.0 {
        let range = Range {
            start: min(p1.0 + 1, p2.0 + 1),
            end: max(p1.0, p2.0),
        };
        range.into_iter().map(|x| Point(x, p1.1)).collect()
    } else if p1.1 != p2.1 {
        let range = Range {
            start: min(p1.1 + 1, p2.1 + 1),
            end: max(p1.1, p2.1),
        };
        range.into_iter().map(|y| Point(p1.0, y)).collect()
    } else {
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{points_between, Point};

    #[test]
    fn test_points_between() {
        assert_eq!(points_between(&Point(0, 0), &Point(0, 0)), HashSet::new());

        assert_eq!(points_between(&Point(1, 0), &Point(0, 0)), HashSet::new());
        assert_eq!(
            points_between(&Point(2, 0), &Point(0, 0)),
            hashset(&[Point(1, 0)])
        );
        assert_eq!(
            points_between(&Point(3, 0), &Point(0, 0)),
            hashset(&[Point(1, 0), Point(2, 0)])
        );

        assert_eq!(points_between(&Point(0, 1), &Point(0, 0)), HashSet::new());
        assert_eq!(
            points_between(&Point(0, 2), &Point(0, 0)),
            hashset(&[Point(0, 1)])
        );
        assert_eq!(
            points_between(&Point(0, 3), &Point(0, 0)),
            hashset(&[Point(0, 1), Point(0, 2)])
        );

        assert_eq!(points_between(&Point(0, 0), &Point(1, 0)), HashSet::new());
        assert_eq!(
            points_between(&Point(0, 0), &Point(2, 0)),
            hashset(&[Point(1, 0)])
        );
        assert_eq!(
            points_between(&Point(0, 0), &Point(3, 0)),
            hashset(&[Point(1, 0), Point(2, 0)])
        );

        assert_eq!(points_between(&Point(0, 0), &Point(0, 1)), HashSet::new());
        assert_eq!(
            points_between(&Point(0, 0), &Point(0, 2)),
            hashset(&[Point(0, 1)])
        );
        assert_eq!(
            points_between(&Point(0, 0), &Point(0, 3)),
            hashset(&[Point(0, 1), Point(0, 2)])
        );
    }

    fn hashset(points: &[Point]) -> HashSet<Point> {
        HashSet::from_iter(points.iter().cloned())
    }
}
