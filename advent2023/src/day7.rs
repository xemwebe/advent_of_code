use super::*;

#[derive(Debug)]
struct Hand {
    score: u64,
    bet: u32,
}

fn calc_frequencies(cards: &[u8; 5]) -> u64 {
    let mut frequencies = vec![0u8; 15];
    for c in cards {
        frequencies[*c as usize] += 1;
    }
    frequencies.sort();
    (frequencies[14]*10 +frequencies[13]) as u64
}

fn calc_frequencies_with_joker(cards: &[u8; 5]) -> u64 {
    let mut frequencies = vec![0i32; 15];
    for c in cards {
        frequencies[*c as usize] += 1;
    }
    let joker = frequencies[1];
    frequencies[1] = 0;
    frequencies.sort();
    let top = frequencies[14];
    let second = frequencies[13];
    let joker_top = (top+joker).min(5);
    let joker_second = (second+joker-(joker_top-top)).min(2);
    (joker_top*10 +joker_second) as u64
}

impl Hand {
    fn new(vals: &[&str]) -> Self {
        let mut cards: [u8; 5] = vals[0].as_bytes()[0..5].try_into().unwrap();
        for i in 0..5 {
            cards[i] = match cards[i] {
                b'2'..=b'9' => cards[i] - b'0',
                b'T' => 10,
                b'J' => 11,
                b'Q' => 12,
                b'K' => 13,
                b'A' => 14,
                _ => 0,
            }
        }
        let mut score = calc_frequencies(&cards);
        for c in cards {
            score = score*100 + c as u64;
        }
        let bet = vals[1].parse().unwrap();
        Self {
            score,
            bet,
        }
    }

    fn with_joker(vals: &[&str]) -> Self {
        let mut cards: [u8; 5] = vals[0].as_bytes()[0..5].try_into().unwrap();
        for i in 0..5 {
            cards[i] = match cards[i] {
                b'2'..=b'9' => cards[i] - b'0',
                b'T' => 10,
                b'J' => 1,
                b'Q' => 12,
                b'K' => 13,
                b'A' => 14,
                _ => 0,
            }
        }
        let mut score = calc_frequencies_with_joker(&cards);
        for c in cards {
            score = score*100 + c as u64;
        }
        let bet = vals[1].parse().unwrap();
        Self {
            score,
            bet,
        }
    }
}

pub fn riddle_7_1(lines: io::Lines<io::BufReader<File>>) {
    let mut hands = Vec::new();
    for line in lines {
        let l = line.unwrap().clone();
        let vals: Vec<&str> = l.split(' ').collect();
        hands.push(Hand::new(&vals));
    }
    hands.sort_by(|a,b| a.score.cmp(&b.score));
    for hand in &hands {
        println!("{hand:?}");
    }
    let mut solution = 0;
    for i in 0..hands.len() {
        solution += (i as u32+1)*hands[i].bet;
    }
    println!("The solution is: {}", solution);
}

pub fn riddle_7_2(lines: io::Lines<io::BufReader<File>>) {
    let mut hands = Vec::new();
    for line in lines {
        let l = line.unwrap().clone();
        let vals: Vec<&str> = l.split(' ').collect();
        hands.push(Hand::with_joker(&vals));
    }
    hands.sort_by(|a,b| a.score.cmp(&b.score));
    let mut solution = 0;
    for i in 0..hands.len() {
        solution += (i as u32+1)*hands[i].bet;
    }
    println!("The solution is: {}", solution);
}
