use std::collections::{
    HashMap,
    HashSet,
};

#[test]
fn test_part1() {
    assert_eq!(
        part1(
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
            .trim(),
        ),
        4361,
    );

    assert_eq!(part1(include_str!("input.txt").trim()), 514969);
    assert_eq!(part1_clean(include_str!("input.txt").trim()), 514969);
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
            .trim()
        ),
        467835
    );

    assert_eq!(part2(include_str!("input.txt").trim()), 78915902);
    assert_eq!(part2_clean(include_str!("input.txt").trim()), 78915902);
}

pub fn part1(s: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = vec![];

    for line in s.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
    }

    let mut has_adjacent_symbol = vec![vec![false; grid[0].len()]; grid.len()];

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if !ch.is_ascii_digit() && *ch != '.' {
                for (nr, nc) in neighbors(r, c, grid.len(), row.len()) {
                    has_adjacent_symbol[nr][nc] = true;
                }
            }
        }
    }

    let mut acc: u32 = 0;
    for (r, row) in grid.iter().enumerate() {
        let mut num = String::new();
        let mut has_adjacent = false;

        for (c, ch) in row.iter().enumerate() {
            if ch.is_ascii_digit() {
                num += &ch.to_string();
                if has_adjacent_symbol[r][c] {
                    has_adjacent = true
                }
            } else {
                // must be . or symbol, if it's a symbol then is_part is false
                if let Ok(part) = num.parse::<u32>() {
                    if has_adjacent {
                        acc += part;
                    }
                }

                num.clear();
                has_adjacent = false;
            }
        }

        // also at end of row
        if has_adjacent && !num.is_empty() {
            // must be . or symbol, if it's a symbol then is_part is false
            let part = num.parse::<u32>().unwrap();
            if has_adjacent {
                acc += part;
            }
        }
    }

    acc
}

pub fn part1_clean(s: &str) -> u32 {
    let grid = s.split('\n').collect::<Vec<&str>>();
    let height = grid.len();
    let width = grid[0].len();

    let mut has_adjacent_symbol = vec![vec![false; width]; height];
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if !ch.is_ascii_digit() && ch != '.' {
                for (nr, nc) in neighbors(r, c, height, width) {
                    has_adjacent_symbol[nr][nc] = true;
                }
            }
        }
    }

    let mut acc: u32 = 0;
    for (r, row) in grid.iter().enumerate() {
        let mut num = String::new();
        let mut run_has_adjacent = false;

        for (c, ch) in row.chars().enumerate() {
            if ch.is_ascii_digit() {
                num.push(ch);
                if has_adjacent_symbol[r][c] {
                    run_has_adjacent = true
                }
            } else {
                // must be . or symbol, if it's a symbol then is_part is false
                if let Ok(num) = num.parse::<u32>() {
                    if run_has_adjacent {
                        acc += num;
                    }
                }

                num.clear();
                run_has_adjacent = false;
            }
        }

        // at end of row
        if let Ok(num) = num.parse::<u32>() {
            if run_has_adjacent {
                acc += num;
            }
        }
    }

    acc
}

fn neighbors(
    r: usize,
    c: usize,
    height: usize,
    width: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .map(move |(dr, dc)| (r as isize + dr, c as isize + dc))
    .filter_map(move |(nr, nc)| {
        ((0..height as isize).contains(&nr) && (0..width as isize).contains(&nc))
            .then_some((nr as usize, nc as usize))
    })
}

pub fn part2(s: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = vec![];

    for line in s.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
    }

    let mut candidate_parts = vec![vec![None; grid[0].len()]; grid.len()];

    for (r, row) in grid.iter().enumerate() {
        let mut num = String::new();
        let mut start_c = None;

        for (c, ch) in row.iter().enumerate() {
            if ch.is_ascii_digit() {
                if num.is_empty() {
                    start_c = Some(c);
                }

                num += &ch.to_string();
            } else {
                // must be . or symbol, if it's a symbol then is_part is false
                if let Ok(part) = num.parse::<u32>() {
                    for i in start_c.unwrap()..c {
                        candidate_parts[r][i] = Some((r, start_c.unwrap(), part));
                    }
                }
                num.clear();
            }
        }

        // also at end of row
        if !num.is_empty() {
            // must be . or symbol, if it's a symbol then is_part is false
            let part = num.parse::<u32>().unwrap();
            for i in start_c.unwrap()..row.len() {
                candidate_parts[r][i] = Some((r, start_c.unwrap(), part));
            }
        }
    }

    let mut acc = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == '*' {
                let mut parts = HashSet::new();

                for (nr, nc) in neighbors(r, c, grid.len(), row.len()) {
                    if let Some(part_no) = candidate_parts[nr][nc] {
                        parts.insert(part_no);
                    }
                }

                if parts.len() == 2 {
                    acc += parts
                        .into_iter()
                        .map(|(_, _, part_no)| part_no)
                        .product::<u32>();
                }
            }
        }
    }
    acc
}

pub fn part2_clean(s: &str) -> u32 {
    let grid = s.split('\n').collect::<Vec<&str>>();
    let height = grid.len();
    let width = grid[0].len();

    let mut candidate_parts = vec![vec![None; width]; height];

    for (r, row) in grid.iter().enumerate() {
        let mut num = String::new();
        let mut start_c = None;

        for (c, ch) in row.chars().enumerate() {
            if ch.is_ascii_digit() {
                if num.is_empty() {
                    start_c = Some(c);
                }

                num += &ch.to_string();
            } else {
                // must be . or symbol, if it's a symbol then is_part is false
                if let Ok(part) = num.parse::<u32>() {
                    for i in start_c.unwrap()..c {
                        candidate_parts[r][i] = Some(((r, start_c.unwrap()), part));
                    }
                }
                num.clear();
            }
        }

        // also at end of row
        if !num.is_empty() {
            // must be . or symbol, if it's a symbol then is_part is false
            let part = num.parse::<u32>().unwrap();
            for i in start_c.unwrap()..row.len() {
                candidate_parts[r][i] = Some(((r, start_c.unwrap()), part));
            }
        }
    }

    grid.iter()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(c, ch)| {
                    if ch != '*' {
                        return None;
                    }

                    let parts: HashMap<(usize, usize), u32> =
                        neighbors(r, c, grid.len(), row.len())
                            .filter_map(|(nr, nc)| candidate_parts[nr][nc])
                            .collect();

                    (parts.len() == 2).then_some(parts.into_values().product::<u32>())
                })
                .sum::<u32>()
        })
        .sum()
}
