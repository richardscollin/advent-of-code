// https://adventofcode.com/2024/day/5
use std::collections::HashSet;

fn parse_rules_updates(input: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules: HashSet<(i32, i32)> = rules
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('|').unwrap();
            (right.parse::<i32>().unwrap(), left.parse::<i32>().unwrap()) // rule is flipped to make it easy to detect if broken
        })
        .collect();

    let updates: Vec<Vec<i32>> = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    (rules, updates)
}

fn part_01(input: &str) -> i32 {
    let (rules, updates) = parse_rules_updates(input);

    updates
        .iter()
        .filter(|update| !is_misordered(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn is_misordered(update: &[i32], rules: &HashSet<(i32, i32)>) -> bool {
    for i in 0..(update.len() - 1) {
        for j in (i + 1)..update.len() {
            if rules.contains(&(update[i], update[j])) {
                return true;
            }
        }
    }
    false
}

fn part_02(input: &str) -> i32 {
    let (rules, mut updates) = parse_rules_updates(input);

    let mut acc: i32 = 0;
    for update in &mut updates {
        if is_misordered(update, &rules) {
            make_ordered(update, &rules);
            acc += update[update.len() / 2];
        }
    }

    acc
}

// this makes the elements sorted by iteratively swapping elements which
// are misordered. I'm not actually sure if this algorithm is guaranteed
// to terminate, so I've added a max iteration amount.
fn make_ordered(update: &mut Vec<i32>, rules: &HashSet<(i32, i32)>) {
    let mut iterations = 0;
    let orig = update.clone();

    'outer: while iterations < 1000 {
        for i in 0..(update.len() - 1) {
            for j in (i + 1)..update.len() {
                if rules.contains(&(update[i], update[j])) {
                    update.swap(i, j);
                    iterations += 1;

                    continue 'outer;
                }
            }
        }

        return; // it's sorted
    }
    panic!("max iterations reached: {orig:?}\n{update:?}");
}

#[cfg(test)]
const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

#[test]
fn example_01() {
    assert_eq!(part_01(INPUT), 143);
}

#[test]
fn example_02() {
    assert_eq!(part_02(INPUT), 123);
}

fn main() {
    let s = std::fs::read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_01(&s));
    println!("part 2: {}", part_02(&s));
}
