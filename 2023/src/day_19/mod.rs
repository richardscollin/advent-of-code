use std::{
    collections::HashMap,
    convert::Infallible,
    str::FromStr,
};

#[allow(unused_imports)]
use indexmap::IndexMap;

#[derive(Debug)]
enum SendTo<'a> {
    Accept,
    Reject,
    Forward(&'a str),
}
impl<'a> TryFrom<&'a str> for SendTo<'a> {
    type Error = Infallible;
    fn try_from(s: &'a str) -> Result<SendTo<'a>, Self::Error> {
        Ok(match s {
            "A" => SendTo::Accept,
            "R" => SendTo::Reject,
            other => SendTo::Forward(other),
        })
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
impl FromStr for Part {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .split(|c: char| c.is_alphabetic() || matches!(c, '=' | ',' | '{' | '}'))
            .filter_map(|num| num.parse::<u32>().ok())
            .collect::<Vec<_>>()
            .as_slice()
        {
            &[x, m, a, s] => Ok(Part { x, m, a, s }),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    name: char,
    operator: char,
    value: u32,
    next: SendTo<'a>,
}
impl<'a> TryFrom<&'a str> for Rule<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (begin, end) = s.split_once(":").unwrap();
        let mut it = begin.chars();
        Ok(Self {
            name: it.next().ok_or(())?,
            operator: it.next().ok_or(())?,
            value: it.collect::<String>().parse::<u32>().map_err(|_| ())?,
            next: SendTo::try_from(end).unwrap(),
        })
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    fallthrough: SendTo<'a>,
}

impl<'a> TryFrom<&'a str> for Workflow<'a> {
    type Error = ();
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (workflow_name, remaining) = s.split_once("{").ok_or(())?;
        let mut parts = remaining
            .strip_suffix("}")
            .ok_or(())?
            .split(",")
            .collect::<Vec<&str>>();
        let last = parts.pop().ok_or(())?;
        let fallthrough = SendTo::try_from(last).unwrap();

        Ok(Self {
            name: workflow_name,
            rules: parts
                .into_iter()
                .filter_map(|e| Rule::try_from(e).ok())
                .collect::<Vec<_>>(),
            fallthrough,
        })
    }
}

fn apply_workflows(workflows: &IndexMap<&str, Workflow>, part: &Part) -> bool {
    let mut current = "in";

    loop {
        println!("{current:?}");
        let workflow = &workflows[current];
        for rule in &workflow.rules {
            let op = match rule.operator {
                '<' => u32::lt,
                '>' => u32::gt,
                _ => panic!(),
            };

            let next = match rule.name {
                'x' if op(&part.x, &rule.value) => &rule.next,
                'm' if op(&part.m, &rule.value) => &rule.next,
                'a' if op(&part.a, &rule.value) => &rule.next,
                's' if op(&part.s, &rule.value) => &rule.next,
                _ => &workflow.fallthrough,
            };

            match next {
                SendTo::Accept => return true,
                SendTo::Reject => return false,
                SendTo::Forward(forward_to) => {
                    current = forward_to;
                }
            }
        }
    }
}

fn part1(s: &str) -> u32 {
    let (workflows, parts) = s.split_once("\n\n").unwrap();

    let workflows: IndexMap<&str, Workflow<'_>> = workflows
        .lines()
        .map(|line| {
            let workflow: Workflow<'_> = line.try_into().unwrap();
            (workflow.name, workflow)
        })
        .collect();

    let parts = parts
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Part>, ()>>()
        .unwrap();

    for workflow in &workflows {
        println!("{workflow:#?}");
    }

    let mut accepted: Vec<Part> = vec![];

    for part in parts {
        if apply_workflows(&workflows, &part) {
            println!("{part:?}");
            accepted.push(part);
        }
    }

    accepted.iter().map(|e| e.x + e.m + e.a + e.s).sum()
}

fn part2(s: &str) -> usize {
    0
}

const SAMPLE: &str = "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

#[test]
fn test_part1() {
    assert_eq!(part1(SAMPLE.trim()), 19114);
    // assert_eq!(part1(include_str!("input.txt")), 0);
}

#[test]
fn test_part2() {
    // assert_eq!(part2(SAMPLE.trim()), 0);
    // assert_eq!(part2(include_str!("input.txt")), 0);
}
