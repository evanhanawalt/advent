use regex::Regex;

#[derive(Debug, Clone)]
struct BluePrint {
    number: i32,
    ore_cost: i32,
    clay_cost: i32,
    obsidian_cost_ore: i32,
    obsidian_cost_clay: i32,
    geode_cost_ore: i32,
    geode_cost_obsidian: i32,
}

#[derive(Debug, Clone)]
struct State {
    blueprint: BluePrint,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_bots: i32,
    clay_bots: i32,
    obsidian_bots: i32,
    geode_bots: i32,
    minute: i32,
    time_limit: i32,
}

impl State {
    fn new(bp: BluePrint, time_limit: i32) -> State {
        State {
            blueprint: bp,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
            minute: 0,
            time_limit: time_limit,
        }
    }

    fn can_build_ore_bot(&self) -> bool {
        self.ore >= self.blueprint.ore_cost
            && (self.time_limit - self.minute) > self.blueprint.ore_cost
    }

    fn can_build_clay_bot(&self) -> bool {
        self.ore >= self.blueprint.clay_cost
    }

    fn can_build_obsidian_bot(&self) -> bool {
        self.ore >= self.blueprint.obsidian_cost_ore
            && self.clay >= self.blueprint.obsidian_cost_clay
    }

    fn can_build_geode_bot(&self) -> bool {
        self.ore >= self.blueprint.geode_cost_ore
            && self.obsidian >= self.blueprint.geode_cost_obsidian
    }

    fn build_ore_bot(&mut self) -> &mut Self {
        self.ore -= self.blueprint.ore_cost;
        self.ore_bots += 1;
        self.ore -= 1;
        return self;
    }

    fn build_clay_bot(&mut self) -> &mut Self {
        self.ore -= self.blueprint.clay_cost;
        self.clay_bots += 1;
        self.clay -= 1;
        return self;
    }

    fn build_obsidian_bot(&mut self) -> &mut Self {
        self.ore -= self.blueprint.obsidian_cost_ore;
        self.clay -= self.blueprint.obsidian_cost_clay;
        self.obsidian_bots += 1;
        self.obsidian -= 1;
        return self;
    }

    fn build_geode_bot(&mut self) -> &mut Self {
        self.ore -= self.blueprint.geode_cost_ore;
        self.obsidian -= self.blueprint.geode_cost_obsidian;
        self.geode_bots += 1;
        self.geode -= 1;
        return self;
    }

    fn process_generation(&mut self) {
        self.minute += 1;
        self.ore += self.ore_bots;
        self.clay += self.clay_bots;
        self.obsidian += self.obsidian_bots;
        self.geode += self.geode_bots;
    }

    fn get_fitness(&self) -> i32 {
        let remaining = self.time_limit - self.minute;
        let future_geodes = self.geode + (remaining * self.geode_bots);
        return future_geodes * 10000000
            + self.obsidian_bots * 10000
            + self.clay_bots * 100
            + self.ore_bots;
    }
}

fn main() {
    part1();
    part2();
}

fn read_input() -> Vec<BluePrint> {
    let mut blueprints: Vec<BluePrint> = Vec::new();
    let splitter = Regex::new(r"\D+").unwrap();
    for line in include_str!("input.txt").lines() {
        let replaced = splitter.replace_all(line, " ").to_string();
        let mut split = replaced.trim().split_whitespace();
        blueprints.push(BluePrint {
            number: split.next().unwrap().parse::<i32>().unwrap(),
            ore_cost: split.next().unwrap().parse::<i32>().unwrap(),
            clay_cost: split.next().unwrap().parse::<i32>().unwrap(),
            obsidian_cost_ore: split.next().unwrap().parse::<i32>().unwrap(),
            obsidian_cost_clay: split.next().unwrap().parse::<i32>().unwrap(),
            geode_cost_ore: split.next().unwrap().parse::<i32>().unwrap(),
            geode_cost_obsidian: split.next().unwrap().parse::<i32>().unwrap(),
        });
    }
    return blueprints;
}

fn simulate(bp: BluePrint, time_limit: i32) -> State {
    let mut states: Vec<State> = vec![State::new(bp, time_limit)];
    let mut next_gen: Vec<State> = Vec::new();

    for _time in 0..time_limit {
        for state in &states {
            if state.can_build_ore_bot() {
                let mut next = state.clone();
                next.build_ore_bot().process_generation();
                let ignore = next_gen.iter().any(|s| {
                    s.ore == next.ore
                        && s.clay == next.clay
                        && s.obsidian == next.obsidian
                        && s.get_fitness() >= next.get_fitness()
                });

                if !ignore {
                    next_gen.push(next);
                }
            }
            if state.can_build_clay_bot() {
                let mut next = state.clone();
                next.build_clay_bot().process_generation();
                let ignore = next_gen.iter().any(|s| {
                    s.ore == next.ore
                        && s.clay == next.clay
                        && s.obsidian == next.obsidian
                        && s.get_fitness() >= next.get_fitness()
                });

                if !ignore {
                    next_gen.push(next);
                }
            }
            if state.can_build_obsidian_bot() {
                let mut next = state.clone();
                next.build_obsidian_bot().process_generation();
                let ignore = next_gen.iter().any(|s| {
                    s.ore == next.ore
                        && s.clay == next.clay
                        && s.obsidian == next.obsidian
                        && s.get_fitness() >= next.get_fitness()
                });

                if !ignore {
                    next_gen.push(next);
                }
            }
            if state.can_build_geode_bot() {
                let mut next = state.clone();
                next.build_geode_bot().process_generation();
                let ignore = next_gen.iter().any(|s| {
                    s.ore == next.ore
                        && s.clay == next.clay
                        && s.obsidian == next.obsidian
                        && s.get_fitness() >= next.get_fitness()
                });

                if !ignore {
                    next_gen.push(next);
                }
            }
            // account for waiting
            let mut next = state.clone();
            next.process_generation();
            next_gen.push(next);
        }

        // prune
        next_gen.sort_by(|a, b| b.get_fitness().cmp(&a.get_fitness()));
        states = next_gen.iter().take(2000).cloned().collect();
        next_gen = Vec::new();
    }

    states.sort_by(|a, b| b.geode.cmp(&a.geode));

    return states.get(0).unwrap().clone();
}

fn part1() {
    let blueprints = read_input();
    let mut results: Vec<State> = Vec::new();
    for bp in blueprints {
        let result = simulate(bp, 24);
        results.push(result);
    }

    let answer: i32 = results.iter().map(|s| s.blueprint.number * s.geode).sum();
    println!("Part1: {answer}");
}

fn part2() {
    let blueprints = read_input();
    let mut results: Vec<State> = Vec::new();

    for index in 0..3 {
        let bp = blueprints.get(index).unwrap().clone();
        let result = simulate(bp, 32);
        results.push(result);
    }

    let answer: i32 = results.iter().map(|s| s.geode).product();
    println!("Part2: {answer}");
}
