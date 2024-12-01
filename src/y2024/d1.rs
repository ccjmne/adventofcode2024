use regex::Regex;

pub fn part1(input: &String) -> u32 {
    let parser = Regex::new(r"^(?P<a>\d+)\s*(?P<b>\d+)$").expect("Invalid regex");
    let (mut l, mut r): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            parser.captures(line).and_then(|caps| {
                let a = caps.name("a")?.as_str().parse::<u32>().ok()?;
                let b = caps.name("b")?.as_str().parse::<u32>().ok()?;
                Some((a, b))
            })
        })
        .unzip();
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
