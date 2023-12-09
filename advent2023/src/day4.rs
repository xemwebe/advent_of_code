use super::*;

fn get_nums(num_str: &str) -> Vec<i32> {
    let nums: Vec<i32> = num_str.split(' ').filter_map(|s| s.parse().ok() ).collect();
    nums
}

pub fn riddle_4_1(lines: io::Lines<io::BufReader<File>>) {
    let mut sum = 0;
    for line in lines {
        let l = line.unwrap().clone();
        let nums: Vec<&str> = l.split(": ").collect();
        let num_halfs: Vec<&str> = nums[1].split(" | ").collect();
        let winning_nums = get_nums(num_halfs[0]);
        let got_nums = get_nums(num_halfs[1]);
        let mut score = 0;
        for got in &got_nums {
            for win in &winning_nums {
                if got==win {
                    if score==0 {
                        score = 1;
                    } else {
                        score *= 2;
                    }
                }
            }
        }
        sum += score;
    }
    println!("The solution is: {}", sum);
}

struct Card {
    winning: Vec<i32>,
    got: Vec<i32>,
    count: usize,
}

pub fn riddle_4_2(lines: io::Lines<io::BufReader<File>>) {
    let mut sum = 0;
    let mut cards = Vec::new();
    for line in lines {
        let l = line.unwrap().clone();
        let nums: Vec<&str> = l.split(": ").collect();
        let num_halfs: Vec<&str> = nums[1].split(" | ").collect();
        let winning = get_nums(num_halfs[0]);
        let got = get_nums(num_halfs[1]);
        cards.push(Card{
            winning,
            got,
            count: 1
        });
    }
    for i in 0..cards.len() {
        let mut score = 0;
        for got in &cards[i].got {
            for win in &cards[i].winning {
                if got==win {
                    score += 1;
                }
            }
        }
        sum += cards[i].count;
        let start = i+1;
        let end = cards.len().min(start+score);
        for j in start..end {
            cards[j].count += cards[i].count;
        }
    }
    println!("The solution is: {}", sum);
}
