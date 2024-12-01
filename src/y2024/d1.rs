use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PARSER: Regex = Regex::new(r"^(?P<a>\d+)\s*(?P<b>\d+)$").unwrap();
}

fn parse(input: &String) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .filter_map(|line| {
            PARSER.captures(line).and_then(|caps| {
                let a = caps.name("a")?.as_str().parse::<u32>().ok()?;
                let b = caps.name("b")?.as_str().parse::<u32>().ok()?;
                Some((a, b))
            })
        })
        .unzip()
}

pub fn part1(input: &String) -> u32 {
    let (mut l, mut r) = parse(input);
    l.sort();
    r.sort();

    l.into_iter()
        .zip(r)
        .map(|(a, b)| (a).abs_diff(b))
        .sum::<u32>()
}

pub fn part2(input: &String) -> u32 {
    let (l, r) = parse(input);
    fn bin(v: Vec<u32>) -> HashMap<u32, u32> {
        v.into_iter().fold(HashMap::new(), |mut acc, x| {
            acc.entry(x).and_modify(|c| *c += 1).or_insert(1 as u32);
            acc
        })
    }

    let r = bin(r);
    bin(l)
        .into_iter()
        .fold(0, |sum, (k, v)| sum + k * v * r.get(&k).unwrap_or(&0))
}
