use super::*;

pub fn riddle_1_1(lines: io::Lines<io::BufReader<File>>) {
    let numbers: Vec<i32> = lines.into_iter()
    .filter_map(|s| s.ok())
    .map(|s| s.parse::<i32>().unwrap_or(0))
    .collect();

    let mut max = 0;
    let mut current = 0;
    for n in numbers {
        if n==0 {
            max = max.max(current);
            current = 0;
        } else {
            current += n;
        }
    }
    println!("The solution is: {}", max);
}

pub fn riddle_1_2(lines: io::Lines<io::BufReader<File>>) {
    let numbers: Vec<i32> = lines.into_iter()
    .filter_map(|s| s.ok())
    .map(|s| s.parse::<i32>().unwrap_or(0))
    .collect();

    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut current = 0;
    for n in numbers {
        if n==0 {
            if current > third {
                if current > first {
                    third = second;
                    second = first;
                    first = current;
                } else if current > second {
                    third = second;
                    second = current;
                } else {
                    third = current;
                }
            }
            current = 0;
        } else {
            current += n;
        }
    }
    println!("The solution is: {}", first+second+third);
}
