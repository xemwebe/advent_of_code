#![feature(iter_advance_by)]

use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    year: u32,
    #[arg(short, long)]
    day: u32,
    #[arg(short, long)]
    part: u32,
    #[arg(short, long)]
    file: Option<PathBuf>,
}

mod year2015;
mod year2020;
mod year2023;

fn main() {
    let args = Args::parse();
    let lines = if let Some(file) = args.file.as_deref() {
        read_lines(file)
    } else {
        let file = format!("data/{}/{}.txt", args.year, args.day);
        read_lines(file)
    };
    if let Ok(lines) = lines {
        let solution = match args.year {
            2015 => year2015::execute(args.day, args.part, lines),
            2020 => year2020::execute(args.day, args.part, lines),
            2023 => year2023::execute(args.day, args.part, lines),
            _ => format!("Error: solution for year {} not implemented", args.year),
        };
        println!("The solution is: {solution}");
    } else {
        eprintln!("Couldn't read input file");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
