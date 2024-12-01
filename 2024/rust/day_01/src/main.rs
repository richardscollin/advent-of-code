fn part_01(input: &str) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>()
}

fn part_02(input: &str) -> u32 {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .unzip();

    let right = utils::count(right);

    left.into_iter()
        .map(|k| k as usize * (right.get(&k).copied().unwrap_or_default()))
        .sum::<usize>() as u32
}

#[cfg(test)]
const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

#[test]
fn example_01() {
    assert_eq!(part_01(INPUT), 11);
}

#[test]
fn example_02() {
    assert_eq!(part_02(INPUT), 31);
}

fn main() {
    let s = std::fs::read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_01(&s));
    println!("part 2: {}", part_02(&s));
}

mod utils {
    use std::collections::HashMap;

    pub fn count<I>(values: I) -> HashMap<I::Item, usize>
    where
        I: IntoIterator,
        I::Item: Eq + std::hash::Hash,
    {
        let mut map = HashMap::new();
        values.into_iter().fold(&mut map, |acc, item| {
            *acc.entry(item).or_default() += 1;
            acc
        });
        map
    }
}
