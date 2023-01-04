use std::collections::HashSet;

fn read_input() -> Vec<(String, u32)> {
    let mut steps: Vec<(String, u32)> = Vec::new();
    for line in include_str!("input.txt").lines() {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap().to_string();
        let value = split.next().unwrap();
        steps.push((direction, value.parse::<u32>().unwrap()));
    }
    return steps;
}

fn find_tail_positions(mut knots: Vec<(i32, i32)>, steps: Vec<(String, u32)>) -> usize {
    let mut tail_positions: HashSet<String> = HashSet::new();
    for (direction, distance) in steps {
        let mut count = distance;
        while count > 0 {
            count -= 1;
            if direction == "L" {
                knots[0].0 -= 1;
            } else if direction == "R" {
                knots[0].0 += 1;
            } else if direction == "U" {
                knots[0].1 += 1;
            } else if direction == "D" {
                knots[0].1 -= 1;
            }

            let mut knot_index: usize = 1;
            while knot_index < knots.len() {
                let diff_x: i32 = knots[knot_index - 1].0 - knots[knot_index].0;
                let diff_y: i32 = knots[knot_index - 1].1 - knots[knot_index].1;
                let abs_x = diff_x.abs();
                let abs_y = diff_y.abs();

                if abs_x > 1 || (abs_x == 1 && abs_y > 1) {
                    if diff_x > 0 {
                        knots[knot_index].0 += 1;
                    } else {
                        knots[knot_index].0 -= 1;
                    }
                }

                if abs_y > 1 || (abs_y == 1 && abs_x > 1) {
                    if diff_y > 0 {
                        knots[knot_index].1 += 1;
                    } else {
                        knots[knot_index].1 -= 1;
                    }
                }
                knot_index += 1;
            }
            let last = knots.last().unwrap();
            tail_positions.insert(format!("{},{}", last.0, last.1));
        }
    }
    return tail_positions.len();
}

fn part1() {
    let steps = read_input();
    let knots: Vec<(i32, i32)> = vec![(0, 0), (0, 0)];
    let result = find_tail_positions(knots, steps);
    println!("Part1: {result}");
}

fn part2() {
    let steps = read_input();
    let knots: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let result = find_tail_positions(knots, steps);
    println!("Part2: {result}");
}
fn main() {
    part1();
    part2();
}
