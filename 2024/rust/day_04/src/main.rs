use std::collections::HashMap;

fn part_01(input: &str) -> usize {
    let grid: Vec<Vec<u8>> = input.lines().map(|e| e.to_string().into_bytes()).collect();

    const WORD: &str = "XMAS";

    #[rustfmt::skip]
    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    let mut matches = 0;
    for direction in DIRECTIONS {
        for (r, row) in grid.iter().enumerate() {
            'start: for (c, _) in row.iter().enumerate() {
                let mut location = (r as i32, c as i32);

                for &letter in WORD.as_bytes() {
                    if !(0..grid.len()).contains(&(location.0 as usize))
                        || !(0..row.len()).contains(&(location.1 as usize))
                        || grid[location.0 as usize][location.1 as usize] != letter
                    {
                        // out of bounds
                        continue 'start;
                    }

                    location = (location.0 + direction.0, location.1 + direction.1);
                }

                matches += 1;
            }
        }
    }

    matches
}

fn part_02(input: &str) -> usize {
    let grid: Vec<Vec<u8>> = input.lines().map(|e| e.to_string().into_bytes()).collect();

    const WORD: &str = "MAS";

    #[rustfmt::skip]
    const DIAGONALS: [(i32, i32); 4] = [
        (-1, -1), (-1, 1),
        ( 1, -1), ( 1, 1),
    ];

    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
    for direction in DIAGONALS {
        for (r, row) in grid.iter().enumerate() {
            'start: for (c, _) in row.iter().enumerate() {
                let mut location = (r as i32, c as i32);

                for &letter in WORD.as_bytes() {
                    if !(0..grid.len()).contains(&(location.0 as usize))
                        || !(0..row.len()).contains(&(location.1 as usize))
                        || grid[location.0 as usize][location.1 as usize] != letter
                    {
                        // out of bounds
                        continue 'start;
                    }

                    location = (location.0 + direction.0, location.1 + direction.1);
                }

                *counts
                    .entry((r as i32 + direction.0, c as i32 + direction.1))
                    .or_default() += 1i32;
            }
        }
    }

    // observe that if any two A's overlap from diagonals, then that counts for part2
    counts.into_values().filter(|v| *v == 2).count()
}

#[cfg(test)]
const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

#[test]
fn example_01() {
    assert_eq!(part_01(INPUT), 18);
}

#[test]
fn example_02() {
    assert_eq!(part_02(INPUT), 9);
}

fn main() {
    let s = std::fs::read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_01(&s));
    println!("part 2: {}", part_02(&s));
}
