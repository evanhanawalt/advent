use std::collections::HashMap;
struct Instruction {
    quant:u16,
    from:u16,
    to:u16
}
fn read_input() -> (HashMap<u16, Vec<char>>, Vec<Instruction>) {
    let mut stacks: HashMap<u16,Vec<char>> = HashMap::new();
    let contents = std::fs::read_to_string("./input.txt").unwrap();
    let split:Vec<&str> = contents.split("\n\n").collect();
    
    let top = split.get(0).unwrap();
    let mut top_lines: Vec<&str> = top.lines().collect();
    let number_line = top_lines.pop().unwrap();
    // initialize stack
    for number in number_line.trim().split("   ") {
        stacks.insert(number.parse::<u16>().unwrap(), Vec::new());
    }
    // fill stacks with values
    for stack_line in top_lines.iter().rev() {
        for (i, c) in stack_line.chars().enumerate() {
            if c != ' ' && c != '[' && c!=']' {
                let stack_num:u16 = ((u16::try_from(i).unwrap()-1)/4) +1;
                stacks.get_mut(&stack_num).unwrap().push(c);
            }
        }
    }
    let bottom = split.get(1).unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in bottom.lines() {
        let tokens: Vec<&str> = line.split(' ').collect();
        let quant = tokens.get(1).unwrap()
                            .parse::<u16>().unwrap();
        let from =  tokens.get(3).unwrap()
                            .parse::<u16>().unwrap();
        let to =  tokens.get(5).unwrap()
                            .parse::<u16>().unwrap();
        instructions.push(Instruction {quant, from, to});
    }


    return (stacks, instructions);
}

fn part1() {
    let (mut stacks, instructions) = read_input();
    for instruction in instructions {
        for _i in 0..instruction.quant {
            let from_stack = stacks.get_mut(&instruction.from).unwrap();
            let el = from_stack.pop().unwrap();
            let to_stack = stacks.get_mut(&instruction.to).unwrap();
            to_stack.push(el);
        }
    }
    let mut value:String = String::new();
    
    for index in 1..=9 {
        let char = stacks.get(&index).unwrap().last().unwrap();
        value.push(*char);
    }
    println!("Part1: {value}");
    
}

fn part2() {
    let (mut stacks, instructions) = read_input();
    for instruction in instructions {
        let from_stack = stacks.get_mut(&instruction.from).unwrap();
        let mut middleman:Vec<char> = Vec::new();
        for _i in 0..instruction.quant {
            middleman.push(from_stack.pop().unwrap())
        }
        let to_stack = stacks.get_mut(&instruction.to).unwrap();
        for _i in 0..instruction.quant {
            to_stack.push(middleman.pop().unwrap());
        }
    }
    let mut value:String = String::new();
    
    for index in 1..=9 {
        let char = stacks.get(&index).unwrap().last().unwrap();
        value.push(*char);
    }
    println!("Part2: {value}");
}

fn main() {
    part1();
    part2();
}