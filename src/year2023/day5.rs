use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug)]
struct Range {
    dest: u64,
    start: u64,
    end: u64,
}

fn convert_values(values: &[u64], map: &[Range]) -> Vec<u64> {
    let mut new_values = Vec::new();
    for v in values {
        let mut new_val = None;
        for r in map {
            if *v >= r.start && *v < r.end {
                new_val = Some(r.dest + (v - r.start));
            }
        }
        new_values.push(new_val.unwrap_or(*v));
    }
    new_values
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut values: Vec<u64> = Vec::new();
    let mut map = Vec::new();
    for line in lines {
        let l = line.unwrap().clone();
        if values.is_empty() {
            let vals: Vec<&str> = l.split(": ").collect();
            values = vals[1].split(' ').filter_map(|s| s.parse().ok()).collect();
            continue;
        }
        let range: Vec<u64> = l.split(' ').filter_map(|s| s.parse().ok()).collect();
        if range.is_empty() {
            if !map.is_empty() {
                values = convert_values(&values, &map);
                map = Vec::new();
            }
        } else {
            map.push(Range {
                dest: range[0],
                start: range[1],
                end: range[1] + range[2],
            });
        }
    }
    if !map.is_empty() {
        values = convert_values(&values, &map);
    }
    let solution = values.iter().min().unwrap();
    format!("{solution}")
}

#[derive(Clone, Debug)]
struct ValueRange {
    start: u64,
    end: u64,
}

fn convert_value_ranges(values: &[ValueRange], map: &[Range]) -> Vec<ValueRange> {
    let mut new_values = Vec::new();
    let mut orig_values = values.to_vec();
    for r in map {
        let dest_end = r.dest + (r.end - r.start);
        let mut extra = None;
        for v in &mut orig_values {
            let v_len = v.end - v.start;
            if v_len == 0 {
                continue;
            }
            if v.start < r.start && v.end > r.end {
                new_values.push(ValueRange {
                    start: r.dest,
                    end: dest_end,
                });
                extra = Some(ValueRange {
                    start: r.end,
                    end: v.end,
                });
                v.end = r.start;
                break;
            } else if v.start >= r.start && v.start < r.end {
                let start = r.dest + (v.start - r.start);
                let end = (start + v_len).min(dest_end);
                let len = end - start;
                new_values.push(ValueRange { start, end });
                if v_len > len {
                    v.start += len;
                } else {
                    v.start = v.end;
                }
            } else if v.end > r.start && v.end <= r.end {
                let len = v.end - r.start;
                new_values.push(ValueRange {
                    start: r.dest,
                    end: r.dest + len,
                });
                v.end -= len;
            }
        }
        if let Some(extra) = extra {
            orig_values.push(extra);
        }
    }
    for v in orig_values {
        if v.end > v.start {
            new_values.push(v);
        }
    }
    new_values
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut values = Vec::new();
    let mut map = Vec::new();
    for line in lines {
        let l = line.unwrap().clone();
        if values.is_empty() {
            let val_str: Vec<&str> = l.split(": ").collect();
            let vals: Vec<u64> = val_str[1]
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect();
            for i in 0..vals.len() / 2 {
                values.push(ValueRange {
                    start: vals[2 * i],
                    end: vals[2 * i] + vals[2 * i + 1],
                })
            }
            continue;
        }
        let range: Vec<u64> = l.split(' ').filter_map(|s| s.parse().ok()).collect();
        if range.is_empty() {
            if !map.is_empty() {
                values = convert_value_ranges(&values, &map);
                map = Vec::new();
            }
        } else {
            map.push(Range {
                dest: range[0],
                start: range[1],
                end: range[1] + range[2],
            });
        }
    }
    if !map.is_empty() {
        values = convert_value_ranges(&values, &map);
    }
    let solution = values.iter().map(|r| r.start).min().unwrap();
    format!("{solution}")
}
