fn main() {
    let mut total = 0;
    for line in include_str!("input.txt").lines() {
        total += decrypt(line);
    }
    println!("{total}");
    let result = encrypt(total);
    println!("Part1: {result}");
}

fn decrypt(input: &str) -> i64 {
    let mut result = 0;
    let base: i64 = 5;
    for (i, c) in input.chars().enumerate() {
        let power = (input.len() - 1 - i) as i64;
        let mut d: i64 = if c == '-' {
            -1
        } else if c == '=' {
            -2
        } else {
            c.to_digit(10).unwrap() as i64
        };

        if power > 0 {
            d *= base.pow(power as u32);
        }
        result += d;
    }

    return result;
}
fn format_radix(mut x: i64, radix: i64) -> String {
    let mut result = vec![];

    loop {
        let m = (x % radix) as u32;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix as u32).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

fn encrypt(input: i64) -> String {
    let mut base_5_list: Vec<i64> = format_radix(input, 5)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    while base_5_list.contains(&4) || base_5_list.contains(&3) {
        for i in 0..base_5_list.len() {
            if base_5_list[i] == 4 {
                base_5_list[i] = -1;
                base_5_list[i - 1] += 1;
            } else if base_5_list[i] == 3 {
                base_5_list[i] = -2;
                base_5_list[i - 1] += 1;
            }
        }
    }

    let string_list: Vec<String> = base_5_list
        .iter()
        .map(|num| {
            if *num == -1 {
                return String::from("-");
            } else if *num == -2 {
                return String::from("=");
            } else {
                return num.to_string();
            }
        })
        .collect();

    return string_list.join("");
}
