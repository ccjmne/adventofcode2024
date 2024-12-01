mod y2024 {
    pub mod d1;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: adventofcode <day>");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "1" => y2024::d1::run(),
        _ => {
            eprintln!("Invalid day: {}", args[1]);
            std::process::exit(1);
        }
    }
}
