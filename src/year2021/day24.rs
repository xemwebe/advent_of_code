use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}
use std::collections::HashMap;

fn read_instructions(lines: io::Lines<io::BufReader<File>>) -> Vec<Instruction> {
    lines
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| Instruction::from_str(&x))
        .collect()
}

#[derive(Debug)]
enum Arg {
    Register(u8),
    Number(i64),
}

impl Arg {
    fn from_str(s: &str) -> Arg {
        match s {
            "w" => Arg::Register(0),
            "x" => Arg::Register(1),
            "y" => Arg::Register(2),
            "z" => Arg::Register(3),
            _ => Arg::Number(s.parse::<i64>().unwrap()),
        }
    }

    fn get_number(&self, registers: &[i64]) -> i64 {
        match self {
            Arg::Number(x) => *x,
            Arg::Register(a) => registers[*a as usize],
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Inp(Arg),
    Add(Arg, Arg),
    Mul(Arg, Arg),
    Div(Arg, Arg),
    Mod(Arg, Arg),
    Eql(Arg, Arg),
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        let tokens: Vec<String> = s.split(" ").map(|x| x.to_owned()).collect();
        match tokens[0].as_str() {
            "inp" => Instruction::Inp(Arg::from_str(&tokens[1])),
            "add" => Instruction::Add(Arg::from_str(&tokens[1]), Arg::from_str(&tokens[2])),
            "mul" => Instruction::Mul(Arg::from_str(&tokens[1]), Arg::from_str(&tokens[2])),
            "div" => Instruction::Div(Arg::from_str(&tokens[1]), Arg::from_str(&tokens[2])),
            "mod" => Instruction::Mod(Arg::from_str(&tokens[1]), Arg::from_str(&tokens[2])),
            "eql" => Instruction::Eql(Arg::from_str(&tokens[1]), Arg::from_str(&tokens[2])),
            _ => panic!("Unknown command found!"),
        }
    }
}

fn process_instructions(input: &[i64], z: i64, instructions: &[Instruction]) -> i64 {
    let mut input_counter = 0;
    let mut registers = vec![0; 4];
    registers[3] = z;
    for instruction in instructions {
        match instruction {
            Instruction::Inp(a) => {
                if let Arg::Register(a) = a {
                    registers[*a as usize] = input[input_counter];
                    input_counter += 1;
                }
            }
            Instruction::Add(a, b) => {
                if let Arg::Register(a) = a {
                    registers[*a as usize] += b.get_number(&registers);
                }
            }
            Instruction::Mul(a, b) => {
                if let Arg::Register(a) = a {
                    registers[*a as usize] *= b.get_number(&registers);
                }
            }
            Instruction::Div(a, b) => {
                if let Arg::Register(a) = a {
                    if b.get_number(&registers) == 0 {
                        return i64::MAX;
                    }
                    registers[*a as usize] /= b.get_number(&registers);
                }
            }
            Instruction::Mod(a, b) => {
                if let Arg::Register(a) = a {
                    if registers[*a as usize] < 0 || b.get_number(&registers) <= 0 {
                        return i64::MAX;
                    }
                    registers[*a as usize] %= b.get_number(&registers);
                }
            }
            Instruction::Eql(a, b) => {
                if let Arg::Register(a) = a {
                    registers[*a as usize] = if registers[*a as usize] == b.get_number(&registers) {
                        1
                    } else {
                        0
                    };
                }
            }
        }
    }
    return registers[3];
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let instructions = read_instructions(lines);
    let mut start_indices = Vec::new();
    for (idx, ins) in instructions.iter().enumerate() {
        if let Instruction::Inp(_) = ins {
            start_indices.push(idx);
        }
    }
    start_indices.push(instructions.len());
    let mut allowed_z = HashMap::new();
    allowed_z.insert(0, vec![]);
    for i in (0..start_indices.len() - 1).rev() {
        let mut new_allowed_z = HashMap::new();
        for z in 0..250000 {
            for w in (1..=9).rev() {
                let result = process_instructions(
                    &[w],
                    z,
                    &instructions[start_indices[i]..start_indices[i + 1]],
                );
                if allowed_z.contains_key(&result) {
                    let mut new_input = allowed_z[&result].clone();
                    new_input.push(w);
                    new_allowed_z.insert(z, new_input);
                    break;
                }
            }
        }
        allowed_z = new_allowed_z;
    }
    let mut solution =  0;
    for d in allowed_z[&0].iter().rev() {
        solution = solution * 10 + *d;
    }
    format!("{}", solution)
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let instructions = read_instructions(lines);
    let mut start_indices = Vec::new();
    for (idx, ins) in instructions.iter().enumerate() {
        if let Instruction::Inp(_) = ins {
            start_indices.push(idx);
        }
    }
    start_indices.push(instructions.len());
    let mut allowed_z = HashMap::new();
    allowed_z.insert(0, vec![]);
    for i in (0..start_indices.len() - 1).rev() {
        let mut new_allowed_z = HashMap::new();
        for z in 0..160000 {
            for w in 1..=9 {
                let result = process_instructions(
                    &[w],
                    z,
                    &instructions[start_indices[i]..start_indices[i + 1]],
                );
                if allowed_z.contains_key(&result) {
                    let mut new_input = allowed_z[&result].clone();
                    new_input.push(w);
                    new_allowed_z.insert(z, new_input);
                    break;
                }
            }
        }
        allowed_z = new_allowed_z;
    }
    let mut solution =  0;
    for d in allowed_z[&0].iter().rev() {
        solution = solution * 10 + *d;
    }
    format!("{}", solution)
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2021_24_1() {
        let lines = read_lines("data/2021/24.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "98491959997994");
    }

    #[test]
    fn test_2021_24_2() {
        let lines = read_lines("data/2021/24.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "61191516111321");
    }
}

