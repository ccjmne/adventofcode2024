use crate::{
    register,
    registry::{register, AnySolution, Output, Solution},
};
use ctor::ctor;
use std::iter::{once, zip};

struct Y2024D2;
register!(2, Y2024D2);
impl Solution for Y2024D2 {
    type Input = Vec<Vec<i32>>;

    fn parse(source: String) -> Self::Input {
        source
            .lines()
            .into_iter()
            .map(|report| {
                report
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part_i(input: &Self::Input) -> Box<Output> {
        Box::new(
            input
                .clone()
                .into_iter()
                .map(|report| {
                    zip(report.clone(), report[1..].to_vec())
                        .map(|(l, r)| ((r - l).abs(), (r - l).signum()))
                        .fold(Ok(i32::MAX), |trend, (delta, dir)| {
                            trend.and_then(|t| {
                                if (t == i32::MAX || t == dir) && (0 < delta && delta <= 3) {
                                    Ok(dir)
                                } else {
                                    Err(())
                                }
                            })
                        })
                })
                .filter_map(Result::ok)
                .count(),
        )
    }

    fn part_ii(input: &Self::Input) -> Box<Output> {
        Box::new(
            input
                .clone()
                .into_iter()
                .map(|report| {
                    let default = report.clone();
                    (0..report.len())
                        .map(|i| {
                            let clone = report.clone();
                            [&clone[..i], &clone[i + 1..]].concat()
                        })
                        .chain(once(default))
                        .map(|report| {
                            zip(report.clone(), report[1..].to_vec())
                                .map(|(l, r)| ((r - l).abs(), (r - l).signum()))
                                .fold(Ok(i32::MAX), |trend, (delta, dir)| {
                                    trend.and_then(|t| {
                                        if (t == i32::MAX || t == dir) && (0 < delta && delta <= 3)
                                        {
                                            Ok(dir)
                                        } else {
                                            Err(())
                                        }
                                    })
                                })
                        })
                        .any(|r| r.is_ok())
                })
                .filter(|&ok| ok)
                .count(),
        )
    }
}
