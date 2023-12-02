//! Day 2: Cube Conundrum
//! <https://adventofcode.com/2023/day/2>
use std::collections::HashMap;
#[test]
fn test_part1() {
    // Determine which games would have been possible if the bag
    // had been loaded with only 12 red cubes, 13 green cubes, and
    // 14 blue cubes. What is the sum of the IDs of those games?
    #[rustfmt::skip]
    assert_eq!(part1("
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ".trim()), 8);

    assert_eq!(part1(include_str!("input.txt")), 2600);
    assert_eq!(part1_cleanup(include_str!("input.txt")), 2600);
}

#[test]
fn test_part2() {
    // Determine the minimum set of cubes required in each game
    // and find the product of the colors, then sum these
    // game1 => 4 red 6 blue 2 green => 4*6*2 summed with all other games
    #[rustfmt::skip]
    assert_eq!(part2("
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ".trim()), 2286);

    assert_eq!(part2(include_str!("input.txt")), 86036);
    assert_eq!(part2_cleanup(include_str!("input.txt")), 86036);
}

fn part1(s: &str) -> u32 {
    let target: HashMap<String, u32> = HashMap::from([
        (String::from("red"), 12),
        (String::from("green"), 13),
        (String::from("blue"), 14),
    ]);

    let mut acc = 0;

    'game: for (game_id, game) in s.lines().enumerate() {
        let Some((_, drawings)) = game.split_once(": ") else {
            panic!("invalid input");
        };

        for drawing in drawings.split("; ") {
            for pair in drawing.split(", ") {
                let Some((occurances, color)) = pair.split_once(' ') else {
                    panic!("invalid input");
                };
                let occurances: u32 = occurances.parse().unwrap();

                if occurances > target[color] {
                    continue 'game;
                }
            }
        }
        acc += game_id as u32 + 1;
    }

    acc
}

fn part1_cleanup(s: &str) -> u32 {
    let target = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    s.lines()
        .enumerate()
        .filter_map(|(game_id, game)| {
            let (_, drawings) = game.split_once(": ")?;
            drawings
                .split("; ")
                .all(|drawing| {
                    drawing.split(", ").all(|pair| {
                        let (occurances, color) = pair.split_once(' ').unwrap();
                        occurances.parse::<u32>().unwrap() <= target[color]
                    })
                })
                .then_some(game_id as u32 + 1)
        })
        .sum()
}

fn part2(s: &str) -> u32 {
    let mut acc = 0;

    for game in s.lines() {
        let Some((_, drawings)) = game.split_once(": ") else {
            panic!("invalid input");
        };

        let mut count: HashMap<&str, u32> = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);

        for drawing in drawings.split("; ") {
            for pair in drawing.split(", ") {
                let Some((occurances, color)) = pair.split_once(' ') else {
                    panic!("invalid input");
                };
                let occurances: u32 = occurances.parse().unwrap();
                *count.get_mut(color).unwrap() = count[color].max(occurances);
            }
        }
        acc += count.values().product::<u32>();
    }

    acc
}

fn part2_cleanup(s: &str) -> u32 {
    s.lines()
        .filter_map(|game| {
            let mut count: HashMap<&str, u32> =
                HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
            let (_, drawings) = game.split_once(": ")?;

            for drawing in drawings.split("; ") {
                for pair in drawing.split(", ") {
                    let (occurances, color) = pair.split_once(' ')?;
                    let occurances: u32 = occurances.parse().ok()?;
                    *count.get_mut(color)? = count[color].max(occurances);
                }
            }
            Some(count.values().product::<u32>())
        })
        .sum()
}
