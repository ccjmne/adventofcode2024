use registry::get;
use std::{fs, path::Path};

mod registry;
mod y2024 {
    pub mod d1;
    pub mod d2;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: adventofcode <day>");
        std::process::exit(1);
    }
    let day: u8 = args[1].parse().expect("Failed to parse number");
    let input = read_input(2024, day, false);
    let (i, ii) = (get(day).unwrap().solve)(input);
    print!("Part I: {:?}\nPart II: {:?}", i, ii);
}

pub fn read_input(year: u16, day: u8, real: bool) -> String {
    let file_path = Path::new(file!()).parent().unwrap().join(format!(
        "y{year}/d{day}_{}.txt",
        if real { "real" } else { "test" }
    ));
    fs::read_to_string(file_path)
        .expect(&format!("Failed to read input for year {year}, day {day}"))
}
