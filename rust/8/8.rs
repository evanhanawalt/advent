fn read_input() -> Vec<Vec<u32>> {
    let mut trees: Vec<Vec<u32>> = Vec::new();
    for line in include_str!("input.txt").split("\n") {
        let mut row: Vec<u32> = Vec::new();
        for tree in line.chars() {
            row.push(tree.to_digit(10).unwrap());
        }
        trees.push(row);
    }

    return trees;
}

fn count_visible(trees: Vec<Vec<u32>>) -> u32 {
    let mut count = 0;

    for (row_index, row) in trees.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            let mut other_col = col_index;
            let mut check = true;
            'left: while other_col > 0 {
                other_col -= 1;
                if value <= &trees[row_index][other_col] {
                    check = false;
                    break 'left;
                }
            }
            if check {
                count += 1;
                continue;
            }
            check = true;
            other_col = col_index;
            'right: while other_col < row.len() - 1 {
                other_col += 1;
                if value <= &trees[row_index][other_col] {
                    check = false;
                    break 'right;
                }
            }
            if check {
                count += 1;
                continue;
            }
            check = true;
            let mut other_row = row_index;
            'top: while other_row > 0 {
                other_row -= 1;
                if value <= &trees[other_row][col_index] {
                    check = false;
                    break 'top;
                }
            }
            if check {
                count += 1;
                continue;
            }
            check = true;
            other_row = row_index;
            'bottom: while other_row < trees.len() - 1 {
                other_row += 1;
                if value <= &trees[other_row][col_index] {
                    check = false;
                    break 'bottom;
                }
            }
            if check {
                count += 1;
                continue;
            }
        }
    }
    return count;
}

fn get_best_tree(trees: Vec<Vec<u32>>) -> u32 {
    let mut result = 0;

    for (row_index, row) in trees.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            let mut check_left: u32 = 0;
            let mut check_right: u32 = 0;
            let mut check_top: u32 = 0;
            let mut check_bottom: u32 = 0;

            let mut other_col = col_index;

            'left: while other_col > 0 {
                other_col -= 1;
                check_left += 1;
                if value <= &trees[row_index][other_col] {
                    break 'left;
                }
            }

            other_col = col_index;
            'right: while other_col < row.len() - 1 {
                other_col += 1;
                check_right += 1;
                if value <= &trees[row_index][other_col] {
                    break 'right;
                }
            }

            let mut other_row = row_index;
            'top: while other_row > 0 {
                other_row -= 1;
                check_top += 1;
                if value <= &trees[other_row][col_index] {
                    break 'top;
                }
            }
            other_row = row_index;
            'bottom: while other_row < trees.len() - 1 {
                other_row += 1;
                check_bottom += 1;
                if value <= &trees[other_row][col_index] {
                    break 'bottom;
                }
            }

            let vis_score = check_left * check_right * check_top * check_bottom;
            if vis_score > result {
                result = vis_score;
            }
        }
    }

    return result;
}

fn part1() {
    let trees = read_input();
    let result = count_visible(trees);
    println!("Part1: {result}");
}

fn part2() {
    let trees = read_input();
    let result = get_best_tree(trees);
    println!("Part2: {result}");
}

fn main() {
    part1();
    part2();
}
