use std::collections::VecDeque;
use aoc_utils::grid::*;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = part_1(input);
    println!("Part 1: {result}");
    let result = part_2(input);
    println!("Part 2: {result}");
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

fn part_2(s: &str) -> u64 {
    let g = s.parse().unwrap();
    count_timelines_wrapper(&g)
}

fn find_mapping(v: &Vec<Position>, p: Position) -> Option<usize> {
    for (i, x) in v.iter().enumerate() {
        if *x == p {
            return Some(i);
        }
    }
    None
}

fn count_timelines_wrapper(g: &Grid) -> u64 {
    if let Some(source) = g.find(&'S') {
        let mut subtimelines = Vec::new();
        let mut mapping = Vec::new();
        count_timelines(g, source, &mut mapping, &mut subtimelines)
    } else {
        0
    }
}

fn count_timelines(g: &Grid, p: Position, mapping: &mut Vec<Position>, subtimelines: &mut Vec<u64>) -> u64 {
    if g[p] == '.' || g[p] == 'S' {
        let mut p_i = p;
        while let Ok((p, c)) = traverse_grid(g, &p_i, &Direction::South) {
            p_i = p;
            if c == '.' {
                continue;
            } else if c == '^' {
                return count_timelines(g, p_i, mapping, subtimelines);
            }
        }
        1
    } else if g[p] == '^' {
        if let Some(i) = find_mapping(&mapping, p) {
            subtimelines[i]
        } else {
            let mut count = 0;
            if let Ok((r, _)) = traverse_grid(g, &p, &Direction::East) {
                count += count_timelines(g, r, mapping, subtimelines);
            }
            if let Ok((l, _)) = traverse_grid(g, &p, &Direction::West) {
                count += count_timelines(g, l, mapping, subtimelines);
            }
            mapping.push(p);
            subtimelines.push(count);
            count
        }
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

    #[test]
    fn day_07_part_2_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_2(input);
        assert_eq!(result, 40);
    }
}