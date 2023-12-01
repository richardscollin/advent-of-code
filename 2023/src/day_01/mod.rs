//! Day 1: Trebuchet?!
//! <https://adventofcode.com/2023/day/1>
#[test]
fn test_part1() {
    // combine the first and last ascii digit of each line into a single two digit number
    // and sum each line
    // 1abc2       ->   12
    // pqr3stu8vwx -> + 38
    //                ----
    //                  50
    #[rustfmt::skip]
    assert_eq!(part1("
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
    ".trim()), 142);

    assert_eq!(part1(include_str!("input.txt")), 54953);
    assert_eq!(part1_functional(include_str!("input.txt")), 54953);
}

#[test]
fn test_part2() {
    // same as above, but now include english words one - nine
    // as well as ascii digits
    // zoneight234 -> 14
    #[rustfmt::skip]
    assert_eq!(part2("
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
    ".trim()), 281);

    assert_eq!(part2(include_str!("input.txt")), 53868);
    assert_eq!(part2_functional(include_str!("input.txt")), 53868);
}

fn part1(s: &str) -> u32 {
    let mut acc: u32 = 0;

    for line in s.lines() {
        let first = line.chars().find(char::is_ascii_digit).unwrap() as u8 - b'0';
        let last = line.chars().rev().find(char::is_ascii_digit).unwrap() as u8 - b'0';
        let num = 10 * (first as u32) + last as u32;
        acc += num;
    }

    acc
}

fn part1_functional(s: &str) -> u32 {
    s.lines()
        .filter_map(|line| {
            let first = line.chars().find(char::is_ascii_digit)?.to_digit(10)?;
            let last = line.chars().rfind(char::is_ascii_digit)?.to_digit(10)?;
            Some(10 * first + last)
        })
        .sum()
}

fn part2(s: &str) -> u32 {
    const WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut acc = 0;

    for line in s.lines() {
        let mut nums = vec![];

        for (i, c) in line.chars().enumerate() {
            if let Some(c) = c.to_digit(10) {
                nums.push(c);
            } else {
                for (wi, word) in WORDS.iter().enumerate() {
                    if line[i..].starts_with(word) {
                        nums.push(1 + wi as u32);
                        break;
                    }
                }
            }
        }

        acc += *nums.first().unwrap() * 10 + *nums.last().unwrap();
    }

    acc
}

fn part2_functional(s: &str) -> u32 {
    const WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    s.lines()
        .map(|line| {
            let mut nums = line.chars().enumerate().filter_map(|(i, c)| {
                c.to_digit(10).or_else(|| {
                    WORDS
                        .iter()
                        .enumerate()
                        .find_map(|(n, word)| line[i..].starts_with(word).then_some(1 + n as u32))
                })
            });

            let first = nums.next().unwrap();
            first * 10 + nums.last().unwrap_or(first)
        })
        .sum()
}
