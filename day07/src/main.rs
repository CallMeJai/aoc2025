use std::collections::VecDeque;
use aoc_utils::grid::*;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = part_1(input);
    println!("Part 1: {result}");
}

fn part_1(s: &str) -> u64 {
    let mut g = s.parse::<Grid>().unwrap();
    count_splits(&mut g)
}

fn count_splits(g: &mut Grid) -> u64 {
    if let Some(source) = g.find(&'S') {
        let mut count = 0;
        let mut beams = VecDeque::new();
        beams.push_back(source);
        while let Some(beam) = beams.pop_front() {
            if let Ok((p, c)) = traverse_grid(g, &beam, &Direction::South) {
                if c == '.' {
                    beams.push_back(p);
                    g[p] = '|';
                } else if c == '^' {
                    count += 1;
                    if let Ok((l, c)) = traverse_grid(g, &p, &Direction::West)
                    && c != '|' {
                        beams.push_back(l);
                        g[l] = '|';
                    }
                    if let Ok((r, c)) = traverse_grid(g, &p, &Direction::East)
                    && c != '|' {
                        beams.push_back(r);
                        g[r] = '|';
                    }
                }
            }
        }
        count
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_07_part_1_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_1(input);
        assert_eq!(result, 21);
    }
}