use crate::year2022::list::List;
use std::cmp::Ordering;
use std::{fs::File, io};

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

lrlex_mod!("year2022/list.l");
lrpar_mod!("year2022/list.y");

// I know, this is approach is far to complex for this little riddle,
// but I thought it would be fun to experiment with some advanced
// rust parser crate.. And I was right!
fn parse_list(lines: io::Lines<io::BufReader<File>>) -> Vec<List> {
    let lexerdef = list_l::lexerdef();
    lines
        .into_iter()
        .map(|s| s.unwrap().trim().to_owned())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let lexer = lexerdef.lexer(&s);
            let (res, errs) = list_y::parse(&lexer);
            for e in errs {
                println!("{}", e.pp(&lexer, &list_y::token_epp));
            }
            match res {
                Some(Ok(r)) => r,
                _ => List::Empty,
            }
        })
        .collect()
}

enum Order {
    Right,
    Unknown,
    Wrong,
}

fn is_ordered(a: &List, b: &List) -> Order {
    match a {
        List::Empty => match b {
            List::Empty => Order::Unknown,
            _ => Order::Right,
        },
        List::Num(a) => match b {
            List::Empty => Order::Wrong,
            List::Num(b) => {
                if *b > *a {
                    Order::Right
                } else if *b == *a {
                    Order::Unknown
                } else {
                    Order::Wrong
                }
            }
            List::Array(_) => is_ordered(&List::Array(vec![Box::new(List::Num(*a))]), b),
        },
        List::Array(va) => match b {
            List::Empty => Order::Wrong,
            List::Num(b) => is_ordered(a, &List::Array(vec![Box::new(List::Num(*b))])),
            List::Array(vb) => {
                let adim = va.len();
                for i in 0..adim {
                    if i >= vb.len() {
                        return Order::Wrong;
                    }
                    match is_ordered(&(*va[i]), &(*vb[i])) {
                        Order::Wrong => {
                            return Order::Wrong;
                        }
                        Order::Right => {
                            return Order::Right;
                        }
                        Order::Unknown => {}
                    };
                }
                if adim < vb.len() {
                    Order::Right
                } else {
                    Order::Unknown
                }
            }
        },
    }
}

fn count_orderd(lists: &Vec<List>) -> usize {
    let mut sum = 0;
    for i in 0..lists.len() / 2 {
        match is_ordered(&lists[i * 2], &lists[i * 2 + 1]) {
            Order::Right => {
                sum += i + 1;
            }
            _ => {}
        }
    }
    sum
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let lists = parse_list(lines);
    let sum = count_orderd(&lists);
    format!("{sum}")
}

fn ordering(a: &List, b: &List) -> Ordering {
    match is_ordered(a, b) {
        Order::Right => Ordering::Less,
        Order::Unknown => Ordering::Equal,
        Order::Wrong => Ordering::Greater,
    }
}

fn find_marker(m: u8, lists: &Vec<List>) -> usize {
    for i in 0..lists.len() {
        match &lists[i] {
            List::Array(va) => match &(*(va[0])) {
                List::Array(vaa) => match *(vaa[0]) {
                    List::Num(x) => {
                        if m == x {
                            return i + 1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
    0
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut lists = parse_list(lines);
    let marker2 = List::Array(vec![Box::new(List::Num(2))]);
    lists.push(List::Array(vec![Box::new(marker2)]));
    let marker6 = List::Array(vec![Box::new(List::Num(6))]);
    lists.push(List::Array(vec![Box::new(marker6)]));

    lists.sort_by(|a, b| ordering(a, b));

    let decoder_key = find_marker(2, &lists) * find_marker(6, &lists);
    format!("{decoder_key}")
}
