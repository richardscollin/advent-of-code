use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
#[repr(u32)]
enum Rank {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Card(char);
impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // left is better
        // const MAP: &str =  "AKQJT98765432J";
        const MAP: &str =  "AKQT98765432J";
        MAP.find(self.0).unwrap().cmp(&MAP.find(other.0).unwrap())
    }
}
impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    rank: Rank,
    orig: Vec<Card>,
}

impl Hand {
    fn new(s: &str) -> Self {
        let mut counter = s.chars().fold(BTreeMap::new(), |mut acc, item| {
            *acc.entry(item).or_default() += 1;
            acc
        });

        // remove for part1
        let jokers = counter.remove(&'J');
        let mut wild_count = jokers.unwrap_or(0);

        let most_non_wild = *counter.values().max().unwrap_or(&0);

        let mut rank = match most_non_wild {
            5 => Rank::FiveOfAKind,
            4 => Rank::FourOfAKind,
            3 if counter.values().any(|&e| e == 2) => Rank::FullHouse,
            3 => Rank::ThreeOfAKind,
            2 if counter.values().filter(|e| **e == 2).count() >= 2 => Rank::TwoPair,
            2 => Rank::OnePair,
            _ => Rank::HighCard,
        };

        while wild_count > 0 {
            rank = match rank {
                Rank::FiveOfAKind => Rank::FiveOfAKind,
                Rank::FourOfAKind => Rank::FiveOfAKind,
                Rank::FullHouse => Rank::FourOfAKind,
                Rank::ThreeOfAKind => Rank::FourOfAKind,
                Rank::TwoPair => Rank::FullHouse,
                Rank::OnePair => Rank::ThreeOfAKind,
                Rank::HighCard => Rank::OnePair,
            };
            wild_count -= 1;
        }


        Self {
            rank,
            orig: s.chars().map(|c|Card(c)).collect(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank).then_with(||self.orig.cmp(&other.orig))
    }
}
impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


fn part1(s: &str) -> usize {
    let mut v = s.lines().map(|line| {
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid: usize = bid.parse().unwrap();
        let hand = Hand::new(hand);
        (hand, bid)
    }).collect::<Vec<_>>();
    v.sort();
    // println!("{:#?}", v);

    let n = v.len();

    v.iter().enumerate().map(|(i, (_, bid))| bid * (n - i)).sum()
}

fn part2(s: &str) -> usize {
    let mut v = s.lines().map(|line| {
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid: usize = bid.parse().unwrap();
        let hand = Hand::new(hand);
        (hand, bid)
    }).collect::<Vec<_>>();
    v.sort();
    // println!("{:#?}", v);

    let n = v.len();

    v.iter().enumerate().map(|(i, (_, bid))| bid * (n - i)).sum()
}

const SAMPLE: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";


#[test]
fn test_part1() {
    let sample = SAMPLE.trim();
    // assert_eq!(part1(sample), 6440);
    // assert_eq!(part1(include_str!("input.txt")), 251216224);
}

#[test]
fn test_part2()  {
    let sample = SAMPLE.trim();
    assert_eq!(part2(sample), 5905);
    // too high 251089155
    //251089155
    //251064011 too high
    // 250825971
    assert_eq!(part2(include_str!("input.txt")), 251216224);
}


