use super::*;

fn read_values(lines: io::Lines<io::BufReader<File>>) -> Vec<(i32,i32)> {
    let mut idx = 0;
    lines
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|s| {
            let value = (s.parse::<i32>().unwrap(), idx);
            idx += 1;
            value
        })
        .collect()
}

fn scrampble(vals: &mut Vec<(i32, i32)>) {
    let len = vals.len() as i32;
    for i in 0..len {
        let mut old_idx = 0;
        for j in 0..vals.len() {
            if vals[j].1 == i {
                old_idx = j;
                break;
            }
        }
        let v = vals[old_idx];
        let mut new_idx = (old_idx as i32) + v.0;
        while new_idx<0 {
            new_idx += len-1;
        }
        while new_idx>=len {
            new_idx -= len-1;
        }
        let new_idx = new_idx as usize;
        if new_idx < old_idx {
            for k in (new_idx..old_idx).rev() {
                vals[k+1] = vals[k]
            }
        } else if new_idx > old_idx {
            for k in old_idx..new_idx {
                vals[k] = vals[k+1]
            }
        }
        vals[new_idx] = v;
//        println!("vals: {vals:?}");
    }
}

fn get_coordinate(vals: &Vec<(i32, i32)>) -> i32 {
    let mut zero_idx = 0;
    for i in 0..vals.len() {
        if vals[i].0 == 0 {
            zero_idx = i;
        }
    }
    let first = (zero_idx + 1000) % vals.len();
    let second = (first + 1000) % vals.len();
    let third = (second + 1000) % vals.len();
    vals[first].0+vals[second].0+vals[third].0
}

pub fn riddle_20_1(lines: io::Lines<io::BufReader<File>>) {
    let mut values = read_values(lines);
    scrampble(&mut values);
    println!("The solution is: {:?}", get_coordinate(&values));
}
