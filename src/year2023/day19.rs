use std::{collections::HashMap, fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    LowerThan(u32),
    GreaterThan(u32),
    True,
}

impl Operation {
    fn new(b: u8, param: u32) -> Self {
        match b {
            b'<' => Self::LowerThan(param),
            b'>' => Self::GreaterThan(param),
            _ => panic!("invalid op"),
        }
    }
    fn test(&self, value: u32) -> bool {
        match self {
            Self::LowerThan(param) => value < *param,
            Self::GreaterThan(param) => value > *param,
            Self::True => true,
        }
    }
    fn apply_on_range(&self, r: Range) -> (Range, Range) {
        match self {
            Self::LowerThan(param) => {
                if r.max < *param {
                    (r, Range { min: 0, max: 0 })
                } else if r.min >= *param {
                    (Range { min: 0, max: 0 }, r)
                } else {
                    (
                        Range {
                            min: r.min,
                            max: r.max.min(param - 1),
                        },
                        Range {
                            min: r.min.max(*param),
                            max: r.max,
                        },
                    )
                }
            }
            Self::GreaterThan(param) => {
                if r.min > *param {
                    (r, Range { min: 0, max: 0 })
                } else if r.max <= *param {
                    (Range { min: 0, max: 0 }, r)
                } else {
                    (
                        Range {
                            min: r.min.max(*param + 1),
                            max: r.max,
                        },
                        Range {
                            min: r.min,
                            max: r.max.min(*param),
                        },
                    )
                }
            }
            Self::True => (r, Range { min: 0, max: 0 }),
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    var_idx: usize,
    op: Operation,
    target: String,
}

#[derive(Debug, Clone)]
struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    fn check(&self, item: &[u32; 4], rules: &HashMap<String, RuleSet>) -> bool {
        for i in 0..self.rules.len() {
            let rule = &self.rules[i];
            if rule.op.test(item[rule.var_idx]) {
                if &rule.target == "A" {
                    return true;
                } else if &rule.target == "R" {
                    return false;
                } else {
                    return rules[&rule.target].check(item, rules);
                }
            }
        }
        false
    }

    fn passing_items(&self, mut ranges: [Range; 4], rules: &HashMap<String, RuleSet>) -> u64 {
        let mut sum = 0;
        for i in 0..self.rules.len() {
            let rule = &self.rules[i];
            let (passed_range, rejected_range) = rule.op.apply_on_range(ranges[rule.var_idx]);
            if &rule.target == "A" {
                let mut prod = (passed_range.max - passed_range.min + 1) as u64;
                for j in 0..4 {
                    if j != rule.var_idx {
                        if ranges[j].max > 0 {
                            prod *= (ranges[j].max - ranges[j].min + 1) as u64;
                        } else {
                            prod = 0;
                        }
                    }
                }
                sum += prod;
            } else if &rule.target != "R" {
                let mut new_ranges = ranges.clone();
                new_ranges[rule.var_idx] = passed_range;
                sum += rules[&rule.target].passing_items(new_ranges, rules);
            }
            ranges[rule.var_idx] = rejected_range;
        }
        sum
    }
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut rules_mode = true;
    let mut rules = HashMap::new();
    let mut items = Vec::new();
    for l in lines {
        let line = l.unwrap();
        if line.is_empty() {
            rules_mode = false;
            continue;
        }
        if rules_mode {
            let parts: Vec<&str> = line.split('{').collect();
            let name = parts[0].to_string();
            let rules_len = parts[1].len() - 1;
            let rule_infos: Vec<&str> = parts[1][0..rules_len].split(',').collect();
            let mut set_rules = Vec::new();
            for rule_info in rule_infos {
                let rule_parts: Vec<&str> = rule_info.split(':').collect();
                let rule = if rule_parts.len() == 1 {
                    let target = rule_parts[0].to_string();
                    Rule {
                        var_idx: 0,
                        op: Operation::True,
                        target,
                    }
                } else {
                    let condition_variable = match rule_parts[0].as_bytes()[0] {
                        b'x' => 0,
                        b'm' => 1,
                        b'a' => 2,
                        b's' => 3,
                        _ => panic!("invalid variable"),
                    };
                    let operator = rule_parts[0].as_bytes()[1];
                    let condition_value: u32 = rule_parts[0][2..].parse().unwrap();
                    let op = Operation::new(operator, condition_value);
                    let target = rule_parts[1].to_string();
                    Rule {
                        var_idx: condition_variable,
                        op,
                        target,
                    }
                };
                set_rules.push(rule);
            }
            rules.insert(name, RuleSet { rules: set_rules });
        } else {
            let parts: Vec<&str> = line[1..line.len() - 1].split(',').collect();
            let mut tile = [0u32; 4];
            for i in 0..4 {
                tile[i] = parts[i].split('=').skip(1).next().unwrap().parse().unwrap();
            }
            items.push(tile);
        }
    }

    let mut sum = 0;
    for item in items {
        if rules["in"].check(&item, &rules) {
            sum += item[0] + item[1] + item[2] + item[3];
        }
    }
    format!("{sum}")
}

#[derive(Clone, Copy, Debug)]
struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn new() -> Self {
        Self { min: 1, max: 4000 }
    }
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut rules = HashMap::new();
    for l in lines {
        let line = l.unwrap();
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split('{').collect();
        let name = parts[0].to_string();
        let rules_len = parts[1].len() - 1;
        let rule_infos: Vec<&str> = parts[1][0..rules_len].split(',').collect();
        let mut set_rules = Vec::new();
        for rule_info in rule_infos {
            let rule_parts: Vec<&str> = rule_info.split(':').collect();
            let rule = if rule_parts.len() == 1 {
                let target = rule_parts[0].to_string();
                Rule {
                    var_idx: 0,
                    op: Operation::True,
                    target,
                }
            } else {
                let condition_variable = match rule_parts[0].as_bytes()[0] {
                    b'x' => 0,
                    b'm' => 1,
                    b'a' => 2,
                    b's' => 3,
                    _ => panic!("invalid variable"),
                };
                let operator = rule_parts[0].as_bytes()[1];
                let condition_value: u32 = rule_parts[0][2..].parse().unwrap();
                let op = Operation::new(operator, condition_value);
                let target = rule_parts[1].to_string();
                Rule {
                    var_idx: condition_variable,
                    op,
                    target,
                }
            };
            set_rules.push(rule);
        }
        rules.insert(name, RuleSet { rules: set_rules });
    }

    let ranges = [Range::new(); 4];
    let sum = rules["in"].passing_items(ranges, &rules);
    format!("{sum}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2023_19_1() {
        let lines = read_lines("data/2023/19.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "333263");
    }

    #[test]
    fn test_2023_19_2() {
        let lines = read_lines("data/2023/19.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "130745440937650");
    }
}
