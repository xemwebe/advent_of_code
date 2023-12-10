use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}
use std::collections::VecDeque;

fn read_algo_and_image(lines: io::Lines<io::BufReader<File>>) -> (String, VecDeque<String>) {
    let mut line_iter = lines.into_iter().filter_map(|x| x.ok());
    let algo_string = line_iter.next().unwrap();
    let image: VecDeque<String> = line_iter.skip(1).collect();

    (algo_string, image)
}

fn extend_image(image: &mut VecDeque<String>, outer_space: char) {
    for row in image.iter_mut() {
        *row = format!("{}{}{}", outer_space, *row, outer_space);
    }
    let empty_line: String = (0..image[0].len()).map(|_| outer_space).collect();
    image.push_front(empty_line.clone());
    image.push_back(empty_line);
}

fn enhance_image(image: &VecDeque<String>, algo: &str) -> VecDeque<String> {
    let mut new_image = VecDeque::new();
    let outer_space = if &image[0][0..=0] == "." {
        &algo[0..=0]
    } else {
        &algo[511..=511]
    };
    let n = image[0].len();
    let outer_row: String = (0..n).into_iter().map(|_| outer_space).collect();
    new_image.push_back(outer_row.clone());
    for i in 1..image.len() - 1 {
        let mut row = outer_space.to_string();
        for j in 1..n - 1 {
            let mut num = 0;
            for k in 0..3 {
                for l in 0..3 {
                    num <<= 1;
                    num += if &image[i + k - 1][j + l - 1..j + l] == "#" {
                        1
                    } else {
                        0
                    }
                }
            }
            row = format!("{}{}", row, &algo[num..=num]);
        }
        row = format!("{}{}", row, outer_space);
        new_image.push_back(row);
    }
    new_image.push_back(outer_row);
    new_image
}

pub fn count_lit_pixels(image: &VecDeque<String>) -> usize {
    image
        .iter()
        .map(|x| x.chars())
        .flatten()
        .filter(|x| *x == '#')
        .count()
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let (algo, mut image) = read_algo_and_image(lines);
    extend_image(&mut image, '.');
    extend_image(&mut image, '.');
    extend_image(&mut image, '.');
    image = enhance_image(&mut image, &algo);
    image = enhance_image(&mut image, &algo);
    format!("{}", count_lit_pixels(&image))
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let (algo, mut image) = read_algo_and_image(lines);
    let mut outer = '.';
    extend_image(&mut image, outer);
    extend_image(&mut image, outer);
    for _ in 0..50 {
        extend_image(&mut image, outer);
        image = enhance_image(&mut image, &algo);
        outer = image[0][0..=0].chars().next().unwrap();
    }
    format!("{}", count_lit_pixels(&image))
}
