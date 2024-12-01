use crate::Day;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref PARSER: Regex = Regex::new(r"^(?P<a>\d+)\s*(?P<b>\d+)$").unwrap();
}

pub struct Day1;

impl Day for Day1 {
    type Input = (Vec<u32>, Vec<u32>);
    fn parse(input: String) -> Self::Input {
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

    fn part_i(data: &Self::Input) -> u32 {
        let (mut l, mut r) = data.clone();
        l.sort();
        r.sort();

        l.into_iter()
            .zip(r)
            .map(|(a, b)| (a).abs_diff(b))
            .sum::<u32>()
    }

    fn part_ii(data: &Self::Input) -> u32 {
        let (l, r) = data;
        fn bin(v: &Vec<u32>) -> HashMap<u32, u32> {
            v.into_iter().fold(HashMap::new(), |mut acc, x| {
                acc.entry(*x).and_modify(|c| *c += 1).or_insert(1 as u32);
                acc
            })
        }

        let r = bin(r);
        bin(l)
            .into_iter()
            .fold(0, |sum, (k, v)| sum + k * v * r.get(&k).unwrap_or(&0))
    }
}
