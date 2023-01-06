fn main() {
    part1();
    part2();
}

fn read_input() -> Vec<i64> {
    let mut inputs: Vec<i64> = Vec::new();
    for line in include_str!("input.txt").lines() {
        inputs.push(line.trim().parse::<i64>().unwrap());
    }
    return inputs;
}

fn part1() {
    let nums = read_input();
    let mut list: Vec<(usize, i64)> = nums.iter().cloned().enumerate().collect();
    for index in 0..nums.len() {
        let (id, _) = list
            .iter()
            .enumerate()
            .find(|(_id, (i, _v))| *i == index)
            .unwrap();

        list.remove(id);
        let mut new_index = nums[index] + id as i64;
        // if the new index is negative, do some math the add the list length just enough to make it positive
        if new_index < 0 {
            let make_positive = ((new_index / list.len() as i64).abs() + 1) * list.len() as i64;
            new_index += make_positive;
        }
        new_index = new_index % (list.len() as i64);
        list.insert(new_index as usize, (index, nums[index]));
    }
    let (zero_index, _) = list
        .iter()
        .enumerate()
        .find(|(_index, (_orig_i, value))| *value == 0)
        .unwrap();
    let result: i64 = vec![1000, 2000, 3000]
        .iter()
        .map(|x| list.get((zero_index + *x) % list.len()).unwrap().1)
        .sum();
    println!("Part1: {result}");
}

fn part2() {
    let times = 10;
    let dec_key = 811589153;
    let nums: Vec<i64> = read_input().iter().map(|v| *v * dec_key).collect();
    let mut list: Vec<(usize, i64)> = nums.iter().cloned().enumerate().collect();

    for _t in 0..times {
        for index in 0..nums.len() {
            let (id, _) = list
                .iter()
                .enumerate()
                .find(|(_id, (i, _v))| *i == index)
                .unwrap();

            list.remove(id);
            let mut new_index = nums[index] + id as i64;

            // if the new index is negative, do some math the add the list length just enough to make it positive
            if new_index < 0 {
                let make_positive = ((new_index / list.len() as i64).abs() + 1) * list.len() as i64;
                new_index += make_positive;
            }

            new_index = new_index % (list.len() as i64);
            list.insert(new_index as usize, (index, nums[index]));
        }
    }

    let (zero_index, _) = list
        .iter()
        .enumerate()
        .find(|(_index, (_orig_i, value))| *value == 0)
        .unwrap();
    let result: i64 = vec![1000, 2000, 3000]
        .iter()
        .map(|x| list.get((zero_index + *x) % list.len()).unwrap().1)
        .sum();
    println!("Part2: {result}");
}
