use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io,
};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct Signal {
    from: usize,
    to: usize,
    p: Pulse,
}

#[derive(Debug)]
struct Queue {
    low_count: u64,
    high_count: u64,
    inner: VecDeque<Signal>,
}

impl Queue {
    fn new() -> Self {
        Self {
            low_count: 0,
            high_count: 0,
            inner: VecDeque::new(),
        }
    }

    fn add_signals(&mut self, from: usize, to: &[usize], p: Pulse) {
        match &p {
            Pulse::High => self.high_count += to.len() as u64,
            Pulse::Low => self.low_count += to.len() as u64,
        }
        for t in to {
            self.inner.push_back(Signal {
                from,
                to: *t,
                p: p.clone(),
            });
        }
    }

    fn score(&self) -> u64 {
        self.low_count * self.high_count
    }
}

trait Machine {
    fn process(&mut self, s: &Signal, q: &mut Queue);
    fn state(&self, total: &mut Vec<u8>);
    fn init_inputs(&mut self, _inputs: &[usize]) {}
}

#[derive(Debug)]
struct FlipFlop {
    name: usize,
    is_on: bool,
    outputs: Vec<usize>,
}

impl FlipFlop {
    fn new(name: usize, outputs: Vec<usize>) -> Self {
        Self {
            name,
            is_on: false,
            outputs,
        }
    }
}

impl Machine for FlipFlop {
    fn process(&mut self, s: &Signal, q: &mut Queue) {
        match s.p {
            Pulse::High => {}
            Pulse::Low => {
                self.is_on = !self.is_on;
                if self.is_on {
                    q.add_signals(self.name, &self.outputs, Pulse::High);
                } else {
                    q.add_signals(self.name, &self.outputs, Pulse::Low);
                }
            }
        }
    }

    fn state(&self, total: &mut Vec<u8>) {
        total.push(if self.is_on { 1 } else { 0 });
    }
}

#[derive(Debug)]
struct Conjunction {
    name: usize,
    state: HashMap<usize, Pulse>,
    outputs: Vec<usize>,
}

impl Conjunction {
    fn new(name: usize, outputs: Vec<usize>) -> Self {
        Self {
            name,
            state: HashMap::new(),
            outputs,
        }
    }
}

impl Machine for Conjunction {
    fn process(&mut self, s: &Signal, q: &mut Queue) {
        //println!("inputs: {:?}", self.state.keys());
        *self.state.get_mut(&s.from).unwrap() = s.p.clone();
        let mut pulse = Pulse::Low;
        for p in self.state.values() {
            if *p == Pulse::Low {
                pulse = Pulse::High;
                break;
            }
        }
        q.add_signals(self.name, &self.outputs, pulse);
    }

    fn state(&self, total: &mut Vec<u8>) {
        let mut bits: u8 = 0;
        for p in self.state.values() {
            bits <<= 1;
            if *p == Pulse::High {
                bits |= 1;
            }
        }
        total.push(bits);
    }

    fn init_inputs(&mut self, inputs: &[usize]) {
        for input in inputs {
            self.state.insert(*input, Pulse::Low);
        }
    }
}

#[derive(Debug)]
struct Broadcaster {
    name: usize,
    outputs: Vec<usize>,
}

impl Broadcaster {
    fn new(name: usize, outputs: Vec<usize>) -> Self {
        Self { name, outputs }
    }
}
impl Machine for Broadcaster {
    fn process(&mut self, s: &Signal, q: &mut Queue) {
        q.add_signals(self.name, &self.outputs, s.p.clone());
    }

    fn state(&self, _total: &mut Vec<u8>) {}
}

fn insert_inputs(mapping: &mut Vec<Vec<usize>>, from: usize, outputs: &[usize]) {
    for o in outputs {
        if *o < mapping.len() {
            mapping[*o].push(from);
        }
    }
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut rules = Vec::new();
    for l in lines {
        rules.push(l.unwrap());
    }

    let mut name_map = HashMap::new();
    let mut idx = 0usize;
    for line in &rules {
        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts[0] == "broadcaster" {
            name_map.insert(parts[0].to_string(), idx);
        } else {
            name_map.insert(parts[0][1..].to_string(), idx);
        }
        idx += 1;
    }
    name_map.insert("rx".to_string(), idx);

    let mut machines: Vec<Box<dyn Machine>> = Vec::with_capacity(idx);
    let mut input_map = vec![Vec::new(); idx];
    for line in &rules {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let outputs: Vec<usize> = parts[1].split(", ").map(|s| name_map[s]).collect();
        if parts[0] == "broadcaster" {
            let idx = name_map[parts[0]];
            insert_inputs(&mut input_map, idx, &outputs);
            machines.push(Box::new(Broadcaster::new(idx, outputs)));
        } else if parts[0].starts_with('%') {
            let idx = name_map[&parts[0][1..]];
            insert_inputs(&mut input_map, idx, &outputs);
            machines.push(Box::new(FlipFlop::new(idx, outputs)));
        } else if parts[0].starts_with('&') {
            let idx = name_map[&parts[0][1..]];
            insert_inputs(&mut input_map, idx, &outputs);
            machines.push(Box::new(Conjunction::new(idx, outputs)));
        } else {
            panic!("invalid machine type");
        }
    }

    for from in 0..input_map.len() {
        machines[from].init_inputs(&input_map[from]);
    }
    //println!("machines: {:?}", machines.keys());
    let mut init_state = Vec::new();
    for m in &machines {
        m.state(&mut init_state);
    }
    //println!("count: 0, state: {init_state:?}");
    let mut queue = Queue::new();
    for _ in 0..1000 {
        queue.add_signals(usize::MAX, &[name_map["broadcaster"]], Pulse::Low);
        while !queue.inner.is_empty() {
            let signal = queue.inner.pop_front().unwrap();
            //println!("Signal: {signal:?}");
            if let Some(m) = machines.get_mut(signal.to) {
                m.process(&signal, &mut queue);
            }
        }
        let mut state = Vec::new();
        for m in &machines {
            m.state(&mut state);
        }
        //println!("count: {count}, state: {state:?}");
        if state == init_state {
            break;
        }
    }
    let score = queue.score();
    format!("{}", score)
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut rules = Vec::new();
    for l in lines {
        rules.push(l.unwrap());
    }

    let mut name_map = HashMap::new();
    let mut idx = 0usize;
    for line in &rules {
        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts[0] == "broadcaster" {
            name_map.insert(parts[0].to_string(), idx);
        } else {
            name_map.insert(parts[0][1..].to_string(), idx);
        }
        idx += 1;
    }
    name_map.insert("rx".to_string(), idx);
    name_map.insert("output".to_string(), idx);

    let mut machines: Vec<Box<dyn Machine>> = Vec::with_capacity(idx);
    let mut input_map = vec![Vec::new(); idx];
    for line in &rules {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let outputs: Vec<usize> = parts[1].split(", ").map(|s| name_map[s]).collect();
        if parts[0] == "broadcaster" {
            let idx = name_map[parts[0]];
            insert_inputs(&mut input_map, idx, &outputs);
            machines.push(Box::new(Broadcaster::new(idx, outputs)));
        } else if parts[0].starts_with('%') {
            let idx = name_map[&parts[0][1..]];
            insert_inputs(&mut input_map, idx, &outputs);
            machines.push(Box::new(FlipFlop::new(idx, outputs)));
        } else if parts[0].starts_with('&') {
            let idx = name_map[&parts[0][1..]];
            insert_inputs(&mut input_map, idx, &outputs);
            machines.push(Box::new(Conjunction::new(idx, outputs)));
        } else {
            panic!("invalid machine type");
        }
    }

    for from in 0..input_map.len() {
        machines[from].init_inputs(&input_map[from]);
    }

    //println!("machines: {:?}", machines.keys());
    let mut queue = Queue::new();
    let mut count = 0;
    let mut no_rx = true;
    while no_rx {
        count += 1;
        queue.add_signals(usize::MAX, &[name_map["broadcaster"]], Pulse::Low);
        while !queue.inner.is_empty() {
            let signal = queue.inner.pop_front().unwrap();
            if signal.to == machines.len() {
                if signal.p == Pulse::Low {
                    no_rx = false;
                    break;
                }
            } else {
                machines[signal.to].process(&signal, &mut queue);
            }
        }
    }
    format!("{count}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2023_20_1() {
        let lines = read_lines("data/2023/20.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "45159");
    }

    #[test]
    fn test_2023_20_2() {
        let lines = read_lines("data/2023/20.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "134549294799713");
    }
}
