use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PARSER: Regex = Regex::new(r"^(?P<a>\d+)\s*(?P<b>\d+)$").unwrap();
}

fn parse(input: &String) -> Vec<(u32, u32)> {
    input
        .lines()
        .filter_map(|line| {
            PARSER.captures(line).and_then(|caps| {
                let a = caps.name("a")?.as_str().parse::<u32>().ok()?;
                let b = caps.name("b")?.as_str().parse::<u32>().ok()?;
                Some((a, b))
            })
        })
        .collect()
}

pub fn part1(input: &String) -> u32 {
    let (mut l, mut r): (Vec<_>, Vec<_>) = parse(input).into_iter().unzip();
    l.sort();
    r.sort();

    l.into_iter()
        .zip(r)
        .map(|(a, b)| (a).abs_diff(b))
        .sum::<u32>()
}

pub fn part2(input: &String) -> u32 {
    return 0;
}
