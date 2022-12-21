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
    Equal(String, String),
}

fn read_operations(lines: io::Lines<io::BufReader<File>>) -> HashMap<String, Operation> {
    let re_num = Regex::new("([a-z]+): ([0-9]+)").unwrap();
    let re_op = Regex::new("([a-z]+): ([a-z]+) ([-+*=/]) ([a-z]+)").unwrap();
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
                "/" => Operation::Div(cap[2].to_owned(), cap[4].to_owned()),
                "=" => Operation::Equal(cap[2].to_owned(), cap[4].to_owned()),
                s => panic!("Operation '{s}' is not supported")
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
        _ => panic!("Unsupported operation found for mode 1")
    };
    ops.insert(current, Operation::Number(value));
    value
}

pub fn riddle_21_1(lines: io::Lines<io::BufReader<File>>) {
    let mut ops = read_operations(lines);
    let result = calc("root".to_string(), &mut ops);
    println!("The solution is: {:?}", result);
}


fn check(a: String, b: String, ops: &mut HashMap<String, Operation>) -> Option<(i64, i64)> {
    if let Some(x) = pre_calc(a, ops) {
        if let Some(y) = pre_calc(b, ops)  {
            return Some((x,y));
        }
    }
    None
}

fn pre_calc(current: String, ops: &mut HashMap<String, Operation>) -> Option<i64> {
    if current == "humn" {
        return None;
    }
    let value = match ops[&current].clone() {
        Operation::Number(x) => { return Some(x) }
        Operation::Add(a, b) => { if let Some((x,y)) = check(a,b,ops) { Some(x+y) } else { None } },
        Operation::Sub(a, b) => { if let Some((x,y)) = check(a,b,ops) { Some(x-y) } else { None } },
        Operation::Mul(a, b) => { if let Some((x,y)) = check(a,b,ops) { Some(x*y) } else { None } },
        Operation::Div(a, b) => { if let Some((x,y)) = check(a,b,ops) { Some(x/y) } else { None } },
        Operation::Equal(a,b) => { let _ = check(a,b,ops); None }
    };
    if let Some(value) = value { 
        ops.insert(current, Operation::Number(value));
    }
    value
}

fn calc_plain(current: &str, ops: &HashMap<String, Operation>) -> i64 {
    let value = match ops[current].clone() {
        Operation::Number(x) => x,
        Operation::Add(a, b) => calc_plain(&a, ops) + calc_plain(&b, ops),
        Operation::Sub(a, b) => calc_plain(&a, ops) - calc_plain(&b, ops),
        Operation::Mul(a, b) => calc_plain(&a, ops) * calc_plain(&b, ops),
        Operation::Div(a, b) => calc_plain(&a, ops) / calc_plain(&b, ops),
        _ => panic!("Unsupported operation found for mode 1")
    };
    value
}

fn roots_judgement(ops: &HashMap<String, Operation>) -> (i64, i64) {
    if let Operation::Equal(a,b) = &ops["root"] {
        (calc_plain(a, ops), calc_plain(b, ops))
    } else {
        panic!("root is not equal operation");
    }
}

fn solution_included(r1: (i64, i64), r2: (i64, i64)) -> bool {
    ((r1.1-r1.0)/(r1.1-r1.0).abs())*((r2.1-r2.0)/(r2.1-r2.0).abs()) < 0
}

pub fn riddle_21_2(lines: io::Lines<io::BufReader<File>>) {
    let mut ops = read_operations(lines);
    let keys: Vec<String> = ops.keys().into_iter().map(|x| x.clone()).collect();
    for k in keys {
        let _ = pre_calc(k.to_owned(), &mut ops);
    }
    
    let mut lower = 1;
    let mut upper = 10;
    let mut lr = (0,0);
    let mut ur = (0,0);
    loop {
        ops.insert("humn".to_owned(), Operation::Number(lower));
        lr = roots_judgement(&ops);
        ops.insert("humn".to_owned(), Operation::Number(upper));
        ur = roots_judgement(&ops);
        if solution_included(lr, ur) {
            break;
        }
        lower *= 10;
        upper *= 10;
    }
    println!("search intervall: [{lower},{upper}");
    loop {
        let middle = (lower+upper)/2;
        ops.insert("humn".to_owned(), Operation::Number(middle));
        let mr = roots_judgement(&ops);
        if mr.0 == mr.1 {
            println!("Solution: {middle}");
            break;
        }
        if solution_included(lr, mr) {
            upper = middle;
        } else {
            lower = middle;
        }
    }
}


