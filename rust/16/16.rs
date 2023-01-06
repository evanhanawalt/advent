use std::{cmp::Ordering, collections::HashMap};

use regex::Regex;

#[derive(Debug, Clone)]
struct Valve {
    flow: i32,
    neighbors: Vec<usize>,
}

#[derive(Debug, Clone)]
struct State {
    index: usize,
    flow: i32,
    time: i32,
    visited: Vec<usize>,
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
        let flow = flow_str.parse::<i32>().unwrap();
        valves.insert(index, Valve { flow, neighbors });
    }
    return valves;
}

fn compute_shortest(valves: &HashMap<usize, Valve>, index: usize) -> HashMap<usize, i32> {
    let mut shortest_distances: HashMap<usize, i32> = HashMap::new();
    let mut visited: Vec<usize> = Vec::new();
    for (k, _v) in valves {
        shortest_distances.insert(*k, i32::MAX);
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

fn get_distances(valves: &HashMap<usize, Valve>) -> HashMap<usize, HashMap<usize, i32>> {
    let mut distances: HashMap<usize, HashMap<usize, i32>> = HashMap::new();
    for (index, _v) in valves {
        distances.insert(*index, compute_shortest(valves, *index));
    }
    return distances;
}

fn get_all_paths(
    valves: &HashMap<usize, Valve>,
    distances: &HashMap<usize, HashMap<usize, i32>>,
    minutes: i32,
) -> Vec<State> {
    let mut paths: Vec<State> = Vec::new();
    let mut unfinished_paths: Vec<State> = Vec::new();
    unfinished_paths.push(State {
        index: index_mapper(&String::from("AA")),
        flow: 0,
        time: minutes,
        visited: Vec::new(),
    });

    let things_to_visit: Vec<usize> = valves
        .into_iter()
        .filter(|(_k, v)| v.flow > 0)
        .map(|(k, _v)| *k)
        .collect();

    while !unfinished_paths.is_empty() {
        let current = unfinished_paths.remove(0);
        // println!("{:?}", current);
        let options: Vec<usize> = things_to_visit
            .iter()
            .filter(|n| !current.visited.contains(n))
            .cloned()
            .collect();
        if options.is_empty() {
            paths.push(current.clone());
        } else {
            for n in &options {
                let steps = distances.get(&current.index).unwrap().get(n).unwrap() + 1;
                let new_time = current.time - steps;
                if new_time <= 0 {
                    paths.push(current.clone());
                } else {
                    let new_flow = current.flow + (valves.get(n).unwrap().flow * new_time);
                    let mut new_visited = Vec::clone(&current.visited);

                    new_visited.push(*n);

                    // dont bother if there is already a path with these visited with a bigger flow
                    let add_this: bool = !unfinished_paths.iter().any(|p| {
                        p.flow > new_flow && p.visited.iter().all(|node| new_visited.contains(node))
                    });

                    if add_this {
                        let new_state = State {
                            flow: new_flow,
                            time: new_time,
                            visited: new_visited,
                            index: *n,
                        };
                        unfinished_paths.push(new_state);
                    }
                }
            }
        }
    }
    return paths;
}

fn part1() {
    let valves = get_valves();
    let distances = get_distances(&valves);
    let mut paths = get_all_paths(&valves, &distances, 30);
    paths.sort_by(|a, b| b.flow.cmp(&a.flow));
    println!("Part1:{}", paths.get(0).unwrap().flow);
}
fn part2() {
    let valves = get_valves();
    let distances = get_distances(&valves);
    let mut paths = get_all_paths(&valves, &distances, 26);
    paths.sort_by(|a, b| b.flow.cmp(&a.flow));

    'main: for i in 0..paths.len() {
        for e in (i + 1)..paths.len() {
            let valid = paths
                .get(i)
                .unwrap()
                .visited
                .iter()
                .all(|node| !paths.get(e).unwrap().visited.contains(node));
            if valid {
                let result = paths.get(i).unwrap().flow + paths.get(e).unwrap().flow;
                println!("Part2: {result}");
                break 'main;
            }
        }
    }
}
fn main() {
    part1();
    part2();
}
