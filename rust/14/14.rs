use std::collections::HashSet;

fn parse_input() -> HashSet<(i32, i32)> {
    let mut blocked_tiles: HashSet<(i32, i32)> = HashSet::new();
    for line in include_str!("input.txt").lines() {
        let a: Vec<(i32, i32)> = line
            .split("->")
            .map(|vert| {
                let mut iter = vert
                    .split(",")
                    .map(|num| num.trim().parse::<i32>().unwrap());
                (iter.next().unwrap(), iter.next().unwrap())
            })
            .collect();
        for i in 0..(a.len() - 1) {
            let (start_x, start_y) = a[i];
            let (end_x, end_y) = a[i + 1];

            if start_x == end_x {
                let increment = if start_y < end_y { 1 } else { -1 };
                let x = start_x;
                let mut y = start_y;
                while y != end_y {
                    blocked_tiles.insert((x, y));
                    y += increment;
                }
            } else if start_y == end_y {
                let increment = if start_x < end_x { 1 } else { -1 };
                let y = start_y;
                let mut x = start_x;
                while x != end_x {
                    blocked_tiles.insert((x, y));
                    x += increment;
                }
            }
            blocked_tiles.insert((end_x, end_y));
        }
    }
    return blocked_tiles;
}

fn simulate(
    blocked_tiles: &mut HashSet<(i32, i32)>,
    last_y: i32,
    floor: i32,
    has_floor: bool,
) -> u32 {
    let mut keep_going = true;
    let mut sand_count = 0;

    while keep_going {
        let mut sand = (500, 0);
        while keep_going {
            if !has_floor && (sand.1 == last_y) {
                sand_count += 1;
                keep_going = false;
                break;
            } else if sand.1 == floor - 1 {
                sand_count += 1;
                blocked_tiles.insert(sand);
                break;
            } else if !blocked_tiles.contains(&(sand.0, sand.1 + 1)) {
                sand.1 = sand.1 + 1;
            } else if !blocked_tiles.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.0 = sand.0 - 1;
                sand.1 = sand.1 + 1;
            } else if !blocked_tiles.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.0 = sand.0 + 1;
                sand.1 = sand.1 + 1;
            } else {
                sand_count += 1;
                blocked_tiles.insert(sand);
                if sand.1 == last_y {
                    keep_going = false;
                }
                break;
            }
        }
    }
    return sand_count;
}

fn part1() {
    let mut blocked_tiles = parse_input();

    let mut bottom = 0;
    for (_x, y) in &blocked_tiles {
        if *y > bottom {
            bottom = *y;
        }
    }
    let num_dropped = simulate(&mut blocked_tiles, bottom, i32::MAX, false);
    let result = num_dropped - 1;
    println!("Part1: {result}");
}

fn part2() {
    let mut blocked_tiles = parse_input();
    let last_y = 0;
    let mut bottom = 0;
    for (_x, y) in &blocked_tiles {
        if *y > bottom {
            bottom = *y;
        }
    }
    bottom += 2;
    let num_dropped = simulate(&mut blocked_tiles, last_y, bottom, true);

    println!("Part2: {num_dropped}");
}
fn main() {
    part1();
    part2();
}
