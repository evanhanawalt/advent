use std::collections::HashMap;

#[derive(Debug)]
enum Line {
    Value(i64),
    Equation((String, String, String)),
    Key(String),
}

#[derive(Debug, Clone, PartialEq)]
enum Evaluation {
    Solved(i64),
    Key(String),
    UnSolved(Box<(Evaluation, String, Evaluation)>),
}

fn main() {
    part1();
    part2();
}

fn read_input() -> HashMap<String, Line> {
    let mut results: HashMap<String, Line> = HashMap::new();
    for line in include_str!("input.txt").lines() {
        let replaced = line.replace(":", "");
        let split: Vec<&str> = replaced.split_whitespace().collect();
        match split.len() {
            2 => {
                results.insert(
                    split[0].to_string(),
                    Line::Value(split[1].parse::<i64>().unwrap()),
                );
            }
            4 => {
                results.insert(
                    split[0].to_string(),
                    Line::Equation((
                        split[1].to_string(),
                        split[2].to_string(),
                        split[3].to_string(),
                    )),
                );
            }
            _ => {
                panic!("Can't Parse Input?")
            }
        }
    }
    return results;
}

fn calculate_result(lookup: &HashMap<String, Line>, token: &String) -> i64 {
    let value = lookup.get(token).unwrap();
    match value {
        Line::Value(v) => return *v,
        Line::Equation((left, operation, right)) => {
            if String::from("/").eq(operation) {
                return calculate_result(lookup, left) / calculate_result(lookup, right);
            } else if String::from("*").eq(operation) {
                return calculate_result(lookup, left) * calculate_result(lookup, right);
            } else if String::from("-").eq(operation) {
                return calculate_result(lookup, left) - calculate_result(lookup, right);
            } else if String::from("+").eq(operation) {
                return calculate_result(lookup, left) + calculate_result(lookup, right);
            } else {
                panic!("There isn't a valid operation??");
            }
        }
        _ => {
            panic!("This should be solvable");
        }
    }
}

fn create_unsolved_equation(lookup: &HashMap<String, Line>, token: &String) -> Evaluation {
    let value = lookup.get(token).unwrap();
    match value {
        Line::Value(v) => return Evaluation::Solved(*v),
        Line::Key(key) => return Evaluation::Key(key.clone()),
        Line::Equation((left, operation, right)) => {
            let left_eval = create_unsolved_equation(lookup, left);
            let right_eval = create_unsolved_equation(lookup, right);

            match (left_eval, right_eval) {
                (Evaluation::Solved(left), Evaluation::Solved(right)) => {
                    if String::from("/").eq(operation) {
                        return Evaluation::Solved(left / right);
                    } else if String::from("*").eq(operation) {
                        return Evaluation::Solved(left * right);
                    } else if String::from("-").eq(operation) {
                        return Evaluation::Solved(left - right);
                    } else if String::from("+").eq(operation) {
                        return Evaluation::Solved(left + right);
                    } else {
                        panic!("There isn't a valid operation??");
                    }
                }
                (left, right) => {
                    return Evaluation::UnSolved(Box::new((left, operation.clone(), right)));
                }
            }
        }
    }
}

fn part1() {
    let lines = read_input();
    let result = calculate_result(&lines, &String::from("root"));
    println!("Part1: {result}");
}

fn part2() {
    let mut lines = read_input();
    let key = String::from("humn");
    lines.insert(key.clone(), Line::Key(key));
    let unsolved = create_unsolved_equation(&lines, &String::from("root"));

    // figure out which side is a number..
    if let Evaluation::UnSolved(a) = unsolved {
        let (left, _op, right) = a.as_ref();

        let (mut number, mut equation) = if let Evaluation::Solved(l_num) = left {
            (*l_num, right)
        } else if let Evaluation::Solved(r_num) = right {
            (*r_num, left)
        } else {
            panic!("one side should be solved??");
        };

        // unwind the equation with algebra until 'equation' = humn
        loop {
            match equation {
                Evaluation::UnSolved(a) => {
                    let (left, operation, right) = a.as_ref();
                    match (left, right) {
                        (Evaluation::Solved(left), right) => {
                            if String::from("+").eq(operation) {
                                number = number - *left;
                            } else if String::from("-").eq(operation) {
                                number = -1 * (number - left);
                            } else if String::from("*").eq(operation) {
                                number = number / *left;
                            } else if String::from("/").eq(operation) {
                                number = *left / number;
                            }
                            equation = right;
                        }
                        (left, Evaluation::Solved(right)) => {
                            if String::from("+").eq(operation) {
                                number = number - *right;
                            } else if String::from("-").eq(operation) {
                                number = number + *right;
                            } else if String::from("*").eq(operation) {
                                number = number / *right;
                            } else if String::from("/").eq(operation) {
                                number = number * *right;
                            }
                            equation = left;
                        }
                        _ => panic!("Error unwinding equation"),
                    }
                }
                Evaluation::Solved(_) => {
                    panic!("Equation should become a key, or an unsolved, not a solved equation");
                }
                Evaluation::Key(_) => {
                    break;
                }
            }
        }
        println!("Part2: {number}");
    } else {
        panic!("Equation should be unsolved?");
    }
}
