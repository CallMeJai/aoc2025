use std::cmp::Ordering;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = part_1(input);
    println!("Part 1: {result}");
    let result = part_2(input);
    println!("Part 2: {result}");
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

fn part_2(s: &str) -> u64 {
    let mut count = 0;
    let (mut ranges, _) = parse_to_inventory(s);
    let mut range_len = usize::MAX;
    while range_len != ranges.len() {
        range_len = ranges.len();
        ranges = fuse_ranges(ranges);
    }
    for (start, end) in ranges {
        count += 1 + end - start;
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

fn fuse_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut fused: Vec<(u64, u64)> = Vec::new();
    for range in ranges {
        let mut found = Vec::new();
        for i in 0..fused.len() {
            match calculate_overlap(fused[i], range) {
                OverlapCase::TwoContainsOne => {fused[i] = range; found.push((i, OverlapCase::TwoContainsOne));},
                OverlapCase::OneStartsTwoEnds | OverlapCase::SameStartTwoEnds => found.push((i, OverlapCase::OneStartsTwoEnds)),
                OverlapCase::TwoStartsOneEnds | OverlapCase::TwoStartsSameEnd => found.push((i, OverlapCase::TwoStartsOneEnds)),
                OverlapCase::Equivalent | OverlapCase::OneContainsTwo | OverlapCase::SameStartOneEnds | OverlapCase::OneStartsSameEnd => found.push((i, OverlapCase::Equivalent)),
                OverlapCase::NoOverlap => ()
            };
        }
        if found.len() == 0 {
            fused.push(range);
        }
        for (i, case) in found.iter() {
            match case {
                OverlapCase::TwoContainsOne => fused[*i] = range,
                OverlapCase::OneStartsTwoEnds => fused[*i].1 = range.1,
                OverlapCase::TwoStartsOneEnds => fused[*i].0 = range.0,
                OverlapCase::Equivalent => (),
                _ => panic!("Bad case in found vector")
            }
        }
    }
    fused
}

enum OverlapCase {
    NoOverlap,
    Equivalent,
    OneContainsTwo,
    TwoContainsOne,
    SameStartTwoEnds,
    SameStartOneEnds,
    OneStartsSameEnd,
    TwoStartsSameEnd,
    OneStartsTwoEnds,
    TwoStartsOneEnds,
}

fn calculate_overlap(r_1: (u64, u64), r_2: (u64, u64)) -> OverlapCase {
    match (r_1.0.cmp(&r_2.0), r_1.0.cmp(&r_2.1), 
        r_1.1.cmp(&r_2.0), r_1.1.cmp(&r_2.1))  {
        (Ordering::Equal, _, _, Ordering::Equal) 
            => OverlapCase::Equivalent,
        (_, _, Ordering::Less, _) | (_, Ordering::Greater, _, _) 
            => OverlapCase::NoOverlap,
        (Ordering::Less, _, _, Ordering::Greater)
            => OverlapCase::OneContainsTwo,
        (Ordering::Greater, _, _, Ordering::Less)
            => OverlapCase::TwoContainsOne,
        (Ordering::Equal, _, _, Ordering::Greater)
            => OverlapCase::SameStartOneEnds,
        (Ordering::Equal, _, _, Ordering::Less)
            => OverlapCase::SameStartTwoEnds,
        (Ordering::Less, _, _, Ordering::Equal)
            => OverlapCase::OneStartsSameEnd,
        (Ordering::Greater, _, _, Ordering::Equal)
            => OverlapCase::TwoStartsSameEnd,
        (Ordering::Less, _, _, Ordering::Less)
            => OverlapCase::OneStartsTwoEnds,
        (Ordering::Greater, _, _, Ordering::Greater)
            => OverlapCase::TwoStartsOneEnds,
    }
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

    #[test]
    fn day_05_part_2_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_2(input);
        assert_eq!(result, 14);
    }
}