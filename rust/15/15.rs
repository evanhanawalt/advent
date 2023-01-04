#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    manhattan_d: i32,
}
fn get_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    return (x1.abs_diff(x2) + y1.abs_diff(y2)) as i32;
}
fn read_input() -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in include_str!("input.txt").lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.replace(&['x', 'y', '=', ':', ','][..], "").parse::<i32>())
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .collect();
        let distance = get_distance(numbers[0], numbers[1], numbers[2], numbers[3]);
        let s: Sensor = Sensor {
            x: numbers[0],
            y: numbers[1],
            manhattan_d: distance,
        };
        sensors.push(s);
    }
    return sensors;
}

fn part1() {
    let sensors: Vec<Sensor> = read_input();
    let test_y: i32 = 2000000;
    let mut ranges: Vec<(i32, i32)> = vec![];
    for sensor in sensors {
        let distance_to_test = get_distance(sensor.x, sensor.y, sensor.x, test_y);
        if distance_to_test <= sensor.manhattan_d {
            let intersection = sensor.manhattan_d - distance_to_test;
            ranges.push((sensor.x - intersection, sensor.x + intersection));
        }
        ranges.sort_by(|a, b| a.0.cmp(&b.0));
    }

    let mut merged = ranges.remove(0);
    while !ranges.is_empty() {
        let current = ranges.remove(0);
        if merged.1 >= current.0 && merged.1 < current.1 {
            merged.1 = current.1;
        } else if merged.1 < current.0 {
            panic!("There is a gap here..... make this less brittle");
        }
    }
    let total = merged.1 - merged.0;
    println!("Part1: {:?}", total);
}

fn part2() {
    let sensors: Vec<Sensor> = read_input();
    let max_val: i32 = 4000000;
    let mut ranges: Vec<(i32, i32)> = vec![];
    let mut result: u64 = 0;
    for y_index in 0..=max_val {
        for sensor in &sensors {
            let distance_to_test = get_distance(sensor.x, sensor.y, sensor.x, y_index);
            if distance_to_test <= sensor.manhattan_d {
                let intersection = sensor.manhattan_d - distance_to_test;
                ranges.push((sensor.x - intersection, sensor.x + intersection));
            }
            ranges.sort_by(|a, b| a.0.cmp(&b.0));
        }

        let mut merged = ranges.remove(0);
        while !ranges.is_empty() {
            let current = ranges.remove(0);
            if merged.1 >= current.0 && merged.1 < current.1 {
                merged.1 = current.1;
            } else if merged.1 < current.0 {
                result = (4000000 * (merged.1 as u64 + 1)) + y_index as u64;
            }
        }
    }

    println!("Part2: {:?}", result);
}

fn main() {
    part1();
    part2();
}
