use std::collections::HashSet;

fn part_01(input: &str) -> i64 {
    parse_equations(input)
        .into_iter()
        .filter_map(|(test_value, nums)| {
            test_matches_part1(test_value, &nums).then_some(test_value)
        })
        .sum()
}

fn test_matches_part1(test_value: i64, nums: &[i64]) -> bool {
    let mut input = HashSet::<i64>::from([nums[0]]);
    let mut output = HashSet::<i64>::new();

    for &num in nums[1..].iter() {
        for value in input {
            output.insert(value + num);
            output.insert(value * num);
        }
        input = std::mem::take(&mut output);
    }

    input.contains(&test_value)
}

fn part_02(input: &str) -> i64 {
    parse_equations(input)
        .into_iter()
        .filter_map(|(test_value, nums)| {
            test_matches_part2(test_value, &nums).then_some(test_value)
        })
        .sum()
}

fn test_matches_part2(test_value: i64, nums: &[i64]) -> bool {
    let mut input = HashSet::<i64>::from([nums[0]]);
    let mut output = HashSet::<i64>::new();

    for &num in nums[1..].iter() {
        for value in input {
            output.insert(value + num);
            output.insert(value * num);
            output.insert(concat_integer(value, num));
        }
        input = std::mem::take(&mut output);
    }

    input.contains(&test_value)
}

fn concat_integer(left: i64, right: i64) -> i64 {
    let decimal_length = right.checked_ilog10().unwrap_or_default() + 1;
    left * 10i64.pow(decimal_length) + right
}
#[test]
fn test_concat() {
    assert_eq!(concat_integer(5, 6), 56);
    assert_eq!(concat_integer(50, 6), 506);
    assert_eq!(concat_integer(50, 0), 500);
    assert_eq!(concat_integer(0, 10), 10);
    assert_eq!(concat_integer(0, 0), 0);
}

fn parse_equations(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let (key, values) = line.split_once(": ").unwrap();
            (
                key.parse().unwrap(),
                values
                    .split(' ')
                    .map(|value| value.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

#[test]
fn example_01() {
    assert_eq!(part_01(INPUT), 3749);
}

#[test]
fn example_02() {
    assert_eq!(part_02(INPUT), 11387);
}

fn main() {
    let s = std::fs::read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_01(&s));
    println!("part 2: {}", part_02(&s));
}
