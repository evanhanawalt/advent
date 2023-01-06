use std::collections::HashMap;

fn main() {
    part1();
    part2()
}
type Rock = (i64, i64);

struct PatternResults {
    height_delta: i64,
    blocked: Vec<Rock>,
    end_index: usize,
}

fn read_input() -> Vec<char> {
    return include_str!("input.txt").chars().collect();
}

fn get_new_rock(index: i64, highest_rock: i64) -> Vec<Rock> {
    let rock_type = index % 5;
    let y_min = highest_rock + 4;
    match rock_type {
        0 => {
            return vec![(2, y_min), (3, y_min), (4, y_min), (5, y_min)];
        }
        1 => {
            return vec![
                (3, y_min + 2),
                (2, y_min + 1),
                (3, y_min + 1),
                (4, y_min + 1),
                (3, y_min),
            ];
        }
        2 => {
            return vec![
                (2, y_min),
                (3, y_min),
                (4, y_min),
                (4, y_min + 1),
                (4, y_min + 2),
            ];
        }
        3 => {
            return vec![(2, y_min), (2, y_min + 1), (2, y_min + 2), (2, y_min + 3)];
        }
        4 => {
            return vec![(2, y_min), (3, y_min), (2, y_min + 1), (3, y_min + 1)];
        }
        _ => panic!("Invalid Rock"),
    }
}

fn find_longest_pattern(strings: &Vec<String>) -> Vec<String> {
    let longest: Vec<String> = Vec::new();
    let mut max_len = strings.len() / 2;
    while max_len > 1 {
        for start in 0..(strings.len() - max_len) {
            let mut pattern: Vec<String> = Vec::new();
            for sub_index in 0..max_len {
                if strings[start + sub_index] == strings[start + sub_index + max_len] {
                    pattern.push(strings[start + sub_index].clone());
                } else {
                    break;
                }
            }
            if pattern.len() == max_len {
                return pattern;
            }
        }
        max_len -= 1;
    }
    return longest;
}

fn make_pattern_key(
    rock_hist: &Vec<Rock>,
    rock_type: i64,
    current_input: usize,
    curr_height: i64,
) -> String {
    let hist_strings: Vec<String> = rock_hist
        .iter()
        .map(|(x, y)| format!("({},{})", *x, curr_height - *y))
        .collect();
    let hist_string = hist_strings.join("|");
    return format!("{hist_string},{rock_type},{current_input}");
}

fn simulate(inputs: &Vec<char>, num_rocks: i64) -> i64 {
    let mut highest_rock = -1;
    let mut rock_index: i64 = -1;
    let mut current_input: usize = 0;
    let mut pattern_history: Vec<String> = Vec::new();
    let mut pattern_map: HashMap<String, PatternResults> = HashMap::new();
    let mut blocked_coordinates: Vec<Rock> = Vec::new();
    while rock_index < num_rocks {
        rock_index += 1;
        let mut current_rock = get_new_rock(rock_index, highest_rock);
        let rock_type = rock_index % 5;
        let pattern_key =
            make_pattern_key(&blocked_coordinates, rock_type, current_input, highest_rock);

        // This has been seen before
        if pattern_map.contains_key(&pattern_key) {
            if rock_index >= 2000 && rock_index < 6000 {
                pattern_history.push(pattern_key.clone());
            } else if rock_index == 6001 {
                let best_pattern = find_longest_pattern(&pattern_history);
                let num_to_simulate = (num_rocks - rock_index) / best_pattern.len() as i64;
                let pattern_h_delta: i64 = best_pattern
                    .iter()
                    .map(|key| pattern_map.get(key).unwrap().height_delta)
                    .sum();

                let total_h_added = num_to_simulate * pattern_h_delta - 1;
                let total_rocks_dropped = num_to_simulate * (best_pattern.len() as i64);
                rock_index += total_rocks_dropped;
                highest_rock += total_h_added;
            }

            let a = pattern_map.get(&pattern_key).unwrap();
            current_input = a.end_index;
            highest_rock += a.height_delta;
            blocked_coordinates = a
                .blocked
                .iter()
                .map(|(x, y)| (*x, *y + highest_rock))
                .collect();
        } else {
            // this needs to be simulated
            loop {
                let direction = inputs.get(current_input).unwrap();
                current_input = current_input + 1;
                if current_input == inputs.len() {
                    current_input = 0;
                }
                let x_delta = if *direction == '<' { -1 } else { 1 };

                let can_shift = !current_rock.iter().any(|(x, y)| {
                    let next = (*x + x_delta, *y);
                    if next.0 < 0 || next.0 > 6 || next.1 < 0 {
                        return true;
                    }
                    blocked_coordinates.contains(&next)
                });

                if can_shift {
                    current_rock = current_rock
                        .iter()
                        .map(|(x, y)| (*x + x_delta, *y))
                        .collect();
                }

                let can_drop = !current_rock.iter().any(|(x, y)| {
                    let next = (*x, *y - 1);
                    if next.0 < 0 || next.0 > 6 || next.1 < 0 {
                        return true;
                    }
                    blocked_coordinates.contains(&next)
                });

                if can_drop {
                    current_rock = current_rock.iter().map(|(x, y)| (*x, *y - 1)).collect();
                } else {
                    let last_top = highest_rock;
                    for (x, y) in current_rock {
                        if y > highest_rock {
                            highest_rock = y;
                        }
                        blocked_coordinates.push((x, y));
                    }
                    let height_delta = highest_rock - last_top;
                    blocked_coordinates = blocked_coordinates
                        .iter()
                        .filter(|(_x, y)| {
                            let h = highest_rock - *y;
                            return h < 101;
                        })
                        .cloned()
                        .collect();

                    let p_blocked: Vec<Rock> = blocked_coordinates
                        .iter()
                        .cloned()
                        .map(|(x, y)| (x, y - highest_rock))
                        .collect();
                    if rock_index > 200 {
                        pattern_map.insert(
                            pattern_key,
                            PatternResults {
                                height_delta,
                                blocked: p_blocked,
                                end_index: current_input,
                            },
                        );
                    }
                    break;
                }
            }
        }
    }
    return highest_rock + 1;
}

fn part1() {
    let input = read_input();
    let result = simulate(&input, 2022);
    println!("Part1: {result}");
}

fn part2() {
    let input = read_input();

    let result = simulate(&input, 1000000000000);
    println!("Part2: {result}");
}
