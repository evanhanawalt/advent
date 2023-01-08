use std::{cmp::Ordering, collections::HashSet};

fn main() {
    part1();
    part2();
}

fn get_least_common_multiple(a: i32, b: i32) -> i32 {
    let mut min = if a > b { a } else { b };
    loop {
        if min % a == 0 && min % b == 0 {
            return min;
        }
        min += 1;
    }
}
type Blizzard = (char, i32, i32);
type State = (i32, i32, i32);
type Coord = (i32, i32);
type VisitState = (i32, i32, i32);
fn read_input() -> (Vec<Blizzard>, i32, i32) {
    let mut blizzards: Vec<Blizzard> = Vec::new();
    let directions: [char; 4] = ['>', '<', '^', 'v'];
    let lines = include_str!("input.txt").lines();
    let num_rows = lines.clone().count() as i32;
    let mut num_cols = 0 as i32;
    for (row, line) in lines.enumerate() {
        num_cols = line.len() as i32;
        for (col, char) in line.chars().enumerate() {
            if directions.contains(&char) {
                blizzards.push((char, row as i32, col as i32));
            }
        }
    }

    return (blizzards, num_rows, num_cols);
}

fn simulate_all_blizzards(
    blizzards: &Vec<Blizzard>,
    loop_length: i32,
    last_row_wall: i32,
    last_col_wall: i32,
) -> Vec<Vec<Coord>> {
    let mut all_blizzards: Vec<Vec<Blizzard>> = vec![blizzards.clone()];
    let mut all_coords: Vec<Vec<Coord>> =
        vec![blizzards.iter().map(|(_, row, col)| (*row, *col)).collect()];
    while all_blizzards.len() < (loop_length as usize) {
        let last = &all_blizzards[all_blizzards.len() - 1];
        let new: Vec<Blizzard> = last
            .iter()
            .map(|(dir, row, col)| {
                let (_next_dir, mut next_row, mut next_col) = if *dir == '^' {
                    (*dir, row - 1, *col)
                } else if *dir == '>' {
                    (*dir, *row, col + 1)
                } else if *dir == 'v' {
                    (*dir, row + 1, *col)
                } else if *dir == '<' {
                    (*dir, *row, col - 1)
                } else {
                    panic!("This shouldn't happen")
                };
                if next_row == 0 {
                    next_row = last_row_wall - 1;
                } else if next_row == last_row_wall {
                    next_row = 1;
                } else if next_col == 0 {
                    next_col = last_col_wall - 1;
                } else if next_col == last_col_wall {
                    next_col = 1;
                }
                return (*dir, next_row, next_col);
            })
            .collect();
        all_coords.push(new.iter().map(|(_, row, col)| (*row, *col)).collect());
        all_blizzards.push(new);
    }

    return all_coords;
}

fn sort_states((time_a, _, _): &State, (time_b, _, _): &State) -> Ordering {
    return time_a.cmp(time_b);
}

fn find_time_to_goal(
    start: State,
    (end_row, end_col): Coord,
    all_blizzards: &Vec<Vec<Coord>>,
    loop_length: i32,
    last_row_wall: i32,
    last_col_wall: i32,
) -> State {
    let (_start_time, start_row, start_col) = start;
    let mut unvisited: Vec<State> = vec![start];
    let mut visited: HashSet<VisitState> = HashSet::new();
    while unvisited.len() > 0 {
        unvisited.sort_by(sort_states);
        let (c_time, c_row, c_col) = unvisited.remove(0);
        let next_moves: Vec<State> = vec![
            (c_time + 1, c_row + 1, c_col),
            (c_time + 1, c_row - 1, c_col),
            (c_time + 1, c_row, c_col + 1),
            (c_time + 1, c_row, c_col - 1),
            (c_time + 1, c_row, c_col),
        ];
        for (m_time, m_row, m_col) in next_moves {
            if m_row == end_row && m_col == end_col {
                return (m_time, m_row, m_col);
            }
            let visit_key = (m_time % loop_length, m_row, m_col);
            let out_of_bounds = !(m_row == start_row && m_col == start_col)
                && (m_col <= 0 || m_col >= last_col_wall || m_row <= 0 || m_row >= last_row_wall);
            let dont_visit = out_of_bounds
                || all_blizzards[(m_time % loop_length) as usize].contains(&(m_row, m_col))
                || visited.contains(&visit_key);
            if dont_visit {
                continue;
            }
            visited.insert(visit_key);
            unvisited.push((m_time, m_row, m_col));
        }
    }
    return (0, 0, 0);
}

fn part1() {
    let (blizzards, num_rows, num_cols) = read_input();
    let last_row_wall = num_rows - 1;
    let last_col_wall = num_cols - 1;
    let loop_length = get_least_common_multiple(num_rows - 2, num_cols - 2);
    let all_coords = simulate_all_blizzards(&blizzards, loop_length, last_row_wall, last_col_wall);
    let start = (0, 0, 1);
    let end = (36, 100);
    let (result, _, _) = find_time_to_goal(
        start,
        end,
        &all_coords,
        loop_length,
        last_row_wall,
        last_col_wall,
    );
    println!("Part1: {result}");
}

fn part2() {
    let (blizzards, num_rows, num_cols) = read_input();
    let last_row_wall = num_rows - 1;
    let last_col_wall = num_cols - 1;
    let loop_length = get_least_common_multiple(num_rows - 2, num_cols - 2);
    let all_coords = simulate_all_blizzards(&blizzards, loop_length, last_row_wall, last_col_wall);
    let start = (0, 0, 1);
    let end = (36, 100);
    let trip_2_start = find_time_to_goal(
        start,
        end,
        &all_coords,
        loop_length,
        last_row_wall,
        last_col_wall,
    );
    let trip_2_end = (0, 1);
    let trip_3_start = find_time_to_goal(
        trip_2_start,
        trip_2_end,
        &all_coords,
        loop_length,
        last_row_wall,
        last_col_wall,
    );

    let (result, _, _) = find_time_to_goal(
        trip_3_start,
        end,
        &all_coords,
        loop_length,
        last_row_wall,
        last_col_wall,
    );
    println!("Part2: {result}");
}
