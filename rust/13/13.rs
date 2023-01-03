use std::{cmp::Ordering, vec};

#[derive(Debug, PartialEq)]
enum Node {
    Leaf(i32),
    Inner(Box<Vec<Node>>),
}

fn parse_line(packet: &str) -> Node {
    let mut last_num: Option<i32> = None;
    let mut vec_stack: Vec<Vec<Node>> = vec![vec![]];

    let push_num_on_top = |num: Option<i32>, vecs: &mut Vec<Vec<Node>>| -> Option<i32> {
        let index = vecs.len() - 1;
        if let Some(num) = num {
            vecs[index].push(Node::Leaf(num));
        }
        return None;
    };
    for c in packet.chars() {
        match c {
            '[' => vec_stack.push(vec![]),
            ']' => {
                last_num = push_num_on_top(last_num, &mut vec_stack);
                let v = vec_stack.pop().unwrap();
                let index = vec_stack.len() - 1;
                vec_stack[index].push(Node::Inner(Box::new(v)));
            }
            ' ' => {}
            ',' => last_num = push_num_on_top(last_num, &mut vec_stack),
            d => last_num = Some(last_num.unwrap_or(0) * 10 + (d.to_digit(10).unwrap() as i32)),
        }
    }
    return Node::Inner(Box::new(vec_stack.pop().unwrap()));
}

fn parse_input() -> Vec<Node> {
    let pairs = include_str!("input.txt").split("\n\n");
    let mut list: Vec<Node> = vec![];
    for pair in pairs {
        for packet in pair.lines() {
            let parsed = parse_line(packet);
            list.push(parsed);
        }
    }
    return list;
}

fn compare(first: &Node, second: &Node) -> Ordering {
    match (first, second) {
        (Node::Leaf(left), Node::Leaf(right)) => (*left).cmp(right),
        (Node::Inner(left), Node::Inner(right)) => {
            let mut index = 0;
            while index < left.len() && index < right.len() {
                match compare(&left[index], &right[index]) {
                    Ordering::Equal => {}
                    other => return other,
                };
                index += 1;
            }
            return left.len().cmp(&right.len());
        }
        (left, Node::Leaf(right)) => {
            compare(left, &Node::Inner(Box::new(vec![Node::Leaf(*right)])))
        }
        (Node::Leaf(left), right) => {
            compare(&Node::Inner(Box::new(vec![Node::Leaf(*left)])), right)
        }
    }
}

fn part1() {
    let mut result = 0;
    let mut pair_num = 1;
    let mut nodes = parse_input();
    while nodes.len() > 1 {
        let left = nodes.remove(0);
        let right = nodes.remove(0);
        let comp = compare(&left, &right);
        match comp {
            Ordering::Equal => result += pair_num,
            Ordering::Less => result += pair_num,
            Ordering::Greater => {}
        }
        pair_num += 1;
    }
    println!("Part1: {result}");
}

fn part2() {
    let mut nodes = parse_input();
    nodes.push(parse_line("[[2]]"));
    nodes.push(parse_line("[[6]]"));
    nodes.sort_by(compare);
    let dividers = vec![parse_line("[[2]]"), parse_line("[[6]]")];
    let result = (0..nodes.len())
        .filter(|&i| dividers.contains(&nodes[i]))
        .map(|i| i + 1)
        .product::<usize>();
    println!("Part2: {result}");
}

fn main() {
    part1();
    part2();
}
