use super::*;

fn read_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
    let number_str = lines.into_iter().next().unwrap().unwrap();
    number_str.split(",").into_iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect()
}

pub fn riddle_7_1(lines: io::Lines<io::BufReader<File>>) {
    let numbers = read_numbers(lines);
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    let mut min_fuel = i32::MAX;
    let mut min_x = -1;
    for x in min..=max {
        let fuel = numbers.iter().map(|h| (h-x).abs() ).sum();
        if fuel < min_fuel {
            min_x = x;
            min_fuel = fuel;
        }
    }
    println!("x: {}, fuel: {}", min_x, min_fuel);
}


pub fn riddle_7_2(lines: io::Lines<io::BufReader<File>>) {
    let numbers = read_numbers(lines);
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    let mut min_fuel = i32::MAX;
    let mut min_x = -1;
    for x in min..=max {
        let fuel = numbers.iter().map(|h| {
            let n = (h-x).abs();
            n*(n+1)/2
        }).sum();
        if fuel < min_fuel {
            min_x = x;
            min_fuel = fuel;
        }
    }
    println!("x: {}, fuel: {}", min_x, min_fuel);
}
