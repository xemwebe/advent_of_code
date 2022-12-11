use super::*;

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    a0: usize,
    a1: usize,
    a2: usize,
    divisible: usize,
    true_monkey: usize,
    false_monkey: usize,
    examined_items: usize,
}

// Static stetup is simpler than writing a parser...
fn init_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![93,98],
            a0: 0,
            a1: 17,
            a2: 0,
            divisible: 19,
            true_monkey: 5,
            false_monkey: 3,
            examined_items: 0,
        },
        Monkey {
            items: vec![95,72,98,82,86],
            a0: 5,
            a1: 1,
            a2: 0,
            divisible: 13,
            true_monkey: 7,
            false_monkey: 6,
            examined_items: 0,
        },
        Monkey {
            items: vec![85, 62, 82, 86, 70, 65, 83, 76],
            a0: 8,
            a1: 1,
            a2: 0,
            divisible: 5,
            true_monkey: 3,
            false_monkey: 0,
            examined_items: 0,
        },
        Monkey {
            items: vec![86, 70, 71, 56],
            a0: 1,
            a1: 1,
            a2: 0,
            divisible: 7,
            true_monkey: 4,
            false_monkey: 5,
            examined_items: 0,
        },
        Monkey {
            items: vec![77, 71, 86, 52, 81, 67],
            a0: 4,
            a1: 1,
            a2: 0,
            divisible: 17,
            true_monkey: 1,
            false_monkey: 6,
            examined_items: 0,
        },
        Monkey {
            items: vec![89, 87, 60, 78, 54, 77, 98],
            a0: 0,
            a1: 7,
            a2: 0,
            divisible: 2,
            true_monkey: 1,
            false_monkey: 4,
            examined_items: 0,
        },
        Monkey {
            items: vec![69, 65, 63],
            a0: 6,
            a1: 1,
            a2: 0,
            divisible: 3,
            true_monkey: 7,
            false_monkey: 2,
            examined_items: 0,
        },
        Monkey {
            items: vec![89],
            a0: 0,
            a1: 0,
            a2: 1,
            divisible: 11,
            true_monkey: 0,
            false_monkey: 2,
            examined_items: 0,
        },
    ]
}

fn process_monkeys(m: &mut Vec<Monkey>) -> usize {
    for _ in 0..20 {
        for i in 0..m.len() {
            let iftrue = m[i].true_monkey;
            let iffalse = m[i].false_monkey;
            for item in m[i].items.clone() {
                let op = ((m[i].a2*item + m[i].a1)*item + m[i].a0)/3;
                if op%m[i].divisible == 0 {
                    m[iftrue].items.push(op);
                } else {
                    m[iffalse].items.push(op);
                }
            }
            m[i].examined_items += m[i].items.len();
            m[i].items = Vec::new();
//            println!("monkeys: {m:#?}")
        }
    }
    let mut max = 0;
    let mut max2 = 0;
    for mm in m {
        if mm.examined_items > max {
            max2 = max;
            max = mm.examined_items
        } else if mm.examined_items > max2 {
            max2 = mm.examined_items
        }
    }
    max*max2
}


fn process_monkeys_rule2(m: &mut Vec<Monkey>) -> usize {
    let mut total_divisibility = 1;
    for i in 0..m.len() {
        total_divisibility *= m[i].divisible;
    }
    for _ in 0..10000 {
        for i in 0..m.len() {
            let iftrue = m[i].true_monkey;
            let iffalse = m[i].false_monkey;
            for item in m[i].items.clone() {
                let op = ((m[i].a2*item + m[i].a1)*item + m[i].a0)%total_divisibility;
                if op%m[i].divisible == 0 {
                    m[iftrue].items.push(op);
                } else {
                    m[iffalse].items.push(op);
                }
            }
            m[i].examined_items += m[i].items.len();
            m[i].items = Vec::new();
//            println!("monkeys: {m:#?}")
        }
    }
    let mut max = 0;
    let mut max2 = 0;
    for mm in m {
        if mm.examined_items > max {
            max2 = max;
            max = mm.examined_items
        } else if mm.examined_items > max2 {
            max2 = mm.examined_items
        }
    }
    max*max2
}

pub fn riddle_11_1(_lines: io::Lines<io::BufReader<File>>) {
    let mut monkeys = init_monkeys();
    let score = process_monkeys(&mut monkeys);
    println!("{:?}", score);
}

pub fn riddle_11_2(_lines: io::Lines<io::BufReader<File>>) {
    let mut monkeys = init_monkeys();
    let score = process_monkeys_rule2(&mut monkeys);
    println!("{:?}", score);
}
