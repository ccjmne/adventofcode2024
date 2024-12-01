use std::{fmt::Debug, fs, path::Path};

use y2024::d1::Day1;

mod y2024 {
    pub mod d1;
}

pub trait Day {
    type Input;
    fn parse(input: String) -> Self::Input;
    fn part_i(data: &Self::Input) -> impl Debug;
    fn part_ii(data: &Self::Input) -> impl Debug;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: adventofcode <day>");
        std::process::exit(1);
    }

    let input = read_input(
        2024,
        args[1].parse().expect("Failed to parse number"),
        false,
    );

    match args[1].as_str() {
        "1" => run_day::<Day1>(input),
        _ => {
            eprintln!("Invalid day: {}", args[1]);
            std::process::exit(1);
        }
    }

    fn run_day<T: Day>(input: String) {
        let data = T::parse(input);
        println!("Part 1: {:?}", T::part_i(&data));
        println!("Part 2: {:?}", T::part_ii(&data));
    }
}

pub fn read_input(year: u16, day: u8, personalised: bool) -> String {
    let file_path = Path::new(file!()).parent().unwrap().join(format!(
        "y{year}/d{day}_{}.txt",
        if personalised { "real" } else { "test" }
    ));
    fs::read_to_string(file_path)
        .expect(&format!("Failed to read input for year {year}, day {day}"))
}
