use super::*;
use std::cmp::{min,max};

type Point = (i32,i32);
type Pair = (Point, Point);


fn get_point(s: &str) -> Point {
    let mut points = s.split(",");
    (points.next().unwrap().parse::<i32>().unwrap(), points.next().unwrap().parse::<i32>().unwrap())
}

fn read_data(lines: io::Lines<io::BufReader<File>>) -> Vec<Pair> {
    let vent_lines: Vec<((i32,i32),(i32,i32))> = lines.into_iter()
    .filter_map(|x| x.ok())
    .filter(|x| !x.is_empty() )
    .map(|s| {
        let mut p_list = s.split(" -> ");
        (get_point(&p_list.next().unwrap()), get_point(&p_list.next().unwrap()))
    }).collect();

     vent_lines
}

fn calc_max_xy(vents: &Vec<Pair>) -> (usize, usize) {
    let mut max_x=0;
    let mut max_y=0;
    vents.iter().for_each(|x| {
        max_x = max(max_x, max(x.0.0, x.1.0));
        max_y = max(max_y, max(x.0.1, x.1.1));
    });
    ((max_x+1) as usize, (max_y+1) as usize)
}

fn mark_grid(grid: &mut Vec<i32>, pair: &Pair, max_x: usize) {
    if pair.0.0 == pair.1.0 {
        let start = min(pair.0.1, pair.1.1);
        let end = max(pair.0.1, pair.1.1);
        for y in start..=end {
            grid[(y as usize)*max_x + pair.0.0 as usize] += 1;
        }
    } else if pair.0.1 == pair.1.1 {
        let start = min(pair.0.0, pair.1.0);
        let end = max(pair.0.0, pair.1.0);
        for x in start..=end {
            grid[(pair.0.1 as usize)*max_x + x as usize] += 1;
        }
    } else {
        let mut x = pair.0.0;
        let end_x = pair.1.0;
        let mut y = pair.0.1;
        let end_y = pair.1.1;
        let step_x = if x<end_x { 1 } else { -1 };
        let step_y = if y<end_y { 1 } else { -1 };
        loop {
            grid[(y as usize)*max_x + x as usize] += 1;
            if x==end_x || y==end_y {
                break;
            }
            x += step_x;
            y += step_y;
        }

    }
}

fn print_grid(grid: &Vec<i32>, max_x: usize) {
    let max_y = grid.len()/max_x;
    for y in 0..max_y {
        for x in 0..max_x {
            match grid[(y as usize)*max_x + x as usize] {
                0 => print!("."),
                x => print!("{}", x),
            }
        }
        println!();
    }
}

pub fn riddle_5_1(lines: io::Lines<io::BufReader<File>>) {
    let vent_lines = read_data(lines);
    let (max_x, max_y) = calc_max_xy(&vent_lines);
    let mut grid = vec![0; max_x*max_y];
    vent_lines.iter().filter(|x| x.0.0==x.1.0 || x.0.1==x.1.1 ).for_each(|x| mark_grid(&mut grid, x, max_x));

    let mut sum = 0;
    grid.iter().for_each(|x| if *x>=2 { sum += 1 });
    println!("Found {} overlaps", sum);
}


pub fn riddle_5_2(lines: io::Lines<io::BufReader<File>>) {
    let vent_lines = read_data(lines);
    let (max_x, max_y) = calc_max_xy(&vent_lines);
    let mut grid = vec![0; max_x*max_y];
    vent_lines.iter().for_each(|x| mark_grid(&mut grid, x, max_x));

    let mut sum = 0;
    grid.iter().for_each(|x| if *x>=2 { sum += 1 });
    print_grid(&grid, max_x);
    println!("Found {} overlaps", sum); 
}