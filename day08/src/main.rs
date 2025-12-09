use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = part_1(input, 1000);
    println!("Part 1: {result}");
    let result = part_2(input);
    println!("Part 2: {result}");
}

fn part_1(s: &str, n: usize) -> usize {
    let mut boxes = Vec::new();
    let mut circuits = Vec::new();
    for line in s.lines() {
        let mut nums = Vec::new();
        for split in line.splitn(3, ',') {
            nums.push(split.parse().unwrap());
        }
        boxes.push(Coord3D{x: nums[0], y: nums[1], z: nums[2]});
        circuits.push(vec![Coord3D{x: nums[0], y: nums[1], z: nums[2]}]);
    }
    let mut connections = Vec::new();
    let mut pair_dists = BinaryHeap::new();
    for i in 0..boxes.len()-1 {
        for j in i+1..boxes.len() {
            pair_dists.push((Reverse(euclidean_distance(boxes[i], boxes[j])), (boxes[i], boxes[j])));
        }
    }
    for _ in 0..n {
        let (_, (box_1, box_2)) = pair_dists.pop().unwrap();
        connections.push((box_1, box_2));
        update_circuits(box_1, box_2, &mut circuits);
    }
    let (top_3, _, _) = circuits.select_nth_unstable_by(3, |a, b| b.len().cmp(&a.len()));
    top_3.iter().map(|c| c.len()).product()
}

fn part_2(s: &str) -> i64 {
    let mut boxes = Vec::new();
    let mut circuits = Vec::new();
    for line in s.lines() {
        let mut nums = Vec::new();
        for split in line.splitn(3, ',') {
            nums.push(split.parse().unwrap());
        }
        boxes.push(Coord3D{x: nums[0], y: nums[1], z: nums[2]});
        circuits.push(vec![Coord3D{x: nums[0], y: nums[1], z: nums[2]}]);
    }
    let mut connections = Vec::new();
    let mut pair_dists = BinaryHeap::new();
    for i in 0..boxes.len()-1 {
        for j in i+1..boxes.len() {
            pair_dists.push((Reverse(euclidean_distance(boxes[i], boxes[j])), (boxes[i], boxes[j])));
        }
    }
    let mut pair_prod = 0;
    while circuits.len() > 1 {
        let (_, (box_1, box_2)) = pair_dists.pop().unwrap();
        connections.push((box_1, box_2));
        update_circuits(box_1, box_2, &mut circuits);
        pair_prod = box_1.x * box_2.x;
    }
    pair_prod
}

fn euclidean_distance(a: Coord3D, b: Coord3D) -> i128 {
    (((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2))).isqrt().into()
}

fn update_circuits(a: Coord3D, b: Coord3D, circuits: &mut Vec<Vec<Coord3D>>) {
    let a_idx = circuits.iter().position(
        |circuit| 
        circuit.iter().any(
            |x|
            *x == a
        )
    ).unwrap();
    let b_idx = circuits.iter().position(
        |circuit| 
        circuit.iter().any(
            |x|
            *x == b
        )
    ).unwrap();
    if a_idx < b_idx {
        let (a_slice, b_slice) = circuits.split_at_mut(b_idx);
        a_slice[a_idx].append(&mut b_slice[0]);
        circuits.swap_remove(b_idx);
    } else if a_idx > b_idx {
        let (b_slice, a_slice) = circuits.split_at_mut(a_idx);
        b_slice[b_idx].append(&mut a_slice[0]);
        circuits.swap_remove(a_idx);
    }
}

#[derive(PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
struct Coord3D {
    x: i64,
    y: i64,
    z: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_08_part_1_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_1(input, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn day_08_part_2_test() {
        let input = include_str!("../inputs/sample.txt");
        let result = part_2(input);
        assert_eq!(result, 25272);
    }
}