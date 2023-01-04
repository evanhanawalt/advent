use std::cell::RefCell;

struct Monkey {
    pub items: Vec<u64>,
    pub operation: Vec<String>,
    pub divisor: u64,
    pub if_true: usize,
    pub if_false: usize,
    pub inspection_count: u64,
}

fn read_input() -> Vec<Vec<String>> {
    return include_str!("input.txt")
        .split("\n\n")
        .map(|entry| entry.lines().map(str::to_string).collect())
        .collect();
}

fn parse_monkeys(input: Vec<Vec<String>>) -> Vec<RefCell<Monkey>> {
    let mut monkeys: Vec<RefCell<Monkey>> = Vec::new();
    for monkey in input {
        let items: Vec<u64> = monkey[1]
            .split(":")
            .last()
            .unwrap()
            .split(",")
            .map(|val| val.trim().parse::<u64>().unwrap())
            .collect();
        let operation: Vec<String> = monkey[2]
            .split("new =")
            .last()
            .unwrap()
            .trim()
            .split(" ")
            .map(str::to_string)
            .collect();
        let divisor: u64 = monkey[3]
            .split("divisible by")
            .last()
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap();

        let if_true: usize = monkey[4]
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let if_false: usize = monkey[5]
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        monkeys.push(RefCell::new(Monkey {
            items,
            operation,
            divisor,
            if_false,
            if_true,
            inspection_count: 0,
        }));
    }
    return monkeys;
}

fn get_monkey_business(
    rounds: u64,
    modulo: u64,
    worry_divisor: u64,
    monkeys: Vec<RefCell<Monkey>>,
) -> u64 {
    for _round in 0..rounds {
        for monkey in &monkeys {
            let mut binding = monkey.borrow_mut();
            while binding.items.len() > 0 {
                let item = binding.items.pop().unwrap();
                let first_param = binding.operation[0].parse::<u64>().unwrap_or(item);
                let second_param = binding.operation[2].parse::<u64>().unwrap_or(item);
                let new_value = if binding.operation[1] == "+" {
                    (first_param + second_param) / worry_divisor % modulo
                } else {
                    (first_param * second_param) / worry_divisor % modulo
                };
                let divisor = binding.divisor;
                if new_value % divisor == 0 {
                    (&monkeys)[binding.if_true]
                        .borrow_mut()
                        .items
                        .push(new_value);
                } else {
                    (&monkeys)[binding.if_false]
                        .borrow_mut()
                        .items
                        .push(new_value);
                }

                binding.inspection_count += 1;
            }
        }
    }

    let mut inspection_counts: Vec<u64> = monkeys
        .iter()
        .map(|monkey| monkey.borrow().inspection_count)
        .collect();
    inspection_counts.sort();
    return inspection_counts.pop().unwrap() * inspection_counts.pop().unwrap();
}

fn part1() {
    let input = read_input();
    let monkeys = parse_monkeys(input);
    let modulo: u64 = monkeys
        .iter()
        .fold(1, |accum, val| accum * val.borrow().divisor);
    let monkey_business = get_monkey_business(20, modulo, 3, monkeys);
    println!("Part1: {monkey_business}");
}
fn part2() {
    let input = read_input();
    let monkeys = parse_monkeys(input);
    let modulo: u64 = monkeys
        .iter()
        .fold(1, |accum, val| accum * val.borrow().divisor);
    let monkey_business = get_monkey_business(10000, modulo, 1, monkeys);
    println!("Part2: {monkey_business}");
}
fn main() {
    part1();
    part2();
}
