fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = part_1(input);
    println!("Part 1: {result}");
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_06_part_1_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_1(input);
        assert_eq!(result, 4277556);
    }
}