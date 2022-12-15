struct Pair {
    pub left_min:u32,
    pub left_max:u32,
    pub right_min:u32,
    pub right_max:u32
}
 trait Overlap {
    fn is_contained(&self) -> bool;
    fn has_overlap(&self) -> bool;
}
impl Overlap for Pair {
    fn is_contained(&self) -> bool {
        if self.left_max >= self.right_max && self.left_min <= self.right_min {
            return true;
        } else if self.right_max >= self.left_max && self.right_min <= self.left_min {
            return true;
        } 
        return false;
    }
    fn has_overlap(&self) -> bool {
        for i in self.left_min..=self.left_max {
            if i >= self.right_min && i <= self.right_max {
                return true;
            }
        }
        return false;
    }
}
fn read_input () -> Vec<Pair> {
    let contents = std::fs::read_to_string("./input.txt").unwrap();
    let mut ret: Vec<Pair> = Vec::new();
    for line in contents.lines() {
        let mut p:Vec<u32> = Vec::new();
        for side in line.split(',') {
            for value in side.split('-') {
                p.push(value.parse::<u32>().unwrap());
            }
        }
        ret.push(Pair { 
            left_min:*p.get(0).unwrap(), 
            left_max:*p.get(1).unwrap(), 
            right_min:*p.get(2).unwrap(), 
            right_max:*p.get(3).unwrap() 
        });
    }
    return ret;
}

fn part1() {
    let pairs = read_input();
    let mut value = 0;
    for p in pairs {
        if p.is_contained() {
            value+=1;
        }
    }
    println!("Part1: {value}");
}

fn part2() {

    let pairs = read_input();
    let mut value = 0;
    for p in pairs {
        if p.has_overlap() {
            value+=1;
        }
    }
    println!("Part2: {value}");
}

fn main() {
    part1();
    part2();
}