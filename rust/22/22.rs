use std::collections::HashMap;

use regex::Regex;
// Not a generalized solution sadly...
fn main() {
    part1();
    part2();
}

#[derive(Debug)]
struct Grid {
    coords: HashMap<(usize, usize), char>,
    row_mins: HashMap<usize, usize>,
    row_maxes: HashMap<usize, usize>,
    col_mins: HashMap<usize, usize>,
    col_maxes: HashMap<usize, usize>,
}
fn read_input() -> (Grid, Vec<char>, Vec<u32>) {
    let mut split = include_str!("input.txt").split("\n\n");
    let grid = split.next().unwrap().lines();
    let instructions = split.next().unwrap();
    let mut coords: HashMap<(usize, usize), char> = HashMap::new();
    let mut row_mins: HashMap<usize, usize> = HashMap::new();
    let mut row_maxes: HashMap<usize, usize> = HashMap::new();
    let mut col_mins: HashMap<usize, usize> = HashMap::new();
    let mut col_maxes: HashMap<usize, usize> = HashMap::new();

    for (row, line) in grid.enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char != ' ' {
                if !row_mins.contains_key(&row) || row_mins.get(&row).unwrap() > &col {
                    row_mins.insert(row, col);
                }
                if !row_maxes.contains_key(&row) || row_maxes.get(&row).unwrap() < &col {
                    row_maxes.insert(row, col);
                }

                if !col_mins.contains_key(&col) || col_mins.get(&col).unwrap() > &row {
                    col_mins.insert(col, row);
                }
                if !col_maxes.contains_key(&col) || col_maxes.get(&col).unwrap() < &row {
                    col_maxes.insert(col, row);
                }
                coords.insert((row, col), char);
            }
        }
    }

    let grid = Grid {
        coords,
        row_mins,
        row_maxes,
        col_mins,
        col_maxes,
    };
    let turn_splitter = Regex::new(r"\d*").unwrap();
    let step_splitter = Regex::new(r"[LR]").unwrap();
    let turns: Vec<char> = turn_splitter
        .replace_all(instructions, " ")
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();
    let steps: Vec<u32> = step_splitter
        .replace_all(instructions, " ")
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    return (grid, turns, steps);
}

fn get_turn_mapper() -> HashMap<(char, char), char> {
    return vec![
        (('L', '>'), '^'),
        (('R', '>'), 'v'),
        (('L', '<'), 'v'),
        (('R', '<'), '^'),
        (('L', '^'), '<'),
        (('R', '^'), '>'),
        (('L', 'v'), '>'),
        (('R', 'v'), '<'),
    ]
    .into_iter()
    .collect();
}

fn next_position_flat(
    (row, col, facing): (usize, usize, char),
    grid: &Grid,
    flat: bool,
) -> (usize, usize, char) {
    match facing {
        '<' => {
            if col != 0 && grid.coords.contains_key(&(row, col - 1)) {
                return (row, col - 1, facing);
            } else if flat {
                return (row, *grid.row_maxes.get(&row).unwrap(), facing);
            } else {
                return cube_turner(row, col, facing);
            }
        }
        '>' => {
            if grid.coords.contains_key(&(row, col + 1)) {
                return (row, col + 1, facing);
            } else if flat {
                return (row, *grid.row_mins.get(&row).unwrap(), facing);
            } else {
                return cube_turner(row, col, facing);
            }
        }
        '^' => {
            if row != 0 && grid.coords.contains_key(&(row - 1, col)) {
                return (row - 1, col, facing);
            } else if flat {
                return (*grid.col_maxes.get(&col).unwrap(), col, facing);
            } else {
                return cube_turner(row, col, facing);
            }
        }
        'v' => {
            if grid.coords.contains_key(&(row + 1, col)) {
                return (row + 1, col, facing);
            } else if flat {
                return (*grid.col_mins.get(&col).unwrap(), col, facing);
            } else {
                return cube_turner(row, col, facing);
            }
        }
        _ => {
            panic!("This shouldn't happen");
        }
    }
}

fn cube_turner(row: usize, col: usize, facing: char) -> (usize, usize, char) {
    let row_sector = row / 50;
    let col_sector = col / 50;
    let row_offset = row % 50;
    let col_offset = col % 50;

    match (row_sector, col_sector, facing) {
        (0, 1, '^') => return (150 + col_offset, 0, '>'),
        (0, 1, '<') => return (149 - row_offset, 0, '>'),
        (0, 2, '^') => return (199, col_offset, '^'),
        (0, 2, '>') => return (149 - row_offset, 99, '<'),
        (0, 2, 'v') => return (50 + col_offset, 99, '<'),
        (1, 1, '<') => return (100, row_offset, 'v'),
        (1, 1, '>') => return (49, 100 + row_offset, '^'),
        (2, 0, '^') => return (50 + col_offset, 50, '>'),
        (2, 0, '<') => return (49 - row_offset, 50, '>'),
        (2, 1, '>') => return (49 - row_offset, 149, '<'),
        (2, 1, 'v') => return (150 + col_offset, 49, '<'),
        (3, 0, '<') => return (0, 50 + row_offset, 'v'),
        (3, 0, 'v') => return (0, 100 + col_offset, 'v'),
        (3, 0, '>') => return (149, 50 + row_offset, '^'),
        _ => {
            panic!("Can't traverse the cube??")
        }
    }
}

fn traverse_flatland(grid: Grid, steps: Vec<u32>, turns: Vec<char>, flat: bool) -> usize {
    let turn_mapper = get_turn_mapper();

    let mut position = (0, *grid.row_mins.get(&0).unwrap(), '>');
    let mut steps_iter = steps.iter();
    let mut turns_iter = turns.iter();

    while let Some(current_steps) = steps_iter.next() {
        let mut steps = *current_steps;
        while steps > 0 {
            let next = next_position_flat(position, &grid, flat);
            if *grid.coords.get(&(next.0, next.1)).unwrap() == '#' {
                break;
            }

            position = next;

            steps -= 1;
        }

        if let Some(turn) = turns_iter.next() {
            position.2 = *turn_mapper.get(&(*turn, position.2)).unwrap();
        }
    }

    let facing_value: usize = if position.2 == 'v' {
        1
    } else if position.2 == '<' {
        2
    } else if position.2 == '^' {
        3
    } else {
        0
    };

    let sum = (position.0 + 1) * 1000 + (position.1 + 1) * 4 + facing_value;
    return sum;
}

fn part1() {
    let (grid, turns, steps) = read_input();
    let result = traverse_flatland(grid, steps, turns, true);
    println!("Part1: {result}");
}
fn part2() {
    let (grid, turns, steps) = read_input();
    let result = traverse_flatland(grid, steps, turns, false);
    println!("Part1: {result}");
}
