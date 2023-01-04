fn read_input() -> Vec<String> {
    return include_str!("input.txt")
        .lines()
        .map(str::to_string)
        .collect();
}

fn calculate_cycles(lines: Vec<String>) -> Vec<i32> {
    let mut cycles: Vec<i32> = Vec::new();
    let mut x: i32 = 1;
    for line in lines {
        if line.contains("noop") {
            cycles.push(x);
        } else if line.contains("addx") {
            let value = line.split(" ").last().unwrap().parse::<i32>().unwrap();
            cycles.push(x);
            cycles.push(x);
            x += value;
        }
    }
    return cycles;
}

fn get_display(cycles: Vec<i32>) -> String {
    let mut result: Vec<char> = Vec::new();

    for (index, value) in cycles.iter().enumerate() {
        let current_x = (index % 40) as i32;
        let is_visible = (value - current_x).abs() <= 1;
        if is_visible {
            result.push('#');
        } else {
            result.push('.');
        }
        if (index + 1) % 40 == 0 {
            result.push('\n');
        }
    }

    return result.iter().collect();
}

fn part1() {
    let lines = read_input();
    let cycles = calculate_cycles(lines);
    let result: i32 = (cycles[19] * 20)
        + (cycles[59] * 60)
        + (cycles[99] * 100)
        + (cycles[139] * 140)
        + (cycles[179] * 180)
        + (cycles[219] * 220);
    println!("Part1: {result}");
}
fn part2() {
    let lines = read_input();
    let cycles = calculate_cycles(lines);
    let output = get_display(cycles);
    println!("Part2:\n{output}");
}
fn main() {
    part1();
    part2();
}
