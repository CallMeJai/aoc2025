use aoc_utils::grid::*;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = part_1(input);
    println!("Part 1: {result}");
}

fn part_1(s: &str) -> u64 {
    let g: Grid = s.parse().expect("Cannot parse to grid");
    let grid_len = g.len();
    let mut count = 0;
    for x in 0..grid_len.x {
        for y in 0..grid_len.y {
            let p = Position {x, y};
            if g[p] == '@' && num_adjacent_rolls(&g, p) < 4 {
                count += 1;
            }
        }
    }
    count
}

fn num_adjacent_rolls(g: &Grid, p: Position) -> u8 {
    let mut num_rolls = 0;
    for d in Direction::iterator() {
        if let Ok((_p, c)) = traverse_grid(g, &p, d) {
            if c == '@' {
                num_rolls += 1;
            }
        }
    }
    num_rolls
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_04_part_1_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_1(input);
        assert_eq!(result, 13);
    }
}