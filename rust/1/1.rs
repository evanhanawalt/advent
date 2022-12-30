fn get_inv() -> Vec<Vec<u32>> {
    let contents = include_str!("./input.txt");
    let mut inventories: Vec<Vec<u32>> = Vec::new();
    let mut current = 0;
    for line in contents.trim().lines() {
        if current == inventories.len() {
            inventories.push(Vec::new());
        }
        if line.is_empty() { current+=1 }
        if let Ok(n) = line.parse() {
            inventories[current].push(n);
        }
    }
    
    return inventories;
}
fn part1() {
    // Read input and create the inventories
    let inventories = get_inv();
    let res: u32 = inventories.iter().map(|v| v.iter().sum()).max().unwrap();
    println!("Part1: {res}");
}

fn part2() {
    let inventories = get_inv();
    let mut sums:Vec<u32> = inventories.iter().map(|v| v.iter().sum()).collect();
    sums.sort();
    sums.reverse();
    let res: u32 = sums.iter().take(3).sum();
    println!("Part2: {res}");
}


fn main (){
    part1();
    part2();
}
