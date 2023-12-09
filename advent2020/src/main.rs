use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("I expect exactly one argument (the number of the riddle, eg. 1_2 for the first days second puzzle");
        return;
    }
    let riddle = &args[1];
    if let Ok(lines) = read_lines(format!("data/input_{}.txt", riddle)) {
        match riddle.as_str() {
            "1_1" => riddle_1_1(lines),
            "1_2" => riddle_1_2(lines),
            _ => println!("Invalid riddle"),
        }
    }
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn riddle_1_1(lines: io::Lines<io::BufReader<File>>) {
    let numbers: Vec<i32> = lines.into_iter()
    .map_while(Result::ok)
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();

    for (idx, i) in numbers.iter().enumerate() {
        for j in numbers[idx+1..].iter() {
            if i+j==2020 {
                println!("The solution is: {}", i*j);
            } 
        }
    }
}


fn riddle_1_2(lines: io::Lines<io::BufReader<File>>) {
    let numbers: Vec<i32> = lines.into_iter()
    .map_while(Result::ok)
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();

    for (idx, i) in numbers.iter().enumerate() {
        for (jdx, j) in numbers[idx+1..].iter().enumerate() {
            for k in numbers[idx+jdx+1..].iter() {
                if i+j+k==2020 {
                    println!("The solution is: {}", i*j*k);
                }
            }
        }
    }
}