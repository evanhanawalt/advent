use std::collections::HashMap;

use regex::Regex;

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

fn traverse_flatland() {}

fn part1() {
    let data = read_input();
    let turn_mapper = get_turn_mapper();
    println!("{:?}", data.0);
}
fn part2() {}
