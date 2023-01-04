use std::{collections::HashMap, time::Instant};

struct Node {
    pub value: u64,
    pub children: Vec<usize>,
    pub parent: usize,
}
fn read_input() -> Vec<String> {
    let file_contents = include_str!("input.txt");
    return file_contents.lines().map(str::to_string).collect();
}

fn create_tree() -> Vec<Node> {
    let lines = read_input();
    let mut last_node: usize = usize::MAX;
    let mut nodes: Vec<Node> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.contains("$ ls") {
            let mut value: u64 = 0;
            let mut pointer = i + 1;
            while pointer < lines.len() {
                let pointer_line = lines.get(pointer).unwrap();
                if pointer_line.starts_with("$") {
                    break;
                } else if !pointer_line.starts_with("dir") {
                    let line_value = pointer_line
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                    value += line_value;
                }
                pointer += 1;
            }
            let current_node: Node = Node {
                value: value,
                children: Vec::new(),
                parent: last_node,
            };
            let new_index = nodes.len();
            nodes.push(current_node);
            if last_node != usize::MAX {
                nodes.get_mut(last_node).unwrap().children.push(new_index);
            }
            last_node = new_index;
        } else if line.starts_with("$ cd ..") {
            last_node = nodes.get(last_node).unwrap().parent;
        }
    }
    return nodes;
}

fn set_total_values(
    current: usize,
    nodes: &Vec<Node>,
    results_map: &mut HashMap<usize, u64>,
) -> u64 {
    let node = nodes.get(current).unwrap();
    let mut total = node.value.clone();
    let children = node.children.clone();
    for child_index in children {
        total += set_total_values(child_index, nodes, results_map);
    }
    results_map.insert(current, total);
    return total;
}

fn part1() {
    let nodes = create_tree();
    let mut results_map: HashMap<usize, u64> = HashMap::new();
    let _total: u64 = set_total_values(0, &nodes, &mut results_map);
    let mut part1_answer = 0;
    for (_i, size) in results_map {
        if size <= 100000 {
            part1_answer += size;
        }
    }

    println!("Part1: {part1_answer}");
}

fn part2() {
    let nodes = create_tree();
    let mut results_map: HashMap<usize, u64> = HashMap::new();
    let total: u64 = set_total_values(0, &nodes, &mut results_map);
    let disk_capacity: u64 = 70000000;
    let desired_free_space: u64 = 30000000;
    let need_to_free = desired_free_space - (disk_capacity - total);
    let mut final_answer: u64 = total;
    for (_i, dir_size) in results_map {
        if dir_size >= need_to_free && final_answer > dir_size {
            final_answer = dir_size;
        }
    }
    println!("Part2: {final_answer}");
}

fn main() {
    let start = Instant::now();

    part1();
    let p1 = start.elapsed();
    let start2 = Instant::now();
    part2();
    let p2 = start2.elapsed();

    println!("Part1:{:?}, Part1:{:?}", p1, p2);
}
