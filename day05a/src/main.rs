use std::collections::HashMap;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn contains_block(line: &String) -> bool {
    return line.contains('[');
}

fn contains_move(line: &String) -> bool {
    return line.contains("move");
}

fn create_stacks(lines: &Vec<String>) -> HashMap<isize, Stack<String>> {
    // e.g line = '[Z] [M] [P]'
    // format = 3 chars then break, 3 cars then break

    // So we can create a map -> Vec

    // First we create the list map
    let mut stacks_list_map: HashMap<isize, Vec<String>> = HashMap::new();
    for line in lines {
        // let mut next_val = 1;
        for (i, c) in line.chars().enumerate() {
            // println!("index: {}", i as isize);
            // println!("char: /{}/", c as char);
            // println!("next_val: /{}/", next_val);

            if i % 4 == 1 && (c as char).to_string() != " " {
                let map_index = (i - 1) / 4;
                // We've found a value so we should add it to one of our stack lists
                let mut existing_arr: Vec<String> = Vec::new();
                if stacks_list_map.contains_key(&(map_index as isize)) {
                    existing_arr = stacks_list_map
                        .get(&(map_index as isize))
                        .unwrap()
                        .to_owned();
                    // existing_arr.push((c as char).to_string());
                }
                existing_arr.push((c as char).to_string());
                // println!("index: {}", i as isize);
                // println!("char: /{}/", c as char);
                stacks_list_map.insert((map_index as isize), existing_arr.to_owned());
                // next_val = next_val + 4;
            }
        }
    }
    // println!("{}", stacks_list_map.len());
    //
    // for (key, value) in &stacks_list_map {
    //     println!("{}", key);
    //     for val in value.iter() {
    //         println!("{}", val)
    //     }
    // }

    // then we just reverse the vecs and push onto stacks?
    let mut stacks_map: HashMap<isize, Stack<String>> = HashMap::new();

    for i in 0..stacks_list_map.len() {
        let mut existing_arr = stacks_list_map.get(&(i as isize)).unwrap().to_owned();
        existing_arr.reverse();

        let mut stack: Stack<String> = Stack::new();
        for item in existing_arr {
            stack.push(item)
        }

        stacks_map.insert((i as isize), stack);
    }

    return stacks_map;
}

fn move_stacks(count: isize, start: isize, end: isize, stacks: HashMap<isize, Stack<String>>) {}

fn main() {
    // This gets the initial blocks
    let blocks: Vec<String> = lines_from_file("./example.txt")
        .iter()
        .filter(|line| contains_block(line))
        .map(|a| a.to_owned())
        .collect::<Vec<String>>();

    let moves: Vec<String> = lines_from_file("./example.txt")
        .iter()
        .filter(|line| contains_move(line))
        .map(|a| a.to_owned())
        .collect::<Vec<String>>();

    let stacks = create_stacks(&blocks);

    for (key, value) in stacks {
        println!("{}", key);
        for val in value.stack.iter() {
            println!("{}", val)
        }
    }
    // Create stacks
    // Create a list of stacks, each containing the data

    // Try to use a fold
    // i.e iterate each item onto the stacks

    println!("Blocks: {:?}", blocks);
    println!("Moves: {:?}", moves);
}
