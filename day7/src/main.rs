use std::{
    collections::{HashMap, HashSet},
    fs,
};

const FILE_NAME: &str = "data1.txt";
const ROOT_NODE_NAME: &str = "root";
const ROOT_NODE_INDEX: usize = 0;

// criterias
const PART_ONE_MAX_DIRECTORY_SIZE: usize = 100000;
const PART_TWO_DISK_SPACE: usize = 70000000;
const PART_TWO_SPACE_NEEDED: usize = 30000000;

// terminal command constants
const COMMAND_TOKEN: &str = "$";
const ROOT_TOKEN: &str = "/";
const PARENT_TOKEN: &str = "..";
const COMMAND_TYPE_INDEX: usize = 1;
const CHANGE_DIRECTORY_NAME_INDEX: usize = 2;

// directory constants
const DIRECTORY_TOKEN: &str = "dir";
const DIRECTORY_NAME_INDEX: usize = 1;

// file constants
const FILE_SIZE_INDEX: usize = 0;
const FILE_NAME_INDEX: usize = 1;

fn main() {
    let mut data = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    data.pop();

    answer(&data);
}

/// This large function should be split but it is difficult due to lifetimes. It currently completes three different purposes.
/// 1. Parse input to build a file system of directories aka nodes. The nodes are stored in an array at
///    specfic indexes. The indexes are used to relate parent nodes to children nodes.
/// 2. Find the total size of each directory by using depth first search to traverse the entire file system.
///    Total size is the size of all files stored directly in a directory and all of its children directories.
/// 3. Answer part one and part two using the created file system with defined total sizes.
fn answer(data: &str) {
    let mut nodes: Vec<Option<Node>> = vec![];
    let mut node_indexes: HashMap<NodeID, usize> = HashMap::new();

    // setup and start at root node to build remainder of file system
    node_indexes.insert(NodeID::new(ROOT_NODE_NAME, 0), 0);
    let root_node = Node::new(ROOT_NODE_NAME, 0);
    nodes.push(Some(root_node));
    let mut current_node = nodes[ROOT_NODE_INDEX].take().unwrap();

    for line in data.lines() {
        let input = parse_line(line);

        match input {
            Input::Command(command) => {
                if command == Command::List {
                    continue;
                } else if Command::get_directory(command) == ROOT_TOKEN {
                    // change current node to root node
                    let current_node_id = NodeID::new(current_node.name, current_node.parent_index);
                    let current_node_index = *node_indexes.get(&current_node_id).unwrap();
                    nodes[current_node_index] = Some(current_node);
                    current_node = nodes[ROOT_NODE_INDEX].take().unwrap();
                } else if Command::get_directory(command) == PARENT_TOKEN {
                    // change current node to parent of current node
                    let parent_node_index = current_node.parent_index;
                    let current_node_id = NodeID::new(current_node.name, current_node.parent_index);
                    let current_node_index = *node_indexes.get(&current_node_id).unwrap();
                    nodes[current_node_index] = Some(current_node);
                    current_node = nodes[parent_node_index].take().unwrap();
                } else {
                    // change current node to node specified
                    let new_node_name = Command::get_directory(command);

                    let current_node_id = NodeID::new(current_node.name, current_node.parent_index);
                    let current_node_index = *node_indexes.get(&current_node_id).unwrap();
                    let new_node_id = NodeID::new(new_node_name, current_node_index);

                    // create specified node if it does not already exist
                    match node_indexes.get(&new_node_id) {
                        Some(_) => (),
                        None => {
                            node_indexes.insert(new_node_id, nodes.len());
                            let new_node = Node::new(new_node_name, current_node_index);
                            nodes.push(Some(new_node));
                        }
                    }

                    let node_id = NodeID::new(new_node_name, current_node_index);
                    let node_index = *node_indexes.get(&node_id).unwrap();
                    nodes[current_node_index] = Some(current_node);
                    current_node = nodes[node_index].take().unwrap();
                }
            }
            Input::File(file) => {
                current_node.files.insert(file);
            }
            Input::Directory(name) => {
                let current_node_id = NodeID::new(current_node.name, current_node.parent_index);
                let current_node_index = *node_indexes.get(&current_node_id).unwrap();
                let new_node_id = NodeID::new(name, current_node_index);

                // create node if it does not already exist
                match node_indexes.get(&new_node_id) {
                    Some(_) => continue,
                    None => {
                        node_indexes.insert(new_node_id, nodes.len());
                        let new_node = Node::new(name, current_node_index);
                        current_node.children_indexes.insert(nodes.len());
                        nodes.push(Some(new_node));
                    }
                }
            }
        }
    }

    // return ownership of current_node back to node tracker
    let current_node_id = NodeID::new(current_node.name, current_node.parent_index);
    let current_node_index = *node_indexes.get(&current_node_id).unwrap();
    nodes[current_node_index] = Some(current_node);

    // Use DFS to traverse file system and find the total size for each directory
    let mut visited: HashSet<NodeID> = HashSet::new();
    let mut current_node = nodes[ROOT_NODE_INDEX].take().unwrap();
    let mut current_node_index = ROOT_NODE_INDEX;
    let mut stack = vec![current_node_index];
    while !stack.is_empty() {
        let current_node_id = NodeID::new(current_node.name, current_node.parent_index);

        if !visited.contains(&current_node_id) {
            stack.push(current_node_index);
            for child_index in &current_node.children_indexes {
                stack.push(*child_index);
            }
            visited.insert(current_node_id);
        } else {
            let mut total_size = 0;
            for file in &current_node.files {
                total_size += file.size;
            }
            for child_index in &current_node.children_indexes {
                let child_total_size = nodes[*child_index].as_ref().unwrap().total_size.unwrap();
                total_size += child_total_size;
            }
            current_node.total_size = Some(total_size);
        }

        nodes[current_node_index] = Some(current_node);
        current_node_index = stack.pop().unwrap();
        current_node = nodes[current_node_index].take().unwrap();
    }

    // return ownership of current_node back to node tracker
    let current_node_id = NodeID::new(current_node.name, current_node.parent_index);
    let current_node_index = *node_indexes.get(&current_node_id).unwrap();
    nodes[current_node_index] = Some(current_node);

    // part one
    let mut part_one_answer = 0;
    for node in &nodes {
        let node_size = node.as_ref().unwrap().total_size.unwrap();
        if node_size < PART_ONE_MAX_DIRECTORY_SIZE {
            part_one_answer += node_size;
        }
    }
    println!("Part one: {}", part_one_answer);

    // part two
    let mut part_two_answer = PART_TWO_DISK_SPACE + 1;
    let root_size = nodes[ROOT_NODE_INDEX].as_ref().unwrap().total_size.unwrap();
    let space_needed = PART_TWO_SPACE_NEEDED - (PART_TWO_DISK_SPACE - root_size);
    for node in &nodes {
        let node_size = node.as_ref().unwrap().total_size.unwrap();
        if node_size > space_needed && node_size < part_two_answer {
            part_two_answer = node_size;
        }
    }
    println!("Part two: {}", part_two_answer);
}

fn parse_line(line: &str) -> Input {
    let tokens: Vec<&str> = line.split(' ').collect();

    if tokens[0] == COMMAND_TOKEN {
        Input::Command(parse_command(tokens))
    } else if tokens[0] == DIRECTORY_TOKEN {
        Input::Directory(tokens[DIRECTORY_NAME_INDEX])
    } else {
        let file = File::new(
            tokens[FILE_SIZE_INDEX].parse().unwrap(),
            tokens[FILE_NAME_INDEX],
        );
        Input::File(file)
    }
}

fn parse_command(tokens: Vec<&str>) -> Command {
    let command_type = tokens[COMMAND_TYPE_INDEX];

    match command_type {
        "ls" => Command::List,
        "cd" => Command::ChangeDirectory(tokens[CHANGE_DIRECTORY_NAME_INDEX]),
        _ => panic!("Only ls and cd commands are supported"),
    }
}

enum Input<'a> {
    Command(Command<'a>),
    File(File<'a>),
    Directory(&'a str),
}

// TODO remove Clone and Copy
#[derive(Clone, Copy, PartialEq)]
enum Command<'a> {
    ChangeDirectory(&'a str),
    List,
}

impl Command<'_> {
    fn get_directory(command: Command) -> &str {
        match command {
            Command::ChangeDirectory(name) => name,
            Command::List => panic!("Cannot get directory name for Command::List"),
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
struct File<'a> {
    size: usize,
    name: &'a str,
}

impl File<'_> {
    fn new(size: usize, name: &str) -> File {
        File { name, size }
    }
}

/// Uniquely identifies a directory. May be used to quicly find a directory in the file system.
#[derive(Hash, Eq, PartialEq)]
struct NodeID<'a> {
    name: &'a str,
    parent_index: usize,
}

impl NodeID<'_> {
    fn new(name: &str, parent_index: usize) -> NodeID {
        NodeID { name, parent_index }
    }
}

/// Represents a directory. The parent and children nodes are stored in an external array.
/// Total size is the size of all files stored directly in this directory and all children directories.
struct Node<'a> {
    name: &'a str,
    parent_index: usize,
    children_indexes: HashSet<usize>,
    files: HashSet<File<'a>>,
    total_size: Option<usize>,
}

impl Node<'_> {
    fn new(name: &str, parent_index: usize) -> Node {
        Node {
            name,
            parent_index,
            children_indexes: HashSet::new(),
            files: HashSet::new(),
            total_size: None,
        }
    }
}
