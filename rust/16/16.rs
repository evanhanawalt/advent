use std::{cmp::Ordering, collections::HashMap};

use regex::Regex;

#[derive(Debug, Clone)]
struct Valve {
    index: usize,
    name: String,
    flow: u32,
    neighbors: Vec<usize>,
}

#[derive(Debug, Clone)]
struct State {
    index: usize,
    name: String,
    flow: u32,
    neighbors: Vec<usize>,
}

fn index_mapper(s: &String) -> usize {
    let chars: &[u8] = s.as_bytes();
    let a = (b'A'.abs_diff(chars[0]) as usize) * 26;
    let b = b'A'.abs_diff(chars[1]) as usize;
    return a + b;
}

fn get_valves() -> HashMap<usize, Valve> {
    let ignore_these = Regex::new(r"(rate=)|(;)|(,)").unwrap();
    let mut valves: HashMap<usize, Valve> = HashMap::new();
    for line in include_str!("input.txt").lines() {
        let filtered = ignore_these.replace_all(line, "").to_string();
        let mut split = filtered.split_whitespace().map(str::to_string);
        let name = split.nth(1).unwrap();
        let flow_str = split.nth(2).unwrap();
        let neighbors: Vec<usize> = split.skip(4).map(|s| index_mapper(&s)).collect();
        let index = index_mapper(&name);
        let flow = flow_str.parse::<u32>().unwrap();
        valves.insert(
            index,
            Valve {
                index,
                flow,
                name,
                neighbors,
            },
        );
    }
    return valves;
}

fn compute_shortest(valves: &HashMap<usize, Valve>, index: usize) -> HashMap<usize, u32> {
    let mut shortest_distances: HashMap<usize, u32> = HashMap::new();
    let mut visited: Vec<usize> = Vec::new();
    for (k, v) in valves {
        shortest_distances.insert(*k, u32::MAX);
    }
    shortest_distances.insert(index, 0);

    while visited.len() < valves.len() {
        // ignore visited, find lowest
        let current_index = *shortest_distances
            .keys()
            .filter(|i| !visited.contains(i))
            .min_by(|a: &&usize, b: &&usize| -> Ordering {
                shortest_distances.get(a).cmp(&shortest_distances.get(b))
            })
            .unwrap();
        let current_distance = *shortest_distances.get(&current_index).unwrap();
        visited.push(current_index);
        let neighbors = valves
            .get(&current_index)
            .unwrap()
            .neighbors
            .iter()
            .filter(|v| !visited.contains(v));

        for neighbor_i in neighbors {
            let new_dist = current_distance + 1;
            if new_dist < *shortest_distances.get(&neighbor_i).unwrap() {
                shortest_distances.insert(*neighbor_i, new_dist);
            }
        }
    }

    return shortest_distances;
}

fn get_distances(valves: &HashMap<usize, Valve>) -> HashMap<usize, HashMap<usize, u32>> {
    // let mut distances: HashMap<usize,<HashMap<usize,u32>>> = HashMap::new();
    let mut distances: HashMap<usize, HashMap<usize, u32>> = HashMap::new();
    for (index, v) in valves {
        distances.insert(*index, compute_shortest(valves, *index));
    }
    return distances;
}

fn get_all_paths(
    valves: &HashMap<usize, Valve>,
    distances: &HashMap<usize, HashMap<usize, u32>>,
    minutes: u32,
) -> Vec<State> {
    let mut paths: Vec<State> = Vec::new();

    return paths;
}

fn part1() {
    let valves = get_valves();
    let distances = get_distances(&valves);

    println!("Part1: ");
}
fn part2() {
    println!("Part2: ");
}
fn main() {
    part1();
    part2();
}
