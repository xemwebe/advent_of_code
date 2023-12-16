use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn solve(lines: io::Lines<io::BufReader<File>>, mut expansion_factor: usize) -> String {
    expansion_factor -= 1;
    let mut galaxies = Vec::new();
    let mut col_marks = Vec::new();
    let mut row_offsets = Vec::new();
    let mut row: usize = 0;
    let mut row_offset = 0;
    for l in lines {
        let line = l.unwrap();
        let line = line.trim();
        if row == 0 {
            col_marks = vec![true; line.len()];
        }
        let mut found_galaxy = false;
        for (col, c) in line.as_bytes().iter().enumerate() {
            if *c == b'#' {
                found_galaxy = true;
                galaxies.push((row, col));
                col_marks[col] = false;
            }
        }
        if !found_galaxy {
            row_offset += expansion_factor;
        }
        row_offsets.push(row_offset);
        row += 1;
    }
    let mut col_offsets = Vec::new();
    let mut col_offset: usize = 0;
    for i in 0..col_marks.len() {
        col_offsets.push(col_offset);
        if col_marks[i] {
            col_offset += expansion_factor;
        }
    }
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        let xi = galaxies[i].0 + row_offsets[galaxies[i].0];
        let yi = galaxies[i].1 + col_offsets[galaxies[i].1];
        for j in i + 1..galaxies.len() {
            let xj = galaxies[j].0 + row_offsets[galaxies[j].0];
            let yj = galaxies[j].1 + col_offsets[galaxies[j].1];
            sum += (xj as i64 - xi as i64).abs() + (yj as i64 - yi as i64).abs();
        }
    }
    format!("{sum}")
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    solve(lines, 2)
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    solve(lines, 1000000)
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2023_11_1() {
        let lines = read_lines("data/2023/11.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "9274989");
    }

    #[test]
    fn test_2023_11_2() {
        let lines = read_lines("data/2023/11.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "357134560737");
    }
}

