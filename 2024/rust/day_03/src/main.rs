fn part_01(input: &str) -> usize {
    let pattern = regex::Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();

    pattern
        .captures_iter(input)
        .map(|cap| {
            let left = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let right = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();

            left * right
        })
        .sum()
}

fn part_02(input: &str) -> usize {
    let pattern = regex::Regex::new(r#"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();

    let mut do_mul = true;
    let mut acc = 0;

    for captures in pattern.captures_iter(input) {
        match captures.get(0).unwrap().as_str() {
            "do()" => do_mul = true,
            "don't()" => do_mul = false,
            _ => {
                let left: usize = captures.get(1).unwrap().as_str().parse().unwrap();
                let right: usize = captures.get(2).unwrap().as_str().parse().unwrap();
                if do_mul {
                    acc += left * right;
                }
            }
        }
    }

    acc
}

#[test]
fn example_01() {
    const INPUT: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    assert_eq!(part_01(INPUT), 161);
}

#[test]
fn example_02() {
    const INPUT: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    assert_eq!(part_02(INPUT), 48);
}

fn main() {
    let s = std::fs::read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_01(&s));
    println!("part 2: {}", part_02(&s));
}
