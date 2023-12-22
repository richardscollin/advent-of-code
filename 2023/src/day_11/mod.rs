#[allow(unused_imports)]
use std::collections::HashMap;

fn make_grid(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|line| line.chars().collect()).collect()
}

fn distance(
    (r1, c1): &(usize, usize),
    (r2, c2): &(usize, usize),
    rows: &[usize],
    cols: &[usize],
) -> usize {
    // assumes rows and cols are sorted
    let [&low_r, &high_r] = std::cmp::minmax(r1, r2);
    let [&low_c, &high_c] = std::cmp::minmax(c1, c2);

    let left_r = rows.partition_point(|&r| r <= low_r);
    let right_r = rows.partition_point(|&r| r <= high_r);

    let left_c = cols.partition_point(|&c| c <= low_c);
    let right_c = cols.partition_point(|&c| c <= high_c);

    (high_r - low_r) + (high_c - low_c) + (right_r - left_r) + (right_c - left_c)
}

fn part1(s: &str) -> usize {
    let grid = make_grid(s);

    let mut empty_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter_map(|(r, row)| row.iter().all(|ch| *ch == '.').then_some(r))
        .collect();

    let mut empty_columns: Vec<usize> = (0..grid[0].len())
        .filter_map(|c| (0..grid.len()).all(|r| grid[r][c] == '.').then_some(c))
        .collect();

    let galaxies: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, ch)| (*ch == '#').then_some((r, c)))
        })
        .collect();

    // println!("{empty_rows:?}");
    // println!("{empty_columns:?}");
    // println!("{galaxies:?}");

    // iterate over all pairs of pairs
    let mut total = 0;
    for (i, g1) in galaxies.iter().enumerate().take(galaxies.len() - 1) {
        for g2 in &galaxies[(i + 1)..] {
            // count empty columns and rows between g1, g2
            let dist = distance(g1, g2, &empty_rows, &empty_columns);
            println!("{g1:?} {g2:?} {dist:?}");
            total += dist;
        }
    }

    total
}

fn part2(s: &str) -> usize {
    0
}

const SAMPLE: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE.trim()), 374);
    // assert_eq!(part1(include_str!("input.txt")), 0);
}

#[test]
fn test_part2() {
    // assert_eq!(part2(SAMPLE.trim()), 0);
    // assert_eq!(part2(include_str!("input.txt")), 0);
}
