fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = part_1(input);
    println!("Part 1: {result}");
}

fn part_1(s: &str) -> u64 {
    let mut count = 0;
    let (f, available_ids) = parse_to_inventory(s);
    for id in available_ids {
        if id_in_ranges(id, &f)  {
            count += 1;
        }
    }
    count
}

fn id_in_ranges(id: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for (lower, upper) in ranges {
        if id <= *upper && id >= *lower {
            return true;
        }
    }
    false
}

fn parse_to_inventory(s: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut fresh_section = true;
    let mut fresh_ids = Vec::new();
    let mut available_ids = Vec::new();
    for line in s.lines() {
        if fresh_section {
            if line.is_empty() {
                fresh_section = false;
            } else {
                fresh_ids.push(line.split_once('-').map(
                    |num|
                    (num.0.parse::<u64>().expect("num 0 parse error"), 
                    num.1.parse::<u64>().expect("num 1 parse error"))
                ).expect("hyphen split error"));
            }
        } else {
            available_ids.push(line.parse().unwrap());
        }
    }
    (fresh_ids, available_ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_05_part_1_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_1(input);
        assert_eq!(result, 3);
    }
}