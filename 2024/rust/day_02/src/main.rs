fn part_01(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let report = line
                .split_ascii_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (report.is_sorted() || report.iter().rev().is_sorted())
                && report
                    .windows(2)
                    .all(|window| (1..=3).contains(&window[0].abs_diff(window[1])))
        })
        .count()
}

mod part_02 {

    pub fn part_02(input: &str) -> usize {
        input
            .lines()
            .filter(|line| {
                let report = line
                    .split_ascii_whitespace()
                    .map(|e| e.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                if is_safe_asc(&report) || is_safe_desc(&report) {
                    return true;
                }

                for i in 0..report.len() {
                    let (left, right) = report.split_at(i);
                    let (_, right) = right.split_first().unwrap();

                    let one_removed = left
                        .iter()
                        .copied()
                        .chain(right.iter().copied())
                        .collect::<Vec<_>>();

                    let (asc_valid, desc_valid) = match (left.last(), right.first()) {
                        (Some(left_last), Some(right_first)) => (
                            (1..=3).contains(&(right_first - left_last)),
                            (1..=3).contains(&(left_last - right_first)),
                        ),
                        (_, _) => (true, true),
                    };

                    if (is_safe_asc(&one_removed) && asc_valid)
                        || (is_safe_desc(&one_removed) && desc_valid)
                    {
                        debug_assert!(
                            one_removed.is_sorted() || one_removed.iter().rev().is_sorted()
                        );
                        return true;
                    }
                }
                false
            })
            .count()
    }

    fn is_safe_asc(report: &[i32]) -> bool {
        report.is_sorted()
            && report
                .windows(2)
                .all(|window| (1..=3).contains(&window[0].abs_diff(window[1])))
    }

    fn is_safe_desc(report: &[i32]) -> bool {
        report.iter().rev().is_sorted()
            && report
                .windows(2)
                .all(|window| (1..=3).contains(&window[0].abs_diff(window[1])))
    }

    #[test]
    fn example_02() {
        assert_eq!(part_02(super::INPUT), 4);
    }
}

#[cfg(test)]
const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

#[test]
fn example_01() {
    assert_eq!(part_01(INPUT), 2);
}

fn main() {
    let s = std::fs::read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_01(&s));
    println!("part 2: {}", part_02::part_02(&s));
}
