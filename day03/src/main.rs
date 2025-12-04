fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = part_1(input);
    println!("Part 1: {result}");
}

fn part_1(s: &str) -> u64 {
    let battery = parse_to_battery(s);
    let mut sum = 0;
    for bank in battery {
        let mut first_max = 0;
        let mut max_index = 0;
        for i in 0..bank.len() {
            if i == bank.len() - 1 {
                break;
            }
            if bank[i] > first_max {
                first_max = bank[i];
                max_index = i;
            }
        }
        let mut second_max = 0;
        for i in max_index+1..bank.len() {
            if bank[i] > second_max {
                second_max = bank[i];
            }
        }
        sum += first_max * 10 + second_max;
    }
    sum
}

fn parse_to_battery(s: &str) -> Vec<Vec<u64>> {
    let mut battery = Vec::new();
    for line in s.lines() {
        let mut bank = Vec::new();
        for joltage in line.chars() {
            bank.push(joltage.to_digit(10).unwrap() as u64);
        }
        battery.push(bank);
    }
    battery
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_01_part_1_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_1(input);
        assert_eq!(result, 357);
    }
}