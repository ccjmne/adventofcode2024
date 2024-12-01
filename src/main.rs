use std::{fs, path::Path};

mod y2024 {
    pub mod d1;
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

    println!(
        "{}",
        match args[1].as_str() {
            "1" => y2024::d1::part1(input),
            _ => {
                eprintln!("Invalid day: {}", args[1]);
                std::process::exit(1);
            }
        }
    )
}

pub fn read_input(year: u16, day: u8, personalised: bool) -> String {
    let file_path = Path::new(file!()).parent().unwrap().join(format!(
        "y{year}/d{day}_{}.txt",
        if personalised { "real" } else { "test" }
    ));
    fs::read_to_string(file_path)
        .expect(&format!("Failed to read input for year {year}, day {day}"))
}
