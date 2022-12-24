use std::{cmp::Ordering, fs, iter::Peekable, str::Chars};

const FILE_NAME: &str = "data1.txt";

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    part_one(&data);
    part_two(&data);
}

fn part_one(data: &str) {
    let mut lines = data.lines();
    let mut sum = 0;
    let mut i = 1;

    loop {
        let mut first_packet = lines.next().unwrap().chars().peekable();
        let mut second_packet = lines.next().unwrap().chars().peekable();
        match is_packets_correct_order(&mut first_packet, &mut second_packet) {
            true => sum += i,
            false => (),
        }

        i += 1;
        match lines.next() {
            Some(_) => continue,
            None => break,
        }
    }

    println!("part one: {}", sum);
}

fn part_two(data: &str) {
    let mut lines = data.lines();
    let mut sorted_packets: Vec<Packet> =
        vec![Packet("[[2]]".to_string()), Packet("[[6]]".to_string())];

    loop {
        sorted_packets.push(Packet(lines.next().unwrap().to_string()));
        sorted_packets.push(Packet(lines.next().unwrap().to_string()));
        match lines.next() {
            Some(_) => continue,
            None => break,
        }
    }

    sorted_packets.sort();
    let mut answer = 1;
    for (i, packet) in sorted_packets.iter().enumerate() {
        if packet.0 == "[[2]]" || packet.0 == "[[6]]" {
            answer *= i + 1;
        }
    }

    println!("part two: {}", answer);
}

#[derive(Eq)]
struct Packet(String);

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match is_packets_correct_order(
            &mut self.0.chars().peekable(),
            &mut other.0.chars().peekable(),
        ) {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn is_packets_correct_order(
    first_chars: &mut Peekable<Chars>,
    second_chars: &mut Peekable<Chars>,
) -> bool {
    let mut first_exhausted = false;
    let mut second_exhausted = true;
    loop {
        let mut first = String::new();
        let mut second = String::new();

        first.push(first_chars.next().unwrap());
        second.push(second_chars.next().unwrap());

        if first == second {
            continue;
        } else if first == "]" {
            return true;
        } else if second == "]" {
            return false;
        } else if first == "[" {
            // convert packet's integer to a list by
            // continuing to remove the [ on this packet's list
            // e.g. this:other [[1]:1 -> [1]]:1 -> 1]]:1
            while *first_chars.peek().unwrap() == '[' {
                first_chars.next();
            }
            // converting other integer to a list gurantees other list
            // will have a length of one. If this list parses ] is it empty
            //  e.g. [[]]:1 -> []]:1 -> ]]:1
            if *first_chars.peek().unwrap() == ']' {
                return true;
            }
            // replace the parsed '[' with the inner list value
            first.clear();
            first.push(first_chars.next().unwrap());

            // can only convert a single integer to a list, otherwise exhaust
            // not exhausted e.g. [1],[2]:1,2 -> ],[2]:,2 -> [2]:2 -> ...
            // exhausted e.g. [1,2]:1,2 -> ,2]:,2 -> -> stop parsing
            if !(*second_chars.peek().unwrap() == ',' && *first_chars.peek().unwrap() == ']') {
                second_exhausted = true;
            }
        } else if second == "[" {
            // See comments above for equivalent reasoning
            while *second_chars.peek().unwrap() == '[' {
                second_chars.next();
            }
            if *second_chars.peek().unwrap() == ']' {
                return false;
            }
            second.clear();
            second.push(second_chars.next().unwrap());
            if !(*first_chars.peek().unwrap() == ',' && *second_chars.peek().unwrap() == ']') {
                first_exhausted = true;
            }
        }

        // if needed, build multi char integers
        while char_is_num(*first_chars.peek().unwrap()) {
            first.push(first_chars.next().unwrap());
        }
        while char_is_num(*second_chars.peek().unwrap()) {
            second.push(second_chars.next().unwrap());
        }

        // will Err when comparing a single char integer with a multi char
        // integer due to trying to parse ',' e.g. 10:1, -> 0:, -> ",".parse::<usize> -> Err
        let first_num = match first.parse::<usize>() {
            Ok(num) => num,
            Err(_) => return true,
        };
        let second_num = match second.parse::<usize>() {
            Ok(num) => num,
            Err(_) => return false,
        };

        #[allow(clippy::comparison_chain)]
        if first_num < second_num {
            return true;
        } else if first_num > second_num {
            return false;
        }

        if first_exhausted {
            return true;
        } else if second_exhausted {
            return false;
        }
    }
}

fn char_is_num(c: char) -> bool {
    c != '[' && c != ']' && c != ','
}

#[cfg(test)]
mod tests {
    use crate::is_packets_correct_order;

    #[test]
    fn test_is_packets_correct_order() {
        let mut p1 = "[1,1,3,1,1]".chars().peekable();
        let mut p2 = "[1,1,5,1,1]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[1],[2,3,4]]".chars().peekable();
        let mut p2 = "[[1],4]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[9]".chars().peekable();
        let mut p2 = "[[8,7,6]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[4,4],4,4]".chars().peekable();
        let mut p2 = "[[4,4],4,4,4]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[7,7,7,7]".chars().peekable();
        let mut p2 = "[7,7,7]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[]".chars().peekable();
        let mut p2 = "[3]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[]]]".chars().peekable();
        let mut p2 = "[[]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[1,[2,[3,[4,[5,6,7]]]],8,9]".chars().peekable();
        let mut p2 = "[1,[2,[3,[4,[5,6,0]]]],8,9]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[],[3,[],4,[2,1,3]],8,7],[0,7,[[5,2,1,0],5],9],[10,[[],4,[7,2],2,8],[[2,1,0,5]],[[7,2],0,[],[3,5,1],5]],[0,10,2]]".chars().peekable();
        let mut p2 = "[[],[3,[[6,3],[2],[8,3]],[[1,6],[3,9,1],[]]]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[3,2,4],[1,[2,3,[5,1,8],7,9]],[[4,[]]]]"
            .chars()
            .peekable();
        let mut p2 = "[[[],[],6],3]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[],[6,7,8,0,5],1,8,3],7,4],[[9,10,2,9,5],5],[10,[3],[[],[9],1,3]]]"
            .chars()
            .peekable();
        let mut p2 = "[[7,[0,[1,2,7,0],3,[10,4,10]],[[10,1,8,5],5,0,[3,8,4,0],[4,9]],9,7],[[[9,2,3,9,0],5,[8]]],[[7,[7,9],[4],[0,4,5,2,10],[0,6,3,0]]],[5,[],[9]],[10,3,8]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[7,7],[],10,[0,8]]],[7,[[],2]],[3,10,[[1,0,10,7],[6,10,5,2],[8],5],[[]]]]"
            .chars()
            .peekable();
        let mut p2 = "[[[],[[4],9],9,[[3,1,7,9,3],[2,5,4,3,9],[7,5],[],[4,0,1,2,4]]],[[2,6],9,2],[],[[5,2,4],3,[2,9,9]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[5],[[0],[0,7,0,6,5],[2],[4,3,5,8]],1]]"
            .chars()
            .peekable();
        let mut p2 = "[[8]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[]],[[7],2,6],[[1,10],10],[[],[],[[10,2,4,7,9],0,[2,7]],8,1]]"
            .chars()
            .peekable();
        let mut p2 = "[[2],[4,9,3,7]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[[[3],[],[],[1,0,4,0,2],0],[10],4,[10,[7,3,4],0,[10,5,6,2],0],7],[6,[4,5,[2],1]]]"
                .chars()
                .peekable();
        let mut p2 = "[[],[9],[[0,[],0,[6,3,1,0],[9]],[3,[10,9,5],3]],[]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[4,9,[1],0,8],[[0,1,10,2,0],10,[6,3,3,9,4],[],4],4],[2,7],[[[6,10],[1,6,8,1],[4,1,5]],3,[2],[[5],8],7]]".chars().peekable();
        let mut p2 = "[[4],[[[6],[],2,9,[8,6,1]],10],[2,[2,[5,6,2,0,7],[8,3,9,2],0,[1]]]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[[[2,7,3,4,5],3,[0,8,5,5],[3],[6,2,3]]],[[[],[10,0],[2,0,0,5],[4,4,9,1],[9,4]],7]]".chars().peekable();
        let mut p2 =
            "[[5,1],[10],[[[4,6,7],10,[8,0,10,6]],[]],[2],[[[1,10,0,9],9,[10,0,7,10,10]],[4]]]"
                .chars()
                .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[2],[7,[],[3,[],5,[4]],9],[[],6,[8,[4,8,3]]],[5,10,[[6,7,10,2,6],10,0,[8,2],[1,7,9,3,8]]],[0,5,10,3]]".chars().peekable();
        let mut p2 = "[[9,[],[],[0],[1]],[[],[[1,0,4,10],7,[8,4,5]],5],[[],[[4,0,7,2,4]]],[[[8],10,1,2,[]]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[],[],[3]],[4,5,7,1,[[9,6,1],5]]]".chars().peekable();
        let mut p2 = "[[0,[[3]]],[[[5,2,0,9,0],6,[]],2,0,[[8,7,2],0,[0,2,1,1]]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[3,[[]],[3,5],[[0,10,10],[]],5],[[10,2,6,1,5],6],[]]"
            .chars()
            .peekable();
        let mut p2 = "[[[[4,5,6],5,2,3],[[2],[9,6,8,4,8],[5,8,6,3,2],8],9,[6]],[[2,[7,9,0,9]],[8,[10,4,8],6,[7,10],3],[[1,0,5,1,8],6]],[9,4],[[5,10,[3,9,2]],[],7,[],5],[2,6]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[5,9],[5,[],[0,1,[0,7,7],[10,3]],8]]".chars().peekable();
        let mut p2 = "[[[],[[10,5,2],[]],[10,[6,6]],[1,[1,3,3],[2],6]],[8,1,[3,7,[8,8,1]],[[8,10,10,3],0]],[[[4,6,9],0,[],7,[9]],9,[[6,4],[5],5,2,[2,5,10,1,9]],5,[[2,3,10,7],[9,6,10],[2],[3,4,3],8]],[[2,7,5,3],6,[10,[10,7],9,9],4,2]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[4,3,8,9,8]".chars().peekable();
        let mut p2 = "[4,3,8,9]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[2,6,1,7],[4,[],3,9,4]]".chars().peekable();
        let mut p2 = "[[],[[9],[[3,0,7,10],7],[8,3]],[0,[[7,10,8,9,5],[7,6],[6,8,9,10,2],[7]],0,[[4]],[1,4,4]],[],[[9,2,[1,9,9,3],[9]],3,3]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[3,6,[],5,10],6,8,[0,[1,7],6,5,6]],[]]"
            .chars()
            .peekable();
        let mut p2 = "[[3,9,8,9],[7,[0,[1,7,9,6,5],0,[6,5,2,1,5],1],[[4,2,8],10,[4,8]],[[10,10],9,[5,4],5,1],8],[],[10,[]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[7,10,[6,5,0]],[[7,2],[[7,0],4,7,[]],[[9],5,[10,0,6,10,10]],[6],1],[[[3],[]],1]]"
                .chars()
                .peekable();
        let mut p2 = "[[[],[[0,7,9,6,10],[4],2,8],[7],3],[[[],8,5,[9,6,8,9,7]]],[4,10,[7],1,5],[],[0,7,[4],[7,[9,3],6,7]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[0,0,10,3,0],[[10,6,10,9],[],6,2],2,[],3],[[],4,[4,4],[10,10,[6,3],3,8],7],[[[],1,3,[]],[9,3,[8,6,8,7,5],2]]]".chars().peekable();
        let mut p2 = "[[[8,10,4]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[10,8,7],[7],0,2,4],8,8,6,[[0,3,3,2,3]]],[2,[6,[0]],[[]],8,[[],8,[5,0,1],[6,6,6,10],[2,0,10,1]]],[9,4,4,[],[[10,1],[5,8,1,5,8],8,10]],[[1,2,[8,1],6,6],[3,8,7,[4,4,10],10],[],9,[4,5,[3]]],[3]]".chars().peekable();
        let mut p2 =
            "[[],[[[7,0,2],[],[6,1,8,4],[10],[]],10,8],[[[7,7],1,4,[8,2,1,4],4],[1,[2],10],9,1,4]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[10,[0,7,[],3,[1,6]],[[2,4,5,4]],[]],[],[6,6,[[2,6,7],7,[5],[8,4,10,4,8],[0]],[10],[]],[[[],[6,0,9,10,2],8,[0]]]]".chars().peekable();
        let mut p2 =
            "[[[6]],[[3],[[]],[[0,6,8,9,5],[7,9,10,2]]],[],[[[1],[9],5],9,[[],[0],5,1,[5,0]],5]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[6,[0],[2,7,4]],8,1,[]]]".chars().peekable();
        let mut p2 = "[[4],[[[5],[10,2,1,8,9],5,[4,0,2,9,9]],0,[8,[7,2,0,1,0],[0,6],8,[7,2]],8]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[]".chars().peekable();
        let mut p2 = "[[[[4,4],[5],[8]],8,8],[10,[[7,7,5,0],9,4],[],[6,4]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[4,[]],[[9,8,2,2,3]],2,[]]]".chars().peekable();
        let mut p2 = "[[],[2,1,[[3,7,4],4,[2],2,[]]],[7],[[0,[7,2,6],[2,5],1,[8,4,10]],[[6,6,8,1],7,[],1,0],[10],10,[0,[9],6]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[6,7,[5,[8,4,6]],10,10],[7],[8,[[8,8,6,10,6],[5,8,4]],[1]],[4,2,[1,[0,3,1,7],[9,1,4,2],[],[4,6,4]],4],[[0,4,2,[10,1,1,9],[8,5]]]]".chars().peekable();
        let mut p2 = "[[],[[[],10,9,[10,8,5,3],[9,7]],3],[[3,10,6],[4],10,10,7],[1,[10,[1,2,9,8],0,10],[[4,9,6,10]],2],[0]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[[],[[8,5,5,8,5],[4,9,1,10,7],[1,5,7],[10,6,5,6],2],[[1,5,6,7,10],[6],4],[[4,3,5],2,7,6,[4,0]],5],[[2,8,6,[8,10],[1,10,10]],[3,[8,9,6,8],8,[0,0,7,9]],[[9,5,6,0],[2],[8,6,4,6,6],[]]],[2,[[]],[[],8],4]]".chars().peekable();
        let mut p2 = "[[5,8,[0,9,0,7,[0,8]],[7,[7,7],[9,4,7,6,7],[8,0]],[[5],[10,1,2,5,4],[],[1,2]]],[8,7],[3,[[2],8,0,5],9,[]],[8,0,4,5],[[]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[3,1,[5,5,2,3,1]],[[],[1,1,3,1,0],3,[10,9,4]],0,7],[[[2,2,4,1,10],[],9,2,[6]],7,[[2],3,[4,5,0,3]]]]".chars().peekable();
        let mut p2 = "[[],[2],[]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[10,[4],[3,[5],1],3,6],[[7],[[6]],[[2,0,2],[],[0,5],[4,7,2]],1,10],[[],[5,0,0],5,7],[6,[[9,2,0]],[]]]".chars().peekable();
        let mut p2 = "[[[6,3,[8]],0,2,1,[3,[3,10,10],[7,3,3,6]]],[[[6,1,9,10],[6,6,4,5,9],[9,0,2,2,3],7,1],2,[]],[],[],[[2,[10,9,5,10,1],9,9],[3,6,5,7],10]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[6,[3],[8,6]],7,[7,[2],6,[6,6,1,0,6]],4],[],[[[5,0,3,5],1,1,[9,4,10,5,1],[1,9,5,3,8]],[[10,1,2,8]]],[8,[],[[3,2,0,2],10],1],[]]".chars().peekable();
        let mut p2 = "[[[7,[],9,3],[1,[10,7,0,2],1],8,3],[10,7,[4,10,9,[2]]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[7,7,1]]".chars().peekable();
        let mut p2 = "[[6,[5,[8],7,9,[9,3,2]],[9,10]],[[1,5,[5,6,8,7]],[]],[],[[7,6,7],7,[]],[[[8,2,9],[10,9,3,6],[0,1,2,4,1],4,[4]]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[0],[[9,3,3],5],[[[4,0,2,8,3],10,1]]]".chars().peekable();
        let mut p2 = "[[[[10],[8]],1],[1],[5,[[10,2,6,9],[0,10,9],0]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[4],[[8,5,10,[8,8,7]],4,[],[7],9]]".chars().peekable();
        let mut p2 = "[[[[7,6,0,5,6],[],[],[3],8],[3]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[3]],[3,[4,[4,10,5,0],7,2,[8,5,6]],[4,[1,3,10,2],[9,8,3,10,6],[2],2],[],[[0,4],2,1]],[7,[],[[3],[10,10,1],5],[[],[5,2,1,8,8],[7,0,6]],[6,[3,1,2,6],2]],[],[[[2,9,6],[1],[3],[]],6,[[9],[1,3,0,4],[2,9,10,6,3]],3,1]]".chars().peekable();
        let mut p2 = "[[[[],6],[],[[4]],[]],[1,7]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[],[[3]],[],8]]".chars().peekable();
        let mut p2 = "[[4,[7,4,[0,1],2],5,2],[[],[7,2,[6,5,1],4,[1,2,10,5]]],[[0,[2],1,[]],9]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[2],[[9,5,9,10,1],1],[],[3,[]],7],[],[1],[]]"
            .chars()
            .peekable();
        let mut p2 = "[[0,7,[[0,6],[6,6,0,4],[10,4,6],[9,3,3,2,10],7],5,4],[],[[[7,9]],[8],[7,[],[3,3,2,7]]],[[],4,3]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[],[3,1],[10,5,1,3],[[4,9,3,[7,7,8],[9,4,2]],3,[4,0,8,[]]]]"
            .chars()
            .peekable();
        let mut p2 = "[[7,[[8,2,4],7,[10,9],[]]],[[],[[1,3],5,[6,7],[6],10],9],[],[[],5,4,[[1,8,6,3,3],1,4,6,8]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[3,[]]]".chars().peekable();
        let mut p2 = "[[[6,[],[]],[6],[9,[1,6],7],[[5,6,5],[5,3,2,8,8]],[4,0,[]]],[[],[2,[6,3,9],2,4],6],[3,[],[4,4,[1,6,2,2,2],[7,10,1,9],[9,10,4,2]]],[[[5],[9,7],[],3]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[1,7,[10,8,8,4,2]]],[5,[[5,3,8]]],[0,6,10],[[2],2],[[],2,0]]"
            .chars()
            .peekable();
        let mut p2 = "[[[[],[0,5],[4,2],0],4,8,0,3],[[2]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[],[]],[7,6,7,[0,4,5,[9,2,7,4,7]],7],[0,0],[]]"
            .chars()
            .peekable();
        let mut p2 = "[[[9,0,[0,2]],[[7,10,1,0],5],4],[[3,[4,1,5,3]],7],[[[8,0,10],4,[10,3],[1,0,10,3,1],5],5,[7,9,[10,8],8,6],[5]],[[[6,0,9,0,7],4],8,[],[9,[2],6,1],1]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[],[10,3,[6,[3,3],[6],[0]]]]".chars().peekable();
        let mut p2 =
            "[[8,6,4],[[3,[10,5,0,2],[2,9,6]],4],[2,0,[[7,0,5,6,0],10,8,3],[0,10,1,[3,7]],9]]"
                .chars()
                .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[2,[6,[],8]],[[5,3,[]],[10,7,[0,0,0],[3,10],1],[[1,3,5,4],[2,10,2,7],0,6]],[[10,[4,9,6]],7,6,[]],[[10],[8,[],4],[[8,1,5,0,4],2,3],[1]],[5,5,[]]]".chars().peekable();
        let mut p2 = "[[[],[[7,4,4]],0],[3],[[[8],0,1,7],[[8,6,0],8,[6,4,0],[9,4],3],5],[2,9,[],1,[[0,6,8,1],10,[]]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[[[]]],[[],1,[10,0],[[],10,[]],5]]".chars().peekable();
        let mut p2 = "[[10,9,[[],7,[],2,[1,1]],[],[[9,3,3,9]]],[],[],[3,9,[]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[9,[6,7]],[[5,6,4,2],0,3,0,6]]]".chars().peekable();
        let mut p2 = "[[8,2,0],[9,5,10,0],[[9,5,[5]],9,2,[[9,5],10,8,[9,2,8,9,9],[]],5],[5,8,[8,5]],[[[8,9,6,9],[0,4,7,9,9]],[5,[10],4,10],[2,1],7,[4,7,[7,6],10,[4,5,9,0,7]]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[],3],6,[[9],[3,7,3,1],[5],[6,6,4]],9,0],[1,0,[9,7],2,[[],[0,1,1],2,[],2]],[[3,3,[10]],[[7,2,5,9,7],[1,0,7],4,8],3,7,10],[[[9,1,4,1,5],0],4,9,[6,0],[]]]".chars().peekable();
        let mut p2 = "[[0,[5],2,[1,[1,9,4,10,9],1,[10,4,8],[10,3]]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[3,6,4,[6,[1,3,7,9,5]]],[[[]],[],[[]]]]"
            .chars()
            .peekable();
        let mut p2 = "[[[5,[3,10,1,2,5]]],[4,[4],2,[4]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[3,[],8]]".chars().peekable();
        let mut p2 = "[[],[[[10,0,8,8,10],[3]],0,[[0,7,6],[8,7,0,9,4],10]]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[8,5,[],6],9,[9,5,9,[]],6,[]]]".chars().peekable();
        let mut p2 = "[[4,10,[],6,10],[[5,3,[3,7,10,3,5]],[9,0],3]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[9,2,5,7,8],6,2,1,[]],[[2,6,7,4],[]],[3,8,[8,2,0,2],[6,6,10,4],[2,4,0,3,8]],8,[4,[0],[7,3]]],[[0,[4],[10,6,10,8,8],[3,10],[1]],[],[8,[6,4,9],[],2,[10,6,0,8]],6,[3,3,5,[5,5,5,1]]],[],[[9,[0,3,10]],[7,8],0]]".chars().peekable();
        let mut p2 = "[[0,3],[[[3,3,4,2,2]],[],4],[[7,[5,5,2,3,10],8],9,3,[]]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[1,8,4,[9],[4,[9,0,9],3]]]".chars().peekable();
        let mut p2 = "[[6,3,[[0,2,4,7],0,[1,8,0]],0],[[[],[7,9],[7],[9,5],[8,4]],[4,[3,10,4,6,0],9]],[[[8]],[],[]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[6,[3],[8,2,6,6,6],3,[7]],[10,[6,7,3,3],9,7,6]],[[10,2,10,9,[8,2]],[],[6,[1],10]],[[],3,7]]".chars().peekable();
        let mut p2 =
            "[[[8,3,[4,7,1,0,1],[9,4,1,6],9],[9,8,[0,1,7,4,2]],[2,9,[8,2]],8,[[8,8,2,0],[]]]]"
                .chars()
                .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[8,[],[[5,3,0,0],[9,8],7],0]]".chars().peekable();
        let mut p2 = "[[1,1,[[],[5,10,0,2,7],[4,6,1,7,4]],5,7],[[[]],1,6],[[[],10,[10,5,8,5,3],[7,1],1],[9],10,[[7,7,3],10,8,[0,3,4]]],[7,10],[5]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[2,1],7,4,[7,7,5,6,[10,1,2,9,8]],[1,[],[9,1,6,5,4],[2,5,2,6]]],[1,[[],[4],[0],0],9,[[1],[5,10,3],[2]],[]],[5,[[1,3,8,1]]],[2,1,4],[[4,[],4,3,[3,10]],8]]".chars().peekable();
        let mut p2 = "[[],[9],[1],[10]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[[1,[10,6]],2,3,10],[5,[9,10,6,9],7],[[[],[10],[1,1,9,9,3],7],7,[5,10,1],6],[]]"
                .chars()
                .peekable();
        let mut p2 = "[[],[9,8],[6,10,8]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[1,6,7,6],[7,5]],[[],8,0,1,[9,4,2]],[10,[9,6,1,3,6],5],0,8]]"
            .chars()
            .peekable();
        let mut p2 =
            "[[3,5,[[],[9,2,8,5],1,[]],0,[]],[9,[[10,6,2,6,7],[],[10],3,[10,0]]],[7],[6,0,7,[]]]"
                .chars()
                .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[7,[4,3,6],10,[7,3,[2,3,2,6],[5,6,1,2,4],6],[[7,0,3]]],[[[6,3,10,6,6],[2,6,3],[0,0]],[[]]],[],[]]".chars().peekable();
        let mut p2 = "[[5,[[7,7,1,3],[],2],[[0,8,6,6,2],[0,7],0],2]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[6,9]]".chars().peekable();
        let mut p2 = "[[[3,3,[5,3,5]],2]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[3,[[5]]],[[[],[0,4,6,3]],[4,2,7,[3,4,8]],1],[[[],[7,4],[9],7,4],7,3],[],[2,5,[[0,8],[3,6],10],1,[]]]".chars().peekable();
        let mut p2 = "[[7,[[1,0,9],[8,8,9,2,9],3]],[5,8,2,[9,[10,3,7,6]]],[0,[[1,4,8,1,9]],[8,6,[9,10,7,6],[9,8,6,10]],2,[6,4,[4,2,2,1,9]]],[10,[[6],10,[3,1,9],10],4]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[],[4,[0,4,7,3],[2]]],[[6,[10,9],6,1]],[2],[8,[[],1],[4,[3,4],2,[8]],8,9],[2,[[],[2],[4],[6,5,3,1,6],2],[8]]]".chars().peekable();
        let mut p2 = "[[4,7]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[3,[],[[10,10,10,0,6],[0,2,6,3],8,9],[[],[1,6],2,[9,5,7]],[4]],[[[9,6,5,7],0,[3,10]],4]]".chars().peekable();
        let mut p2 = "[[[5,[],[],[1,9,0,4,0]],6,7],[[[],10,[],[9,2,4]]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[],6,[[]],1],[3,7,[5,2,[3],8],[[4,6,10,2,7],[1,4,10,3],[2,1,2,9]],[8]]]"
            .chars()
            .peekable();
        let mut p2 =
            "[[],[6,8,[[4,5,2,4,1],[10]]],[[[1,4],[],[2,6,3,3,9],2]],[[2,1,2,[4,8,4,2]],[[],4]]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[6,10],[10],[[[0,5,5,7,2],0],[7,[0,6,5]],[[1,1,3,3,2],9]],[10,8],[[8],[[6,3],4],[[1,7,2],10]]]".chars().peekable();
        let mut p2 = "[[5,8],[0,3,10],[0]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[8,2,7,9]".chars().peekable();
        let mut p2 = "[8,2,7,9,3]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[4]],[[[3,9,3]],[]]]".chars().peekable();
        let mut p2 = "[[],[[2],2,[0,[2,5,9,8],4,8,7],9,9],[6],[0,[[3,3,1,8,8]],4,[1,1,[]]],[7]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[[[],6,5],[5,9,[2]],2,[[0,8,8,8],6,1,4,1]],[0],[3,4,0],[[[6],[],6,[5,1,4,1,9]]]]"
                .chars()
                .peekable();
        let mut p2 = "[[9,3,10,4,[6,3,8,[6,5]]],[[]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[],[[[7,5],[3,6,7],4,[0,6],[9,6]],[[6,5,7,6,6],[3,3],[8],[7,4,7,8],[10,1,5]],[[6,7],[3,9,7,7]],2,[[6],[10,0,0]]]]".chars().peekable();
        let mut p2 = "[[[4,4,8,[9,8,9,8],0],5,[0,5,[2,10,5],[5,8],[7,0,0,6,3]]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[],[10,[7,[6,5,4,7],0],[],[[],[1]]],[9,6,1,9,9],[[[8],[3,3,3,7]],[9],4],[8,4]]"
                .chars()
                .peekable();
        let mut p2 = "[[],[[[2,9,3]]],[[],[[],[]],9,[[],[]],2],[7]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[9,[[],0,[],[6],6]],[4,[[5,3,10],8],10,[10,[10]]],[[5,[5,6],[0],7,8],1,[[1,3]],[[0,0,3,9,3],4]],[[4,[9,9],0],[4,[0,4,9,6,10],7,1],[[],4,10,[0,2,8,3]]],[[[10,8]]]]".chars().peekable();
        let mut p2 = "[[[2,9,8,[4,7,9,8]],[[5,7,0,9,4],[],[7,5],6,6],6,[[5,3,1],7,[2,1,5,0,10],1,[1,8,5]],[]],[],[10,3,10],[[10,2,[0,5,4],[8,3,0],8],[4,[],3,[9,1,4,4]]],[9,6,6,[[10,7,0],[0,10,2,4,3],[9,7,7,9,10],[6,6,8,10,5],7]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[3,8,[]],[8],[2,3,[5,2,[8,9],[5],[]],[[7,2],[8,2,6,4,5],[3,4],[2,3],5],8],[[6,7,2,2]],[[5,[4],[3,9,1,2],[7,9,7],[10,1,10]],[6,[4,3,5,1],3,[10,10,7,5,6],7]]]".chars().peekable();
        let mut p2 = "[[3,[[2,2,0],8],[[3,4,4],10,10]],[8],[[[7],0,10,4,[3,8,5,9]]],[6,8,[],3,4]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[6]],9,[0,[],[5],[5,7,1,4]],9,[3,4,1,[2,10]]],[[5],[]]]"
            .chars()
            .peekable();
        let mut p2 = "[[7,[3,[10],6,3]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[5,[8],[7],2],[4,8,[[8],[5],[10,3,8],3,[4,3,1,0]]]]"
            .chars()
            .peekable();
        let mut p2 = "[[6,0,[[]],5,6],[],[2,[[2,3,5,4],[6,6],7,4,1],[9,[2]]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[],[[],[8,6,5,10],[2,[0,4,9,4,1],[10,1,0,2,1]]],[2,[],[[7],[10,5],[10,0,2]]]]"
                .chars()
                .peekable();
        let mut p2 = "[[7,2,0,7,[10]],[]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[8,[2,9,3],[10,9,10]]]]".chars().peekable();
        let mut p2 =
            "[[[6,3,3,[3,6,1,4]]],[[],[[0,9,8],6,[],[4,1],0],[[],[7],[10,0,6,3,7],1,[3,3]],[8]]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[[2,6,7,6,[6,8,8]],10,0,1],[]]".chars().peekable();
        let mut p2 =
            "[[[],[],7,[[10],9,[6,8,0,7],8,[2]]],[[],[9,9],9,[[2,6],4,3,5]],[[[3],[]],[[8,2],1]]]"
                .chars()
                .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[]]]".chars().peekable();
        let mut p2 = "[[[5,4,0]],[[[1,7,9],[]],[3,3,1,7,3]],[0,[2,[],10,2],[[]],[7,2,[7,0,10,3,8]]],[1,[]],[[6,10,[5,8,5,0,9],[9,1],4],[4,[4,0,1,7]],8,8]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[5,6,5,[0,7,6,4],9],8,[8],[[2,7,3],4,10,7,[5,10,10,10,1]]]]"
            .chars()
            .peekable();
        let mut p2 = "[[6,1,[[1],[10,2,6],[7,4],[6,4,3,3,5],6],[0],9],[],[5,[[6,0,9,6,2],5,10,1,9],2,[[0,6,1,4,10],3,[]]],[[5,7,[7],[6],3],[[3,0],6,[1,10,3,10],4,4],7]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[[2,[5,2]]]]".chars().peekable();
        let mut p2 = "[[6,1,5,[[3,7,2],[],[0,0]]],[6,8,1,9]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[4,[],[4,5,6]],[],[2],[2,[8,1],5,[8,1],4]],[[6,[7,6,4,2,0]],[9,[7,3]],[],[9,[10,8,0],[],5]],[6,[8,[8,4,3,10]]]]".chars().peekable();
        let mut p2 = "[[[0,[5,10,4,7,8],[3,0,6,7],10,4],10,5,7],[1,[],8,9]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[5,[7,2,9]],[7]]".chars().peekable();
        let mut p2 = "[[1],[[],3,[[0,5,1],[9,6,9,10]],[7,8,10,9],[0]],[[5,[9],4,[0,9,4,0]],[],[[7,6],10,[8],6,1]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[7,5,2],[3],4,2],[[[10,9,2]],[[5,3,5,7],7,[6,0,0],0],[[6,10,1,5],[]],[0]],[[[8,5],[6,0,5],6,8,[4,8,10,5]],7,[[10],[],3,[]]],[],[[[8],1,[3,7,8]],[[9,7,7,8],[8,4],[6,5,3,9,10],[8]],[[7,4],[1,4,5,9]],[5],6]]".chars().peekable();
        let mut p2 =
            "[[4,7,4,4],[2],[],[6,[[9,2,6,7],[8,4,9,9,0]],[5,7,[8,4],[8,6,9,3,8],[8,9,7]]],[]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[10,2],3,[],4],[[2],3,2,8],[8,[1],10]],[[[10,1,2,8]],[0,[10],[2,9,3,7]],[5],[1,1,7]],[[6,[9],[4],4,[6,8,5,4]]],[[4,[],[9,5,5],[1],[9,10]],[],[2]]]".chars().peekable();
        let mut p2 =
            "[[],[[[7]],[[1,5,10,5],[10,2],6,[],[9]],1,[]],[],[[4,9,[4,2],[7,8],[5,9]],[6,8,4,5]]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[[1,3,[10,7,2,6],9]],[[2,5,[1,7,5,7],0,4],7,[5,[6,6],[]]],[[4,[2]],8,2],[[[5,1]],0]]"
                .chars()
                .peekable();
        let mut p2 = "[[[],[[2,10,9,1],[8],[4],[1,6,8,1]]],[9,3],[7,[6,10,[10,0],[9,5,7],8],[[0,0,7,9,6],9],8],[0,0,[3]],[1,2,0]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[2,[[3,1,1],8,[6],[8]],[[1,0,6],[7,4,8]],4],[[],4,7,[6,9,[5,8]]]]"
            .chars()
            .peekable();
        let mut p2 =
            "[[[]],[[3],10,[],0],[[[3,5,3,5,1],[9,7,8,9]],[[3,1,10,7,4],[],[9]],[[1,0,8,7],2]]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[10,[[1,2,4,8,8],3,1,4],6,[[8],[10,6,0]],[]],[[6],[9,[],3,4],[[10,4],5],9]]"
            .chars()
            .peekable();
        let mut p2 = "[[[3,[10],4,[0,0,3,5,6],[0,1,10,3,6]],8,[],9],[]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[5,9],2,[],[9,[],4]],[0,[9,[5],3]]]".chars().peekable();
        let mut p2 = "[[],[2,[],8],[[0],[]],[[[3,5],[3]]],[[[6,8,0],[2,10,10],[7,0],5,[3,3]]]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[6,8,6,[0]],[]]".chars().peekable();
        let mut p2 = "[[],[[[10],10,8]],[],[[],1,5,[[7,2,0,0],[],[]]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[3],[10,10,6,0,0],[],[5,0,3]]".chars().peekable();
        let mut p2 = "[[[[6,1]]],[9],[0],[[[9,4],8,3,7],2,4,[]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[2,6,1,7],[[[1,9],4,1,[3,8,0,4],[2,6]],4,5]]"
            .chars()
            .peekable();
        let mut p2 = "[[[10],[],0,[[3,5],6,4],[]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[],10],0,[[3,0,7,1]],4,2],[[[2],[7],[1],[8,6,4,2,9]]]]"
            .chars()
            .peekable();
        let mut p2 = "[[[],8,[]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[3],[10,10,6,0,0],[],[5,0,3]]".chars().peekable();
        let mut p2 = "[[[[6,1]]],[9],[0],[[[9,4],8,3,7],2,4,[]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[2,6,1,7],[[[1,9],4,1,[3,8,0,4],[2,6]],4,5]]"
            .chars()
            .peekable();
        let mut p2 = "[[[10],[],0,[[3,5],6,4],[]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[],10],0,[[3,0,7,1]],4,2],[[[2],[7],[1],[8,6,4,2,9]]]]"
            .chars()
            .peekable();
        let mut p2 = "[[[],8,[]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[],[0,[9,[6,8,2,2],[9,9,7,3,7]]],[[7,[3,4],4,[5,4,4,2],4]]]"
            .chars()
            .peekable();
        let mut p2 =
            "[[[[9,7,1],10,0,6],[],[2,4,5,[7,5,2,6,1]],9,[3,[3,3,8,10],[7],[1,8,6,1,2]]],[0,7]]"
                .chars()
                .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[]]]]".chars().peekable();
        let mut p2 = "[[[[9,2,3,10,4],[],1,5,[2,7,3]],6],[6,2,[[],0,7,[]],[2],[[],[],[],[4,5,9,0],0]],[[0,0,0,3],0],[[[1,7,4],[3,6,10],[2,2],9]],[1,[[],2,10,6],[[7,5,0],[3,3,2],6],0,4]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[[1],2,[[10]],[4]],[8,[[],[],[7,2,2,10]],6],[]]"
            .chars()
            .peekable();
        let mut p2 = "[[[0],2],[[[7],1]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[3,7],[7,4],[9,5,10,10,3]],4,[7,10,6]]]"
            .chars()
            .peekable();
        let mut p2 = "[[]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[4,0,1,7,1],8],[8],[]],[[[1]],9,7],[[0,6],1],[[9,[10,2]],[]]]"
            .chars()
            .peekable();
        let mut p2 = "[[10]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[5,3,[]],[9,9,[[7,7],6,[3,7,6]],[10,[6],[9,3,6,8],10]]]"
            .chars()
            .peekable();
        let mut p2 = "[[[[5,2,0],[2,0,5,6,5],[9,2,5],0]],[6,2,[[7,10],[],[8,8,4,7,6],[4,10,2],[0,8,1]],[[8]]],[[[7,3,3,9],[3,1],[3,6,2,8]],[[]],0,[[1,9,9],8],[1,4,[8,10,3,8,9],[5,3,2,8,2],6]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[3,[5,5,[1,0,10],[0,2,8,8]],10,[]],[[[2,5],8,8,[2]]],[4],[6,3]]"
            .chars()
            .peekable();
        let mut p2 =
            "[[],[6,[[3,3,1,0],1,[10,4],7,[3]],[0,1,[5,6],[]],10,1],[7,[1,2,6,[7,5],[1,3,3]]]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[]],[[[3,5,0,9,9],[10],[3],9],5,[4],[3]],[],[[9,10],[[5,0,5,1],[0,8,2,5,8]],3,[[5,2,9,1]],[7,[],2]]]".chars().peekable();
        let mut p2 = "[[5],[10,[[8]],0],[10,[]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[0,[[],3,[10,10,8]],[[9,4,1,8],[6],[4]]],[8,[[3,3,5],5,[],2,10],[[10,3,1,4]],3,[1]]]"
                .chars()
                .peekable();
        let mut p2 = "[[],[10,[9,[],[],[5,10,9,3],[5,10]]],[],[[7,2],[4,2,6],6,2]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[1,[2,7,2],[2,7,2],[9,2,7,0,4]],[[7,4,8],[8,8,0],4,[2,4],7],[[4,0],5,[7,2],6,6],[8,4,[4,7,3,9,5],7]],[2],[7,[7,[3,4,10,2],3],10],[7,[],9,2],[5,2,6,10]]".chars().peekable();
        let mut p2 = "[[5,[[],[4,4],1],[],[2,7],4]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[4],[0,[0,10,5,[9,8,1,0],2],5,4]]".chars().peekable();
        let mut p2 =
            "[[[],1,[9,[0],[],[1]],[4,4,5,6,[4,4]]],[],[[[1],[]],1,[[9,10,4,1,7],10,[]],[],10]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[3,1],6,[],0],[[],[4,5,8],[8,0,0],8,[4,7,7]]],[],[9,1]]"
            .chars()
            .peekable();
        let mut p2 = "[[[[10],[8,9,1],[9,10,1]],8],[[]],[]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[9],[[[0,5,6]]]]".chars().peekable();
        let mut p2 = "[[[4,3,6],9,1,[[],5,[9,6,7]]],[2,[1,[4,4,0,9]]],[[[1,4,0,6,7]],8,3],[[[1,2],[4,6],[0,10,10,1],[],[7,2,2,9,5]],3,1]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[8,[[],[0,7,1],[],[4,7,1,10]],3,[0,[2],[],5,[6]]],[],[[5,[0,9,4,5],5],[[],[5,5,1,10],10,7],5,[[5,3,5,10,2],0]],[[[7,1,0,10]],[2,[],[0,3,8,1,1],[3,7,1,5]],8,[[5,9],3]],[[0,3,10,8,10],6,0,[]]]".chars().peekable();
        let mut p2 = "[[[],2,2,6,0]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[0,[[5,2,7,7,2],[3,8,3]],7,9]]".chars().peekable();
        let mut p2 = "[[],[[[1,0,3,6],[3,10],[9]]],[[3,[8,10,0,9],8,[7,2,4],0],[[4],[9,1,4,3],10,3,2],0,2,6],[3,[[4,6],[3,1,2,6,4],[],[7,10,6,8],6],[],3,[[]]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[6,5],[[[1,10,10,9,6],4,[3,1,8,9],10],[[7],3],[[2],[2,4,5,5],[7,4,4],7,10],10,[[9,5,2,6,10],10,[6,10,10,9]]]]".chars().peekable();
        let mut p2 = "[[],[4,[0,[6,6,4,9]],[[3,9],9,[4,10,9,9],6],10,4]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[9,[[1,4,2,7,4],[2,9,5,8,2]],[6]],[[[0,9,3],7,[]],[10,[2,3],7,[4],[1,5]]]]"
            .chars()
            .peekable();
        let mut p2 = "[[[6,6,[7,0,9,0,3],9,1]],[9,[5],10,2],[[],10,[[10,4,6,4,2],3,1,[9,5,9],4]],[],[[10,9,[9,3,4]],[[2],[1,7,4,9,5],[],[8,7],8],[[9,0,9],5,2,[8,0],5],[]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[10,1,10],8],[],[4,[5],[8,10,0]],[[0,10,4,6]]],[3,6,[8,2,6,2]]]"
            .chars()
            .peekable();
        let mut p2 = "[[],[[],[8,[],10],7,0]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[2,0,8,[[9,4,8,5],10,[0,7],[9,0,1,5]],[3,7,6]],[[5,4,2]],[4,3]]"
            .chars()
            .peekable();
        let mut p2 = "[[[],[[]]],[],[7,[],[],[[5],2,[6,1,6]],3],[9,[[0,9,10,8],[8,7,3]]],[10]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[8,3,[0,[0,1,2,3,3]]],[0,7]]".chars().peekable();
        let mut p2 = "[[[],10,[10,6,5,10]],[[[2,4]],5],[[6],2,[[1,5,6,6,1]],[[9,2,2,10]]],[10,[[],[]]],[6,3,[[],2,[7,6,6,3],6],1,7]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[[[],[0,3],[8,6,8]],[9,2],[[0,10]],5],[[[9],[8,1,3,9],4,7]]]"
            .chars()
            .peekable();
        let mut p2 = "[[10],[2,[[9,10,0,3,9],[4,5,9,1]]],[3,3,[0,9,6,4],8,[9]],[[[],[]],10,[[0,1,0],[8,3]]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[4,[[4,1,0],[9,6,3],[],8,[]],[[10,6],[2,0,4,1],0,[8],2],5,10],[[9],6,[],[[10,2,3],[10,0,5],8,5]],[7]]".chars().peekable();
        let mut p2 = "[[[9,[9,5,7,0]],[2,4,[4,1,2,4],[2,8]],[[6],1,[10],[1,4,7,2,10]],3,10],[[8],[8,[],2],[[9,9],6],[[]],6],[[6,[6,9],2,[3,9,10,2],[2,8,7,6]],[[7,6,9,9]],[],8],[2,[4,[1,4,2]],[]],[]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[10,0,10],[3,[7],2],[[[9,9,8,5,10],[1,1,5]],7],[[],[0,[9,2],0,10],[0,10,[4,8,7,6,6]]],[]]".chars().peekable();
        let mut p2 = "[[[[8,7,1],2,[6,1,5,8]],[4,[9]]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[9,4,[3,[7,8],[3],[10],2]],[8],[]]".chars().peekable();
        let mut p2 = "[[[1,7,5,1],0]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[2],2],[9,[7,[8,3,4],4],0],[1,[6,6,[7,9,9],[10,1,9],[5,10,9,7]],[[10],4],[[0,4],[10,10,10,1]],[0,1]],[[[9,1,8,0],[7,4,3],[8]],[],[[2,9,1,4,10],[4,9,3,9],9,2,2],3],[[3,[8,7]],[[]],[[8]],1]]".chars().peekable();
        let mut p2 =
            "[[],[8,2],[[1,1,3],[[7,6,2,10],4],[],[2,4]],[[[],10,[2,2,2],7],[]],[6,5,0,7]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[[7,[0,9,6,0,9],[4,8,0,9]],[[3,9,7,8]],4,1],[4,1,[[],1],1,[4,[1,0]]],[],[[],[],9]]"
                .chars()
                .peekable();
        let mut p2 = "[[[[4],[7,9,4,7,9]],9],[5],[[]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[0,10],2,[5,2,10],5],[],[]],[[[5,2,10,8,9],[10,10,8,5,3]],2,[[],10,6,5]]]"
            .chars()
            .peekable();
        let mut p2 = "[[[0,2,[]],[[]],10,6,[4,10,[9,1,5]]],[0,[],[]],[[[3,0,0],3,[9,0,2],5,8],8],[1,[[1,7,9,8],[0]],2],[[[4,6,0,3,0],[7,6],1,[5,9,1,6],1],7]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[3,7],[7,4],[9,5,10,10,3]],4,[7,10,6]]]"
            .chars()
            .peekable();
        let mut p2 = "[[]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[4,0,1,7,1],8],[8],[]],[[[1]],9,7],[[0,6],1],[[9,[10,2]],[]]]"
            .chars()
            .peekable();
        let mut p2 = "[[10]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[5,3,[]],[9,9,[[7,7],6,[3,7,6]],[10,[6],[9,3,6,8],10]]]"
            .chars()
            .peekable();
        let mut p2 = "[[[[5,2,0],[2,0,5,6,5],[9,2,5],0]],[6,2,[[7,10],[],[8,8,4,7,6],[4,10,2],[0,8,1]],[[8]]],[[[7,3,3,9],[3,1],[3,6,2,8]],[[]],0,[[1,9,9],8],[1,4,[8,10,3,8,9],[5,3,2,8,2],6]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[3,[5,5,[1,0,10],[0,2,8,8]],10,[]],[[[2,5],8,8,[2]]],[4],[6,3]]"
            .chars()
            .peekable();
        let mut p2 =
            "[[],[6,[[3,3,1,0],1,[10,4],7,[3]],[0,1,[5,6],[]],10,1],[7,[1,2,6,[7,5],[1,3,3]]]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[]],[[[3,5,0,9,9],[10],[3],9],5,[4],[3]],[],[[9,10],[[5,0,5,1],[0,8,2,5,8]],3,[[5,2,9,1]],[7,[],2]]]".chars().peekable();
        let mut p2 = "[[5],[10,[[8]],0],[10,[]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[0,[[],3,[10,10,8]],[[9,4,1,8],[6],[4]]],[8,[[3,3,5],5,[],2,10],[[10,3,1,4]],3,[1]]]"
                .chars()
                .peekable();
        let mut p2 = "[[],[10,[9,[],[],[5,10,9,3],[5,10]]],[],[[7,2],[4,2,6],6,2]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[1,[2,7,2],[2,7,2],[9,2,7,0,4]],[[7,4,8],[8,8,0],4,[2,4],7],[[4,0],5,[7,2],6,6],[8,4,[4,7,3,9,5],7]],[2],[7,[7,[3,4,10,2],3],10],[7,[],9,2],[5,2,6,10]]".chars().peekable();
        let mut p2 = "[[5,[[],[4,4],1],[],[2,7],4]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[4],[0,[0,10,5,[9,8,1,0],2],5,4]]".chars().peekable();
        let mut p2 =
            "[[[],1,[9,[0],[],[1]],[4,4,5,6,[4,4]]],[],[[[1],[]],1,[[9,10,4,1,7],10,[]],[],10]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[3,1],6,[],0],[[],[4,5,8],[8,0,0],8,[4,7,7]]],[],[9,1]]"
            .chars()
            .peekable();
        let mut p2 = "[[[[10],[8,9,1],[9,10,1]],8],[[]],[]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[9],[[[0,5,6]]]]".chars().peekable();
        let mut p2 = "[[[4,3,6],9,1,[[],5,[9,6,7]]],[2,[1,[4,4,0,9]]],[[[1,4,0,6,7]],8,3],[[[1,2],[4,6],[0,10,10,1],[],[7,2,2,9,5]],3,1]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[8,[[],[0,7,1],[],[4,7,1,10]],3,[0,[2],[],5,[6]]],[],[[5,[0,9,4,5],5],[[],[5,5,1,10],10,7],5,[[5,3,5,10,2],0]],[[[7,1,0,10]],[2,[],[0,3,8,1,1],[3,7,1,5]],8,[[5,9],3]],[[0,3,10,8,10],6,0,[]]]".chars().peekable();
        let mut p2 = "[[[],2,2,6,0]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[0,[[5,2,7,7,2],[3,8,3]],7,9]]".chars().peekable();
        let mut p2 = "[[],[[[1,0,3,6],[3,10],[9]]],[[3,[8,10,0,9],8,[7,2,4],0],[[4],[9,1,4,3],10,3,2],0,2,6],[3,[[4,6],[3,1,2,6,4],[],[7,10,6,8],6],[],3,[[]]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[6,5],[[[1,10,10,9,6],4,[3,1,8,9],10],[[7],3],[[2],[2,4,5,5],[7,4,4],7,10],10,[[9,5,2,6,10],10,[6,10,10,9]]]]".chars().peekable();
        let mut p2 = "[[],[4,[0,[6,6,4,9]],[[3,9],9,[4,10,9,9],6],10,4]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[9,[[1,4,2,7,4],[2,9,5,8,2]],[6]],[[[0,9,3],7,[]],[10,[2,3],7,[4],[1,5]]]]"
            .chars()
            .peekable();
        let mut p2 = "[[[6,6,[7,0,9,0,3],9,1]],[9,[5],10,2],[[],10,[[10,4,6,4,2],3,1,[9,5,9],4]],[],[[10,9,[9,3,4]],[[2],[1,7,4,9,5],[],[8,7],8],[[9,0,9],5,2,[8,0],5],[]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[10,1,10],8],[],[4,[5],[8,10,0]],[[0,10,4,6]]],[3,6,[8,2,6,2]]]"
            .chars()
            .peekable();
        let mut p2 = "[[],[[],[8,[],10],7,0]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[2,0,8,[[9,4,8,5],10,[0,7],[9,0,1,5]],[3,7,6]],[[5,4,2]],[4,3]]"
            .chars()
            .peekable();
        let mut p2 = "[[[],[[]]],[],[7,[],[],[[5],2,[6,1,6]],3],[9,[[0,9,10,8],[8,7,3]]],[10]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[8,3,[0,[0,1,2,3,3]]],[0,7]]".chars().peekable();
        let mut p2 = "[[[],10,[10,6,5,10]],[[[2,4]],5],[[6],2,[[1,5,6,6,1]],[[9,2,2,10]]],[10,[[],[]]],[6,3,[[],2,[7,6,6,3],6],1,7]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[[[],[0,3],[8,6,8]],[9,2],[[0,10]],5],[[[9],[8,1,3,9],4,7]]]"
            .chars()
            .peekable();
        let mut p2 = "[[10],[2,[[9,10,0,3,9],[4,5,9,1]]],[3,3,[0,9,6,4],8,[9]],[[[],[]],10,[[0,1,0],[8,3]]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[4,[[4,1,0],[9,6,3],[],8,[]],[[10,6],[2,0,4,1],0,[8],2],5,10],[[9],6,[],[[10,2,3],[10,0,5],8,5]],[7]]".chars().peekable();
        let mut p2 = "[[[9,[9,5,7,0]],[2,4,[4,1,2,4],[2,8]],[[6],1,[10],[1,4,7,2,10]],3,10],[[8],[8,[],2],[[9,9],6],[[]],6],[[6,[6,9],2,[3,9,10,2],[2,8,7,6]],[[7,6,9,9]],[],8],[2,[4,[1,4,2]],[]],[]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[10,0,10],[3,[7],2],[[[9,9,8,5,10],[1,1,5]],7],[[],[0,[9,2],0,10],[0,10,[4,8,7,6,6]]],[]]".chars().peekable();
        let mut p2 = "[[[[8,7,1],2,[6,1,5,8]],[4,[9]]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[9,4,[3,[7,8],[3],[10],2]],[8],[]]".chars().peekable();
        let mut p2 = "[[[1,7,5,1],0]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[2],2],[9,[7,[8,3,4],4],0],[1,[6,6,[7,9,9],[10,1,9],[5,10,9,7]],[[10],4],[[0,4],[10,10,10,1]],[0,1]],[[[9,1,8,0],[7,4,3],[8]],[],[[2,9,1,4,10],[4,9,3,9],9,2,2],3],[[3,[8,7]],[[]],[[8]],1]]".chars().peekable();
        let mut p2 =
            "[[],[8,2],[[1,1,3],[[7,6,2,10],4],[],[2,4]],[[[],10,[2,2,2],7],[]],[6,5,0,7]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[[7,[0,9,6,0,9],[4,8,0,9]],[[3,9,7,8]],4,1],[4,1,[[],1],1,[4,[1,0]]],[],[[],[],9]]"
                .chars()
                .peekable();
        let mut p2 = "[[[[4],[7,9,4,7,9]],9],[5],[[]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[0,10],2,[5,2,10],5],[],[]],[[[5,2,10,8,9],[10,10,8,5,3]],2,[[],10,6,5]]]"
            .chars()
            .peekable();
        let mut p2 = "[[[0,2,[]],[[]],10,6,[4,10,[9,1,5]]],[0,[],[]],[[[3,0,0],3,[9,0,2],5,8],8],[1,[[1,7,9,8],[0]],2],[[[4,6,0,3,0],[7,6],1,[5,9,1,6],1],7]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[],[3,[2,0,[3,5],[8,8,4],[8,3,7]],10,4,[[8,7,4],9]],[3,3],[[],[0,8,5,5],8,1]]"
                .chars()
                .peekable();
        let mut p2 = "[[[5,6,[],[]],4,9,[[0,7,0,4,8],[3],9,[3,4,7,10],1]]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[10,4,[7,[6,4,7],5,10,6],[[6,7,1,0,1],9]],[5,9,[[8,2],[5],[2],0,[]]]]"
            .chars()
            .peekable();
        let mut p2 = "[[9,[2,2]],[],[9,9,[[],5],[[]],[]],[],[]]"
            .chars()
            .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[5,3],[[],4],[[[6],5,5,9],[1,7,4,[2,5,1,10,5],2],6]]"
            .chars()
            .peekable();
        let mut p2 = "[[],[2,[[0,9],9,7],[6,[8,7,3,4]],[],[7,1,[0]]]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[4,[[2,7,3],[4,9,7,3],3]]]".chars().peekable();
        let mut p2 = "[[],[[],7,0,8],[[10,[2,10,0,2,4],[9,9,9,5]],9],[[[6,2,10],[9],[2],3],[],6]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[[10],6],[],[],[7,8]]".chars().peekable();
        let mut p2 = "[[[5,1]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[0,1,[4,10]]],[9,[]],[],[4,6,4]]".chars().peekable();
        let mut p2 = "[[1,3,3,[7],2],[[[9,2,8,0,6],[9,2,10]],[[9,5],[6,6],10,[2,9,5,6,0],[7,0,8,4,3]],[[2]]],[1,0,[10,[],[7,1],3],4]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[8,0]],[0],[[7,[2,3],[0,1,4,1],10],9]]"
            .chars()
            .peekable();
        let mut p2 = "[[[[0,3,0],10,4,[0,10],[7]],[0,4,[8,10,7,10,10],[],3],[[6,2,5],[],7],6,1],[[],6,6,[[3,5,3,8,0]]],[[0],4,[],[]],[6,8,8,[0,[4,9,2,7,4],[2,1,3,0,5]]]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[[7,5,6,2,8],4,[2],7,[2]],10,4,0,[]],[[[3,7,6,10,4],0,[10,9,7,1]],[3,[],[4],0],[3,[2,3],[1,6,10,2],2]],[],[],[6,[6,8,[7,5,1,6],[1,5,2],[9,1,9,2]],[[10,1,6],[6,0,4,4],9,0,[2,1,10,0,9]]]]".chars().peekable();
        let mut p2 = "[[3,[2,[1,5],[3]],[],[[2,6,10],5,[8,5,7,4],[8,9,6,6],1]]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[0,[[4],1,4],[[9,0,7,1],[4],[0,2,2,3,2],2,4]],[9,10]]"
            .chars()
            .peekable();
        let mut p2 = "[[0,[2,8,[2,9,3],3,[]],[10],4],[[6,[6,7,7],9,5],[[9]],[2,[0,8,1],[3,6,1]],[[2,6,0],8,[6,2],[0,10,5,8,1]],9]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[9,3,6,[[1],[10,1]],6],[]]".chars().peekable();
        let mut p2 = "[[0,3,[4,[],[10,0,1,6],10,[6,3,2,8]],[[3,0],[2,8,5,10]]],[[],7,3],[3],[10,[3]],[4,5,8,[[],[4,3,8],4,5,7],10]]".chars().peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 =
            "[[5],[9,[3,[1,9,8,4,8],0,7,[4,2,2]]],[],[[[2],3],5,6,[[5],1,2,[9],[8,6]],1],[6,8]]"
                .chars()
                .peekable();
        let mut p2 = "[[],[],[0,[[4,8,9,3],[2,1,7]],8],[[],10,4,[[]]]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[8,8,6,[]]],[[[5,3,8],5],[0]]]".chars().peekable();
        let mut p2 =
            "[[[1,[10,9,7]],9],[],[[10,6]],[[6],[6,[]],[[],1,8,3,[10]],7],[4,10,[[7],[]]]]"
                .chars()
                .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[],[[],2,4,[8,7,[8,2,7,1,9],9,[1,1,10]],8],[[],1]]"
            .chars()
            .peekable();
        let mut p2 = "[[[[3],[5,10],[1,10,0,3,7],[8,9,1,2,5]],5,[0,[4,0,5,5,10]],[2,[1],6],[]],[0,3],[],[[[9,7,2,10],[8],0,[10,9,3],2],6,[8,0,[6,10,2,4,4]]]]".chars().peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[[4,[6]],1,9,[]],[4,2,[[6,1,9,9],[6,3,1,7,8]]]]"
            .chars()
            .peekable();
        let mut p2 =
            "[[[6,[4,0,1,2,4],2,[7,4]],[[5,4],0]],[7],[[9,8,[10],[5,9,3,5,0]],[[10],[7,2],[3]],8]]"
                .chars()
                .peekable();
        assert!(is_packets_correct_order(&mut p1, &mut p2));

        let mut p1 = "[[1,7,[[8,7,5],1,2]],[0,10],[[[3,10]],[],[6,[7,9,7,3],6,[4,6],10]],[]]"
            .chars()
            .peekable();
        let mut p2 = "[[[1],2],[],[],[0,[3,[5,4,1,9,2],[8,10,4,5]],8]]"
            .chars()
            .peekable();
        assert!(!is_packets_correct_order(&mut p1, &mut p2));
    }
}
