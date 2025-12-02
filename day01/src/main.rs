fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = part_1(input);
    println!("Part 1: {result}");
}

fn part_1(s: &str) -> i32 {
    let mut count = 0;
    let mut dial: i32 = 50;
    for line in s.lines() {
        dial = (dial + turn(line)) % 100;
        if dial == 0 {
            count += 1;
        }
    }
    count
}

fn turn(s: &str) -> i32 {
    (match s.chars().next() {
        Some('R') => 1,
        Some('L') => -1,
        Some(_) | None => panic!("Parse error on first char of {s}")
    } * s.chars().skip(1).collect::<String>().parse::<i32>().unwrap() % 100)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_01_part_1_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_1(input);
        assert_eq!(result, 3);
    }
}