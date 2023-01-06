fn main() {
    part1();
    part2();
}

type Cube = (i32, i32, i32);

fn read_input() -> Vec<Cube> {
    let mut output: Vec<Cube> = Vec::new();
    for line in include_str!("input.txt").lines() {
        let mut split = line.split(',');
        output.push((
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        ));
    }
    return output;
}

fn count_adjacent_surfaces(cubes: &Vec<Cube>, c: &Cube) -> u32 {
    let mut count = 0;
    let (x, y, z) = c;
    if cubes.contains(&(*x - 1, *y, *z)) {
        count += 1
    };
    if cubes.contains(&(*x + 1, *y, *z)) {
        count += 1
    };
    if cubes.contains(&(*x, *y - 1, *z)) {
        count += 1
    };

    if cubes.contains(&(*x, *y + 1, *z)) {
        count += 1
    };

    if cubes.contains(&(*x, *y, *z - 1)) {
        count += 1
    };
    if cubes.contains(&(*x, *y, *z + 1)) {
        count += 1
    };
    return count;
}

fn get_all_surface_area(cubes: &Vec<Cube>) -> u32 {
    let mut count: u32 = 0;
    for c in cubes {
        let uncovered_sides: u32 = 6 - count_adjacent_surfaces(cubes, c);
        count += uncovered_sides;
    }
    return count;
}

fn part1() {
    let cubes = read_input();
    let result = get_all_surface_area(&cubes);
    println!("Part1: {result}");
}

fn part2() {
    let cubes = read_input();
    let mut min = 100;
    let mut max = 0;
    for (x, y, z) in &cubes {
        min = *[*x, *y, *z, min].iter().min().unwrap();
        max = *[*x, *y, *z, max].iter().max().unwrap();
    }
    min -= 1;
    max += 1;
    let mut search_positions: Vec<Cube> = vec![(min, min, min)];
    let mut visited: Vec<Cube> = Vec::new();
    let mut result: u32 = 0;
    while !search_positions.is_empty() {
        let current = search_positions.remove(0);
        let (x, y, z) = current;
        if visited.contains(&current) {
            continue;
        }
        if cubes.contains(&current) {
            continue;
        }
        if x < min || y < min || z < min || x > max || y > max || z > max {
            continue;
        }

        // processs this as an in-bound water position
        visited.push((x, y, z));
        result += count_adjacent_surfaces(&cubes, &current);
        search_positions.push((x - 1, y, z));
        search_positions.push((x + 1, y, z));
        search_positions.push((x, y - 1, z));
        search_positions.push((x, y + 1, z));
        search_positions.push((x, y, z - 1));
        search_positions.push((x, y, z + 1));
    }

    println!("Part2: {result}");
}
