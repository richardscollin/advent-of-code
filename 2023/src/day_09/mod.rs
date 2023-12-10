fn find_diff(v: &[isize]) -> Vec<isize> {
    let it1 = v.iter();
    let it2 = v.iter().skip(1);
    it2.zip(it1).map(|(a, b)| a - b).collect()
}

fn predict(mut v: Vec<isize>) -> isize {
    let mut lasts: Vec<isize> = vec![];
    lasts.push(*v.last().unwrap());

    loop {
        let diffs = find_diff(&v);
        lasts.push(*diffs.last().unwrap());

        if diffs.iter().all(|e| *e == 0) {
            break;
        }

        v = diffs;
    }

    lasts.iter().sum()
}

fn predict_front(mut v: Vec<isize>) -> isize {
    let mut firsts: Vec<isize> = vec![];
    firsts.push(*v.first().unwrap());

    loop {
        let diffs = find_diff(&v);
        firsts.push(*diffs.first().unwrap());

        if diffs.iter().all(|e| *e == 0) {
            break;
        }

        v = diffs;
    }

    let mut curr = firsts.pop().unwrap();

    while let Some(e) = firsts.pop() {
        curr = e - curr;
    }

    curr
}

pub fn part1(s: &str) -> isize {
    s.lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .filter_map(|num| num.parse::<isize>().ok())
                .collect::<Vec<_>>();

            predict(numbers)
        })
        .sum()
}

pub fn part2(s: &str) -> isize {
    s.lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .filter_map(|num| num.parse::<isize>().ok())
                .collect::<Vec<_>>();

            predict_front(numbers)
        })
        .sum()
}

const SAMPLE: &str = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE.trim()), 114);
    assert_eq!(part1(include_str!("input.txt")), 1834108701);
}

#[test]
fn test_part2() {
    assert_eq!(part2(SAMPLE.trim()), 2);
    assert_eq!(part2(include_str!("input.txt")), 993);
}
