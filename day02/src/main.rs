fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = part_1(input);
    println!("Part 1: {result}");
}

fn part_1(s: &str) -> u64 {
    let mut sum = 0;
    let ranges = parse_to_ranges(s);
    for range in ranges.iter() {
        for num in range.0..=range.1 {
            if is_doubled_sequence(num) {
                sum += num;
            }
        }
    }
    sum
}

fn parse_to_ranges(s: &str) -> Vec<(u64, u64)> {
    s.split(',').map(
        |pair| 
        pair.split_once('-').map(
            |num|
            (num.0.parse::<u64>().expect("num 0 parse error"), 
            num.1.parse::<u64>().expect("num 1 parse error"))
        ).expect("hyphen split error")
    ).collect::<Vec<(u64, u64)>>()
}

fn is_doubled_sequence(num: u64) -> bool {
    if num.ilog10() % 2 != 1 {
        return false;
    }
    return num / 10_u64.pow(num.ilog10() / 2 + 1) == num % 10_u64.pow(num.ilog10() / 2 + 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_02_part_1_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_1(input);
        assert_eq!(result, 1227775554);
    }
}