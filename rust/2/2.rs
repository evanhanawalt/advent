use std::collections::HashMap;
fn read_input () -> Vec<String> {
    let contents = std::fs::read_to_string("./input.txt").unwrap();
    let ret: Vec<String> = contents.lines().map(str::to_string).collect();
    return ret;
    
}
fn part1() {
    let lines = read_input();
    let mut score_map:HashMap<String, i32> = HashMap::new();
    score_map.insert(String::from("A X"), 4);
    score_map.insert(String::from("A Y"), 8);
    score_map.insert(String::from("A Z"), 3);
    score_map.insert(String::from("B X"), 1);
    score_map.insert(String::from("B Y"), 5);
    score_map.insert(String::from("B Z"), 9);
    score_map.insert(String::from("C X"), 7);
    score_map.insert(String::from("C Y"), 2);
    score_map.insert(String::from("C Z"), 6);
    
    let mut score:i32 = 0;
    for line in lines.iter() {
        score += score_map.get(line).unwrap();
    }
    println!("Part 1: {score}");
}

fn part2() {
    let lines = read_input();
    let mut score_map:HashMap<String, i32> = HashMap::new();
    score_map.insert(String::from("A X"), 3);
    score_map.insert(String::from("A Y"), 4);
    score_map.insert(String::from("A Z"), 8);
    score_map.insert(String::from("B X"), 1);
    score_map.insert(String::from("B Y"), 5);
    score_map.insert(String::from("B Z"), 9);
    score_map.insert(String::from("C X"), 2);
    score_map.insert(String::from("C Y"), 6);
    score_map.insert(String::from("C Z"), 7);
    
    let mut score:i32 = 0;
    for line in lines.iter() {
        score += score_map.get(line).unwrap();
    }
    println!("Part 2: {score}");
}
fn main() {
    part1();
    part2();
}