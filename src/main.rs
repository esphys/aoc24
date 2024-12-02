mod days;

use clap::{Args, Parser};
use std::fs::read_to_string;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    day: String,

    #[command(flatten)]
    file: File,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
struct File {
    #[arg(short='b', long="bigboy")]
    bigboy: bool,

    #[arg(short='e', long="example")]
    example: bool,
}

fn main() {
    let registry = days::get_registry();
    let args = Cli::parse();
    
    let day_i = args.day;

    let mut file = format!("src/inputs/{}.txt", day_i);
    if args.file.bigboy {
        file = format!("src/inputs/{}-bb.txt", day_i);
    } else if args.file.example {
        file = format!("src/inputs/{}-e.txt", day_i);
    }

    let binding = read_to_string(file).unwrap();
    let file = binding.lines();

    if let Some(&(_, func)) = registry.iter().find(|&&(day, _)| day == day_i) {
        func(file);
    } else {
        eprintln!("No solutions found for day: {}", day_i);
    }
}
