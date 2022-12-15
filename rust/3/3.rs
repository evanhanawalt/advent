
fn get_char_value(char: char) -> u32 {
    if char.is_ascii_lowercase() {
        return (char as u32) - 96;
    } else {
        return (char as u32) - 38;
    }
}

fn read_input () -> Vec<String> {
    let contents = std::fs::read_to_string("./input.txt").unwrap();
    let ret: Vec<String> = contents.lines().map(str::to_string).collect();
    return ret;
}

fn part1() {
    let contents = read_input();
    let mut result:u32 = 0;
    for line in contents {
        let sack_size = line.len() / 2;
        let left = line.get(..sack_size).unwrap();
        let right = line.get(sack_size..).unwrap();
        'checker: for left_char in left.chars() {
            if right.contains(left_char) {
                result += get_char_value(left_char);
                break 'checker;
            }
        }
    }
    println!("Part1: {}", result);
}

fn part2() {
    let contents = read_input();
    let mut result:u32 = 0;
    let mut current_set: Vec<String> = Vec::new();
    for line in contents {
        if current_set.len() == 2 {
            let one = current_set.remove(0);
            let two = current_set.remove(0);
            'char_checker: for char in line.chars() {
                if one.contains(char) && two.contains(char) {
                    result += get_char_value(char);
                    break 'char_checker;
                }
            }
        } else {
            current_set.push(line);
        }
    }
    println!("Part2: {}", result);
}

fn main() {
    part1();
    part2();
}