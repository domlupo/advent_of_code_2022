use std::{cmp::max, collections::HashMap, fs};

const FILE_NAME: &str = "data1.txt";
const CHAR_BYTE_TO_NUMBER: usize = 48;

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    let trees = Trees::new(data);

    let mut visible_tree_count = 0;
    trees.heights.keys().for_each(|point| {
        if trees.is_visible(point) {
            visible_tree_count += 1;
        }
    });

    println!("Part one: {}", visible_tree_count);
}

fn part_two(data: &str) {
    let trees = Trees::new(data);

    let mut max_visiblity = 0;
    trees.heights.keys().for_each(|point| {
        max_visiblity = max(max_visiblity, trees.get_visibility(point));
    });

    println!("Part two: {}", max_visiblity);
}

#[derive(Eq, Hash, PartialEq)]
struct Point(usize, usize);

struct Trees {
    heights: HashMap<Point, usize>,
    max_x_index: usize,
    max_y_index: usize,
}

impl Trees {
    fn new(data: &str) -> Trees {
        let mut heights = HashMap::new();

        let mut y = 0;
        data.lines().for_each(|line| {
            for (x, c) in line.as_bytes().iter().enumerate() {
                heights.insert(Point(x, y), *c as usize - CHAR_BYTE_TO_NUMBER);
            }
            y += 1;
        });

        let max_x_index = data.find('\n').unwrap() - 1;
        let max_y_index = y - 1;
        Trees {
            heights,
            max_x_index,
            max_y_index,
        }
    }

    fn is_visible(&self, point: &Point) -> bool {
        self.is_top_visible(point)
            || self.is_left_visible(point)
            || self.is_right_visible(point)
            || self.is_bottom_visible(point)
    }

    fn get_visibility(&self, point: &Point) -> usize {
        self.get_top_visibility(point)
            * self.get_left_visibility(point)
            * self.get_right_visibility(point)
            * self.get_bottom_visibility(point)
    }

    fn is_left_visible(&self, point: &Point) -> bool {
        let y = point.1;
        let height = self.heights.get(point).unwrap();
        for i in 0..point.0 {
            let left_tree_height = match self.heights.get(&Point(i, y)) {
                Some(height) => height,
                None => panic!("Tree should exist"),
            };

            // tree is not visible from left since at least one tree
            // to the left of it is taller or same height
            if left_tree_height >= height {
                return false;
            }
        }
        true
    }

    fn get_left_visibility(&self, point: &Point) -> usize {
        let y = point.1;
        let height = self.heights.get(point).unwrap();
        let mut visibility = 0;
        for i in (0..point.0).rev() {
            let left_tree_height = match self.heights.get(&Point(i, y)) {
                Some(height) => height,
                None => panic!("Tree should exist"),
            };

            visibility += 1;

            // tree is not visible from left since at least one tree
            // to the left of it is taller or same height
            if left_tree_height >= height {
                return visibility;
            }
        }

        visibility
    }

    fn is_right_visible(&self, point: &Point) -> bool {
        let y = point.1;
        let height = self.heights.get(point).unwrap();
        for i in point.0 + 1..=self.max_x_index {
            let right_tree_height = match self.heights.get(&Point(i, y)) {
                Some(height) => height,
                None => panic!("Tree should exist"),
            };

            // tree is not visible from right since at least one tree
            // to the right of it is taller or same height
            if right_tree_height >= height {
                return false;
            }
        }
        true
    }

    fn get_right_visibility(&self, point: &Point) -> usize {
        let y = point.1;
        let height = self.heights.get(point).unwrap();
        let mut visibility = 0;
        for i in point.0 + 1..=self.max_x_index {
            let right_tree_height = match self.heights.get(&Point(i, y)) {
                Some(height) => height,
                None => panic!("Tree should exist"),
            };

            visibility += 1;

            // tree is not visible from right since at least one tree
            // to the right of it is taller or same height
            if right_tree_height >= height {
                return visibility;
            }
        }
        visibility
    }

    fn is_top_visible(&self, point: &Point) -> bool {
        let x = point.0;
        let height = self.heights.get(point).unwrap();
        for i in 0..point.1 {
            let top_tree_height = match self.heights.get(&Point(x, i)) {
                Some(height) => height,
                None => panic!("Tree should exist"),
            };

            // tree is not visible from top since at least one tree
            // to the top of it is taller or same height
            if top_tree_height >= height {
                return false;
            }
        }
        true
    }

    fn get_top_visibility(&self, point: &Point) -> usize {
        let x = point.0;
        let height = self.heights.get(point).unwrap();
        let mut visibility = 0;
        for i in (0..point.1).rev() {
            let top_tree_height = match self.heights.get(&Point(x, i)) {
                Some(height) => height,
                None => panic!("Tree should exist"),
            };

            visibility += 1;

            // tree is not visible from top since at least one tree
            // to the top of it is taller or same height
            if top_tree_height >= height {
                return visibility;
            }
        }
        visibility
    }

    fn is_bottom_visible(&self, point: &Point) -> bool {
        let x = point.0;
        let height = self.heights.get(point).unwrap();
        for i in point.1 + 1..=self.max_y_index {
            let top_tree_height = match self.heights.get(&Point(x, i)) {
                Some(height) => height,
                None => panic!("Tree should exist"),
            };

            // tree is not visible from top since at least one tree
            // to the top of it is taller or same height
            if top_tree_height >= height {
                return false;
            }
        }
        true
    }

    fn get_bottom_visibility(&self, point: &Point) -> usize {
        let x = point.0;
        let height = self.heights.get(point).unwrap();
        let mut visibility = 0;
        for i in point.1 + 1..=self.max_y_index {
            let top_tree_height = match self.heights.get(&Point(x, i)) {
                Some(height) => height,
                None => panic!("Tree should exist"),
            };

            visibility += 1;

            // tree is not visible from top since at least one tree
            // to the top of it is taller or same height
            if top_tree_height >= height {
                return visibility;
            }
        }
        visibility
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point, Trees};

    #[test]
    fn test_trees_new() {
        /*
        123
        456
        789
        */
        let data = "123\n456\n789\n";

        let trees = Trees::new(data);
        assert_eq!(*trees.heights.get(&Point(0, 0)).unwrap(), 1);
        assert_eq!(*trees.heights.get(&Point(2, 0)).unwrap(), 3);
        assert_eq!(*trees.heights.get(&Point(0, 2)).unwrap(), 7);
        assert_eq!(*trees.heights.get(&Point(2, 2)).unwrap(), 9);
    }

    #[test]
    fn test_trees_is_visible() {
        /*
        33333
        12122
        33333
        */
        let data = "33333\n12122\n33333\n";

        let trees = Trees::new(data);
        assert_eq!(trees.is_visible(&Point(0, 1)), true);
        assert_eq!(trees.is_visible(&Point(1, 1)), true);
        assert_eq!(trees.is_visible(&Point(2, 1)), false);
        assert_eq!(trees.is_visible(&Point(3, 1)), false);
        assert_eq!(trees.is_visible(&Point(4, 1)), true);

        /*
        33333
        22121
        33333
        */
        let data = "33333\n22121\n33333\n";

        let trees = Trees::new(data);
        assert_eq!(trees.is_visible(&Point(0, 1)), true);
        assert_eq!(trees.is_visible(&Point(1, 1)), false);
        assert_eq!(trees.is_visible(&Point(2, 1)), false);
        assert_eq!(trees.is_visible(&Point(3, 1)), true);
        assert_eq!(trees.is_visible(&Point(3, 1)), true);

        /*
        313
        323
        313
        323
        323
        */
        let data = "313\n323\n313\n323\n323\n";

        let trees = Trees::new(data);
        assert_eq!(trees.is_visible(&Point(1, 0)), true);
        assert_eq!(trees.is_visible(&Point(1, 1)), true);
        assert_eq!(trees.is_visible(&Point(1, 2)), false);
        assert_eq!(trees.is_visible(&Point(1, 3)), false);
        assert_eq!(trees.is_visible(&Point(1, 4)), true);

        /*
        323
        323
        313
        323
        313
        */
        let data = "323\n323\n313\n323\n313\n";

        let trees = Trees::new(data);
        assert_eq!(trees.is_visible(&Point(1, 0)), true);
        assert_eq!(trees.is_visible(&Point(1, 1)), false);
        assert_eq!(trees.is_visible(&Point(1, 2)), false);
        assert_eq!(trees.is_visible(&Point(1, 3)), true);
        assert_eq!(trees.is_visible(&Point(1, 3)), true);
    }

    #[test]
    fn test_trees_visibility() {
        /*
        111111
        121122
        111111
        */
        let data = "111111\n121122\n111111\n";

        let trees = Trees::new(data);
        assert_eq!(trees.get_visibility(&Point(0, 1)), 0);
        assert_eq!(trees.get_visibility(&Point(1, 1)), 3);
        assert_eq!(trees.get_visibility(&Point(2, 1)), 1);
        assert_eq!(trees.get_visibility(&Point(3, 1)), 1);
        assert_eq!(trees.get_visibility(&Point(4, 1)), 3);
        assert_eq!(trees.get_visibility(&Point(5, 1)), 0);

        /*
        111111
        221121
        111111
        */
        let data = "111111\n221121\n111111\n";

        let trees = Trees::new(data);
        assert_eq!(trees.get_visibility(&Point(0, 1)), 0);
        assert_eq!(trees.get_visibility(&Point(1, 1)), 3);
        assert_eq!(trees.get_visibility(&Point(2, 1)), 1);
        assert_eq!(trees.get_visibility(&Point(3, 1)), 1);
        assert_eq!(trees.get_visibility(&Point(4, 1)), 3);
        assert_eq!(trees.get_visibility(&Point(5, 1)), 0);

        /*
        111
        121
        111
        111
        121
        121
        */
        let data = "111\n121\n111\n111\n121\n121\n";

        let trees = Trees::new(data);
        assert_eq!(trees.get_visibility(&Point(1, 0)), 0);
        assert_eq!(trees.get_visibility(&Point(1, 1)), 3);
        assert_eq!(trees.get_visibility(&Point(1, 2)), 1);
        assert_eq!(trees.get_visibility(&Point(1, 3)), 1);
        assert_eq!(trees.get_visibility(&Point(1, 4)), 3);
        assert_eq!(trees.get_visibility(&Point(1, 5)), 0);

        /*
        121
        121
        111
        111
        121
        111
        */
        let data = "121\n121\n111\n111\n121\n111\n";

        let trees = Trees::new(data);
        assert_eq!(trees.get_visibility(&Point(1, 0)), 0);
        assert_eq!(trees.get_visibility(&Point(1, 1)), 3);
        assert_eq!(trees.get_visibility(&Point(1, 2)), 1);
        assert_eq!(trees.get_visibility(&Point(1, 3)), 1);
        assert_eq!(trees.get_visibility(&Point(1, 4)), 3);
        assert_eq!(trees.get_visibility(&Point(1, 5)), 0);
    }
}
