use std::{cell::RefCell, rc::Rc};

struct Directory {
    pub value: u64,
    pub children: Vec<Rc<RefCell<Option<Directory>>>>,
    pub parent: Rc<RefCell<Option<Directory>>>,
    pub total_value: u64,
}
fn read_input() -> Vec<String> {
    let file_contents = include_str!("input.txt");
    return file_contents.lines().map(str::to_string).collect();
}

fn build_tree() -> Rc<RefCell<Option<Directory>>> {
    let lines = read_input();
    let mut root: Rc<RefCell<Option<Directory>>> = Rc::new(RefCell::new(None));
    let mut last_node: Rc<RefCell<Option<Directory>>> = Rc::new(RefCell::new(None));
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
            let current_directory: RefCell<Option<Directory>> = RefCell::new(Some(Directory {
                value: value,
                children: Vec::new(),
                parent: Rc::clone(&last_node),
                total_value: 0,
            }));
            let new_ref = Rc::new(current_directory);
            if last_node.as_ref().borrow().is_some() {
                let a = last_node.clone();
                a.borrow_mut()
                    .as_mut()
                    .unwrap()
                    .children
                    .push(Rc::clone(&new_ref));
            } else {
                root = Rc::clone(&new_ref);
            }

            last_node = Rc::clone(&new_ref);
        } else if line.starts_with("$ cd ..") {
            if last_node.as_ref().borrow().is_some() {
                let last_ref = last_node.clone();
                last_node = Rc::clone(&last_ref.as_ref().borrow().as_ref().unwrap().parent);
            }
        }
    }
    return root;
}

fn set_totals(node: Rc<RefCell<Option<Directory>>>) -> u64 {
    let binding = node.clone();
    let mut total = 0;
    for child in &binding.borrow_mut().as_mut().unwrap().children {
        let sub = Rc::clone(child);
        total += set_totals(sub);
    }
    total += binding.borrow_mut().as_mut().unwrap().value;
    binding.borrow_mut().as_mut().unwrap().total_value = total;
    return total;
}

fn get_part1_value(node: Rc<RefCell<Option<Directory>>>) -> u64 {
    let binding = node.clone();
    let mut total: u64 = 0;
    for child in &binding.borrow_mut().as_mut().unwrap().children {
        let sub = Rc::clone(child);
        total += get_part1_value(sub);
    }
    let cur_val: u64 = binding.borrow_mut().as_mut().unwrap().total_value;
    if cur_val <= 100000 {
        total += cur_val;
    }
    return total;
}

fn get_part2_value(
    node: Rc<RefCell<Option<Directory>>>,
    need_to_free: u64,
    best_guess: u64,
) -> u64 {
    let binding = node.clone();
    let cur_val: u64 = binding.borrow_mut().as_mut().unwrap().total_value;
    let mut result = best_guess;
    if cur_val >= need_to_free && result > cur_val {
        result = cur_val;
    }
    for child in &binding.borrow_mut().as_mut().unwrap().children {
        let sub = Rc::clone(child);
        let child_best = get_part2_value(sub, need_to_free, result);
        if child_best >= need_to_free && result > child_best {
            result = child_best;
        }
    }

    return result;
}

fn part1() {
    let a = build_tree();
    let _total = set_totals(Rc::clone(&a));
    let result = get_part1_value(Rc::clone(&a));

    println!("Part1: {result}");
}

fn part2() {
    let a = build_tree();
    let total = set_totals(Rc::clone(&a));
    let disk_capacity: u64 = 70000000;
    let desired_free_space: u64 = 30000000;
    let need_to_free = desired_free_space - (disk_capacity - total);
    let result = get_part2_value(Rc::clone(&a), need_to_free, u64::MAX);
    println!("Part2: {result}");
}

fn main() {
    part1();
    part2();
}
