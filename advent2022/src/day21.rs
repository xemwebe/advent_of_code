use super::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug,Clone)]
enum Operation {
    Number(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn read_operations(lines: io::Lines<io::BufReader<File>>) -> HashMap<String, Operation> {
    let re_num = Regex::new("([a-z]+): ([0-9]+)").unwrap();
    let re_op = Regex::new("([a-z]+): ([a-z]+) ([-+*/]) ([a-z]+)").unwrap();
    let mut ops = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        for cap in re_num.captures_iter(&line) {
            ops.insert(cap[1].to_owned(), Operation::Number(cap[2].parse::<i64>().unwrap()));
        }
        for cap in re_op.captures_iter(&line) {
            let op = match &cap[3] {
                "+" => Operation::Add(cap[2].to_owned(), cap[4].to_owned()),
                "-" => Operation::Sub(cap[2].to_owned(), cap[4].to_owned()),
                "*" => Operation::Mul(cap[2].to_owned(), cap[4].to_owned()),
                _ => Operation::Div(cap[2].to_owned(), cap[4].to_owned()),
            };
            ops.insert(cap[1].to_owned(), op);
        }
    }
    ops
}

fn calc(current: String, ops: &mut HashMap<String, Operation>) -> i64 {
    let value = match ops[&current].clone() {
        Operation::Number(x) => x,
        Operation::Add(a, b) => calc(a, ops) + calc(b, ops),
        Operation::Sub(a, b) => calc(a, ops) - calc(b, ops),
        Operation::Mul(a, b) => calc(a, ops) * calc(b, ops),
        Operation::Div(a, b) => calc(a, ops) / calc(b, ops),
    };
    ops.insert(current, Operation::Number(value));
    value
}

pub fn riddle_21_1(lines: io::Lines<io::BufReader<File>>) {
    let mut ops = read_operations(lines);
    let result = calc("root".to_string(), &mut ops);
    println!("The solution is: {:?}", result);
}
