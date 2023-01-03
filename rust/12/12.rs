use std::collections::HashMap;

struct HeightMap {
    num_rows: usize,
    num_cols: usize,
    start: (usize, usize),
    end: (usize, usize),
    heights: Vec<i8>,
}

fn read_input() -> HeightMap {
    let lines: Vec<&str> = include_str!("input.txt").lines().collect();
    let mut map: HeightMap = HeightMap {
        num_rows: lines.len(),
        num_cols: lines[0].chars().count(),
        start: (0, 0),
        end: (0, 0),
        heights: vec![],
    };

    let mut height_map: HashMap<char, i8> = ('a'..='z')
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a as i8))
        .collect();
    height_map.insert('S', 0);
    height_map.insert('E', 25);

    for (row_i, row) in lines.iter().enumerate() {
        for (col_i, value) in row.chars().enumerate() {
            let height = *height_map.get(&value).unwrap();

            match value {
                'S' => map.start = (row_i, col_i),
                'E' => map.end = (row_i, col_i),
                _ => (),
            }
            map.heights.push(height);
        }
    }
    return map;
}

fn get_node_index((row, col): (usize, usize), num_cols: usize) -> usize {
    return (row * num_cols) + col;
}

fn pop_nearest_node(
    map: &HeightMap,
    unvisited: &mut Vec<(usize, usize)>,
    distances: &Vec<u32>,
) -> (usize, usize) {
    let (index, coords, _distance) = unvisited
        .iter()
        .enumerate()
        .map(|(i, (row, col))| {
            (
                i,
                (*row, *col),
                distances[get_node_index((*row, *col), map.num_cols)],
            )
        })
        .min_by_key(|(_, _, distance)| *distance)
        .unwrap();
    unvisited.remove(index);
    return coords;
}

fn get_height(node: (usize, usize), map: &HeightMap) -> i8 {
    return map.heights[get_node_index(node, map.num_cols)];
}

fn get_neighbors(map: &HeightMap, node: (usize, usize), reverse: bool) -> Vec<(usize, usize)> {
    let current_height = get_height(node, map);
    let (row, col) = node;

    // filter and check for integer underflow, out of bounds, and height too high

    let neighbors_in_grid = [
        (row.checked_add(1), Some(col)),
        (row.checked_sub(1), Some(col)),
        (Some(row), col.checked_add(1)),
        (Some(row), col.checked_sub(1)),
    ]
    .iter()
    .filter(|(row, col)| (row.is_some() && col.is_some()))
    .map(|(row, col)| (row.unwrap(), col.unwrap()))
    .filter(|(row, col)| (*row < map.num_rows && *col < map.num_cols))
    .filter(|neighbor| {
        let n_height = get_height(*neighbor, map);
        if reverse {
            return current_height <= n_height + 1;
        } else {
            return n_height <= current_height + 1;
        }
    })
    .collect();
    return neighbors_in_grid;
}

fn search(map: HeightMap, reverse: bool) -> Option<u32> {
    let num_rows = map.num_rows;
    let num_cols = map.num_cols;
    let mut distances = vec![u32::MAX; num_rows * num_cols];
    let mut unvisited: Vec<(usize, usize)> = Vec::with_capacity(num_rows * num_cols);

    for row in 0..num_rows {
        for col in 0..num_cols {
            unvisited.push((row, col));
        }
    }

    distances[get_node_index(map.start, num_cols)] = 0;

    while !unvisited.is_empty() {
        let current: (usize, usize) = pop_nearest_node(&map, &mut unvisited, &distances);
        let current_distance: u32 = distances[get_node_index(current, num_cols)];
        let new_distance: u32 = current_distance + 1;
        if !reverse && current == map.end {
            return Some(current_distance);
        } else if reverse && get_height(current, &map) == 0 {
            return Some(current_distance);
        }

        for neighbor in get_neighbors(&map, current, reverse) {
            let n_index = get_node_index(neighbor, num_cols);
            let neighbor_distance = distances[n_index];
            if new_distance < neighbor_distance {
                distances[n_index] = new_distance;
            }
        }
    }

    return None;
}

fn part1() {
    let map: HeightMap = read_input();
    if let Some(result) = search(map, false) {
        println!("Part1: {result}");
    } else {
        println!("Part1: Error..");
    }
}

fn part2() {
    let mut map: HeightMap = read_input();
    let temp = map.end;
    map.end = map.start;
    map.start = temp;
    if let Some(result) = search(map, true) {
        println!("Part2: {result}");
    } else {
        println!("Part2: Error..");
    }
}

fn main() {
    part1();
    part2();
}
