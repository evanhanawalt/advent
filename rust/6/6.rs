use std::collections::HashSet;

fn read_input() -> &'static str {
    let file_contents = include_str!("input.txt");

    return file_contents;
}

fn part1() {
    let input = read_input();
    let mut last_chars: Vec<char> = Vec::new();
    let mut result = 0;
    for (i,char) in input.chars().enumerate() {
        if last_chars.len() < 3 {
            last_chars.push(char);
        } else {
            let set:HashSet<char> = last_chars.iter().cloned().collect();

            if set.contains(&char) || set.len() != 3 {
                last_chars.remove(0);
                last_chars.push(char);
            } else  {
                result = i +1;
                break;
            }
        }
    }
    println!("Part1: {result}");
}

fn part2() {
    let input = read_input();
    let mut last_chars: Vec<char> = Vec::new();
    let mut result = 0;
    for (i,char) in input.chars().enumerate() {
        if last_chars.len() < 13 {
            last_chars.push(char);
        } else {
            let set:HashSet<char> = last_chars.iter().cloned().collect();

            if set.contains(&char) || set.len() != 13 {
                last_chars.remove(0);
                last_chars.push(char);
            } else  {
                result = i +1;
                break;
            }
        }
    }
    println!("Part2: {result}");
}

fn main() {
    part1();
    part2();
}