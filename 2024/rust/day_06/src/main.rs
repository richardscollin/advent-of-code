use std::collections::HashSet;

const CARDINAL_DIRECTIONS: [(i32 /* row */, i32 /* col */); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part_01(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let (guard_row, guard_col) = take_guard(&mut grid).unwrap();
    simulate_path(guard_row, guard_col, &grid).len()
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.into()).collect()
}

fn take_guard(grid: &mut [Vec<u8>]) -> Option<(i32, i32)> {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'^' {
                grid[r][c] = b'.';
                return Some((r as i32, c as i32));
            }
        }
    }

    None
}

fn simulate_path(mut guard_row: i32, mut guard_col: i32, grid: &[Vec<u8>]) -> HashSet<(i32, i32)> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut turns = 0;
    while (0..height).contains(&guard_row) && (0..width).contains(&guard_col) {
        visited.insert((guard_row, guard_col));
        loop {
            let (dr, dc) = CARDINAL_DIRECTIONS[turns % CARDINAL_DIRECTIONS.len()];
            let nr = guard_row + dr;
            let nc = guard_col + dc;

            if (0..width).contains(&nc)
                && (0..height).contains(&nr)
                && grid[nr as usize][nc as usize] == b'#'
            {
                turns += 1;
            } else {
                guard_row = nr;
                guard_col = nc;
                break;
            }
        }
    }

    visited
}

fn part_02(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let (start_row, start_col) = take_guard(&mut grid).unwrap();
    let visited = simulate_path(start_row, start_col, &grid);
    let max_steps = grid.len() * grid[0].len();

    visited
        .into_iter()
        .filter(|loc| *loc != (start_row, start_col))
        .filter(|(row, col)| {
            grid[*row as usize][*col as usize] = b'#';
            let steps = simulate(start_row, start_col, &grid, max_steps as i32);
            grid[*row as usize][*col as usize] = b'.';

            steps == -1
        })
        .count()
}

fn simulate(mut guard_row: i32, mut guard_col: i32, grid: &[Vec<u8>], max_steps: i32) -> i32 {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut turns = 0;
    let mut steps: i32 = 0;
    while (0..height).contains(&guard_row) && (0..width).contains(&guard_col) {
        loop {
            let (dr, dc) = CARDINAL_DIRECTIONS[turns % CARDINAL_DIRECTIONS.len()];
            let nr = guard_row + dr;
            let nc = guard_col + dc;

            if (0..width).contains(&nc)
                && (0..height).contains(&nr)
                && grid[nr as usize][nc as usize] == b'#'
            {
                turns += 1;
            } else {
                guard_row = nr;
                guard_col = nc;
                break;
            }
        }

        if steps > max_steps {
            return -1;
        }

        steps += 1;
    }

    steps
}

#[allow(dead_code)]
fn display_grid(grid: &[Vec<u8>]) {
    for line in grid {
        println!("{}", std::str::from_utf8(line).unwrap());
    }
}

#[cfg(test)]
const INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

#[test]
fn example_01() {
    assert_eq!(part_01(INPUT), 41);
}

#[test]
fn example_02() {
    assert_eq!(part_02(INPUT), 6);
}

fn main() {
    let s = std::fs::read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_01(&s));
    println!("part 2: {}", part_02(&s));
}
