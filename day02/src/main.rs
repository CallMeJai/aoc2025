fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = part_1(input);
    println!("Part 1: {result}");
    let result = part_2(input);
    println!("Part 2: {result}");
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

fn part_2(s: &str) -> u64 {
    let mut sum = 0;
    let ranges = parse_to_ranges(s);
    for range in ranges.iter() {
        for num in range.0..=range.1 {
            if is_repeated_sequence(num) {
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

fn is_repeated_sequence(num: u64) -> bool {
    if num < 10 {
        return false;
    }
    'chunk_size: for sequence_length in 1..=(num.ilog10() as usize / 2 + 1) {
        if let Some(chunks) = num_into_chunks(num, sequence_length) {
            let start = chunks[0];
            for chunk in chunks.into_iter() {
                if chunk != start {
                    continue 'chunk_size;
                }
            }
            return true;
        }
    }
    false
}

fn num_into_chunks(num: u64, len: usize) -> Option<Vec<u64>> {
    let mut v = Vec::new();
    let mut x = num;
    if len > num.ilog10() as usize + 1 {
        None
    } else if (num.ilog10() as usize + 1) % len != 0 {
        None
    } else {
        while x != 0 {
            v.push(x % 10_u64.pow(len as u32));
            x /= 10_u64.pow(len as u32);
        }
        v.reverse();
        Some(v)
    }
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

    #[test]
    fn day_02_part_2_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_2(input);
        assert_eq!(result, 4174379265);
    }
}