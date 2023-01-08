use std::collections::{HashMap, HashSet};

fn main() {
    part1();
    part2();
}

fn read_input() -> HashSet<(i32, i32)> {
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    for (row, line) in include_str!("input.txt").lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                positions.insert((row as i32, col as i32));
            }
        }
    }
    return positions;
}

fn get_proposed_move(
    (row, col): &(i32, i32),
    first_direction: i32,
    taken_positions: &HashSet<(i32, i32)>,
) -> (i32, i32) {
    let n = taken_positions.contains(&(row - 1, *col));
    let ne = taken_positions.contains(&(row - 1, col + 1));
    let nw = taken_positions.contains(&(row - 1, col - 1));
    let s = taken_positions.contains(&(row + 1, *col));
    let se = taken_positions.contains(&(row + 1, col + 1));
    let sw = taken_positions.contains(&(row + 1, col - 1));
    let e = taken_positions.contains(&(*row, col + 1));
    let w = taken_positions.contains(&(*row, col - 1));
    let has_neighbors = n || ne || nw || s || se || sw || e || w;
    if !has_neighbors {
        return (*row, *col);
    }

    for i in 0..4 {
        let direction = (first_direction + i) % 4;
        match direction {
            0 => {
                if !(n || ne || nw) {
                    return (row - 1, *col);
                }
            }
            1 => {
                if !(s || se || sw) {
                    return (row + 1, *col);
                }
            }
            2 => {
                if !(w || nw || sw) {
                    return (*row, col - 1);
                }
            }
            3 => {
                if !(e || ne || se) {
                    return (*row, col + 1);
                }
            }
            _ => {
                panic!("This shouldn't happen");
            }
        }
    }

    return (*row, *col);
}

fn perform_round(
    elves_position: &HashSet<(i32, i32)>,
    first_direction: i32,
) -> HashSet<(i32, i32)> {
    let mut proposed_moves: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut proposed_destinations: HashSet<(i32, i32)> = HashSet::new();
    let mut doubled: HashSet<(i32, i32)> = HashSet::new();

    for position in elves_position {
        let next = get_proposed_move(position, first_direction, &elves_position);
        proposed_moves.insert(*position, next);
        if proposed_destinations.contains(&next) {
            doubled.insert(next);
        } else {
            proposed_destinations.insert(next);
        }
    }

    let mut results: HashSet<(i32, i32)> = HashSet::new();
    for (key, value) in proposed_moves {
        if doubled.contains(&value) {
            results.insert(key);
        } else {
            results.insert(value);
        }
    }

    return results;
}

fn part1() {
    let mut positions = read_input();
    for i in 0..10 {
        positions = perform_round(&positions, i % 4);
    }

    let mut row_min = 0;
    let mut row_max = 0;
    let mut col_min = 0;
    let mut col_max = 0;
    let num_elves = positions.len() as i32;
    for (row, col) in positions {
        if row > row_max {
            row_max = row;
        }
        if row < row_min {
            row_min = row;
        }
        if col > col_max {
            col_max = col;
        }
        if col < col_min {
            col_min = col;
        }
    }

    println!("{row_min}, {row_max}\n{col_min},{col_max}");
    let result = (1 + col_max - col_min) * (1 + row_max - row_min) - num_elves;
    println!("Part1: {result}");
}

fn part2() {
    let mut positions = read_input();
    let mut i = 0;
    loop {
        let next = perform_round(&positions, i % 4);
        i += 1;
        if next == positions {
            break;
        } else {
            positions = next;
        }
    }
}
