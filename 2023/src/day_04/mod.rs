//! Day 4: Scratchcards
//! <https://adventofcode.com/2023/day/4>
use std::collections::{HashMap, HashSet};

fn part1(s: &str) -> usize {
    let mut acc = 0;
    for card in s.lines() {
        let (_, card) = card.split_once(": ").unwrap();
        let (winning, ours) = card.split_once(" | ").unwrap();

        let winning: HashSet<usize> = winning
            .split(' ')
            .filter_map(|num| num.parse().ok())
            .collect();

        let matches = ours
            .split(' ')
            .filter_map(|num| num.parse().ok())
            .filter(|num| winning.contains(num))
            .count();

        if matches != 0 {
            acc += 2usize.pow(matches as u32 - 1);
        }
    }
    acc
}

fn part2(s: &str) -> usize {
    let mut mult: HashMap<usize, usize> = HashMap::new(); // default value of 1

    for (card_id, card) in s.lines().enumerate() {
        let (_, card) = card.split_once(": ").unwrap();
        let (winning, ours) = card.split_once(" | ").unwrap();

        let winning: HashSet<usize> = winning
            .split(' ')
            .filter_map(|num| num.parse().ok())
            .collect();

        let matches = ours
            .split(' ')
            .filter_map(|num| num.parse().ok())
            .filter(|num| winning.contains(num))
            .count();

        let multipier = *mult.entry(card_id).or_insert(1);

        for i in (card_id + 1)..(card_id + 1 + matches) {
            *mult.entry(i).or_insert(1) += multipier;
        }
    }
    mult.values().sum()
}

const SAMPLE: &str = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE.trim()), 13);
    assert_eq!(part1(include_str!("input.txt")), 25183);
}

#[test]
fn test_part2() {
    assert_eq!(part2(SAMPLE.trim()), 30);
    assert_eq!(part2(include_str!("input.txt")), 5667240);
}
