use crate::{
    register,
    registry::{register, AnySolution, Output, Solution},
};
use ctor::ctor;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

static PARSER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?P<a>\d+)\s*(?P<b>\d+)$").unwrap());

register!(1, Y2024D1);
struct Y2024D1;
impl Solution for Y2024D1 {
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

    fn part_i(data: &Self::Input) -> Box<Output> {
        let (mut l, mut r) = data.clone();
        l.sort();
        r.sort();

        Box::new(
            l.into_iter()
                .zip(r)
                .map(|(a, b)| (a).abs_diff(b))
                .sum::<u32>(),
        )
    }

    fn part_ii(data: &Self::Input) -> Box<Output> {
        let (l, r) = data;
        fn bin(v: &Vec<u32>) -> HashMap<u32, u32> {
            v.into_iter().fold(HashMap::new(), |mut acc, x| {
                acc.entry(*x).and_modify(|c| *c += 1).or_insert(1 as u32);
                acc
            })
        }

        let r = bin(r);
        Box::new(
            bin(l)
                .into_iter()
                .fold(0, |sum, (k, v)| sum + k * v * r.get(&k).unwrap_or(&0)),
        )
    }
}
