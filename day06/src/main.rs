use aoc_utils::grid::*;
use std::str::FromStr;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = part_1(input);
    println!("Part 1: {result}");
    let result = part_2(input);
    println!("Part 2: {result}");
}

fn part_1(s: &str) -> u64 {
    let mut nums: Vec<Vec<u64>> = Vec::new();
    let mut ops = Vec::new();
    for line in s.lines() {
        for (col, item) in line.split_whitespace().enumerate() {
            if item != "*" && item != "+" {
                if let Some(eqn) = nums.get_mut(col) {
                    eqn.push(item.parse::<u64>().unwrap());
                } else {
                    nums.push(vec![item.parse::<u64>().unwrap()])
                }
            } else {
                if item == "+" {
                    ops.push('+');
                } else if item == "*" {
                    ops.push('*');
                } else {
                    println!("Ops matching failed");
                }
            }
        }
    }
    let mut sum = 0;
    for (operands, operator) in nums.iter().zip(ops) {
        if operator == '*' {
            sum += operands.iter().fold(1, |acc, operand| acc * operand)
        } else if operator == '+' {
            sum += operands.iter().fold(0, |acc, operand| acc + operand)
        } else {
            panic!("Bad operator in list");
        }
    }
    sum
}

fn part_2(s: &str) -> u64 {
    let g = Grid::from_str(&s).unwrap();
    let (nums, ops) = cephalopod_equations(&g);
    let mut sum = 0;
    for (operands, operator) in nums.iter().zip(ops) {
        if operator == '*' {
            sum += operands.iter().fold(1, |acc, operand| acc * operand)
        } else if operator == '+' {
            sum += operands.iter().fold(0, |acc, operand| acc + operand)
        } else {
            panic!("Bad operator in list");
        }
    }
    sum
}

fn cephalopod_equations(g: &Grid) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut nums = Vec::new();
    nums.push(Vec::new());
    let mut ops = Vec::new();
    let mut eq_index = 0;
    'col: for col in (0..g.len().x).rev() {
        let mut digit_collector = "".to_string();
        let mut y = 0;
        let c = g[Position{x: col, y}];
        if c != ' ' {
                digit_collector.extend([c]);
        }
        '_row: while let Ok((pos, c)) = traverse_grid(g, &Position {x: col, y: y}, &Direction::South) {
            y = pos.y;
            if c == ' ' {
                continue;
            } else if c == '*' || c == '+' {
                if digit_collector.len() > 0 {
                    if let Some(eqn) = nums.get_mut(eq_index) {
                        eqn.push(digit_collector.parse::<u64>().unwrap());
                    } else {
                        nums.push(vec![digit_collector.parse::<u64>().unwrap()])
                    }
                }
                ops.push(c);
                eq_index += 1;
                continue 'col;
            } else {
                digit_collector.extend([c]);
            }
        }
        if digit_collector.len() > 0 {
            if let Some(eqn) = nums.get_mut(eq_index) {
                eqn.push(digit_collector.parse::<u64>().unwrap());
            } else {
                nums.push(vec![digit_collector.parse::<u64>().unwrap()])
            }
        }
    }
    (nums, ops)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_06_part_1_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_1(input);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn day_06_part_2_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_2(input);
        assert_eq!(result, 3263827);
    }
}