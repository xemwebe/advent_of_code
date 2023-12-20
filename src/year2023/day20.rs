use std::{
    fs::File, 
    io,
    collections::{VecDeque, HashMap},
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
struct Signal<'a> {
    from: &'a str,
    to: &'a str,
    p: Pulse
}

#[derive(Debug)]
struct Queue<'a> {
    low_count: u64,
    high_count: u64,
    inner: VecDeque<Signal<'a>>
}

impl<'a> Queue<'a> {
    fn new() -> Self {
        Self {
            low_count: 0,
            high_count: 0,
            inner: VecDeque::new(),
        }
    }

    fn add_signals(&mut self, from: &'a str, to: &[&'a str], p: Pulse) {
        match &p {
            Pulse::High => { self.high_count += to.len() as u64 },
            Pulse::Low => { self.low_count += to.len() as u64 },
        }
        for t in to {
            self.inner.push_back(Signal { from, to: *t, p: p.clone() });
        }
    }

    fn score(&self) -> u64 {
        self.low_count*self.high_count
    }
}

trait Machine<'a> {
    fn process(&mut self, s: &Signal<'a>, q: &mut Queue<'a>);
    fn state(&self, total: &mut Vec<u8>);
    fn init_inputs(&mut self, _inputs: &[&'a str]) {}
}


#[derive(Debug)]
struct FlipFlop<'a> {
    name: &'a str,
    is_on: bool,
    outputs: Vec<&'a str>,
}

impl<'a> FlipFlop<'a> {
    fn new(name: &'a str, outputs: Vec<&'a str>) -> Self {
        Self {
            name,
            is_on: false,
            outputs
        }
    }
}

impl<'a> Machine<'a> for FlipFlop<'a> {
    fn process(&mut self, s: &Signal<'a>, q: &mut Queue<'a>) {
        match s.p {
            Pulse::High => {},
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
        total.push(if self.is_on { 1 } else { 0 } );
    }
}

#[derive(Debug)]
struct Conjunction<'a> {
    name: &'a str,
    state: HashMap<&'a str, Pulse>,
    outputs: Vec<&'a str>,
}

impl<'a> Conjunction<'a> {
    fn new(name: &'a str, outputs: Vec<&'a str>) -> Self {
        Self {
            name,
            state: HashMap::new(),
            outputs
        }
    }
}

impl<'a> Machine<'a> for Conjunction<'a> {
    fn process(&mut self, s: &Signal<'a>, q: &mut Queue<'a>) {
        //println!("inputs: {:?}", self.state.keys());
        *self.state.get_mut(s.from).unwrap() = s.p.clone();
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
            bits <<=1;
            if *p == Pulse::High {
                bits |= 1;
            }
        }
        total.push(bits);
    }

    fn init_inputs(&mut self, inputs: &[&'a str]) {
        for input in inputs {
            self.state.insert(*input, Pulse::Low);
        }
    }
}

#[derive(Debug)]
struct Broadcaster<'a> {
    name: &'a str,
    outputs: Vec<&'a str>,
}

impl<'a> Broadcaster<'a> {
    fn new(name: &'a str, outputs: Vec<&'a str>) -> Self {
        Self {
            name,
            outputs
        }
    }
}
impl<'a> Machine<'a> for Broadcaster<'a> {
    fn process(&mut self, s: &Signal<'a>, q: &mut Queue<'a>) {
        q.add_signals(self.name, &self.outputs, s.p.clone());
    }

    fn state(&self, _total: &mut Vec<u8>) {}
}

fn insert_inputs<'a>(mapping: &mut HashMap<&'a str, Vec<&'a str>>, from: &'a str, outputs: &Vec<&'a str>) {
    for o in outputs {
        if !mapping.contains_key(o) {
            mapping.insert(o, Vec::new());
        }
        (*mapping.get_mut(o).unwrap()).push(from);
    }
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut rules = Vec::new();
    for l in lines {
        rules.push(l.unwrap());
    }
    let mut machines: HashMap<&str, Box<dyn Machine>> = HashMap::new();
    let mut input_map = HashMap::new();
    for line in &rules {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let outputs: Vec<&str> = parts[1].split(", ").collect();
        if parts[0] == "broadcaster" {
            insert_inputs(&mut input_map, parts[0], &outputs);
            machines.insert(parts[0], Box::new(Broadcaster::new(parts[0], outputs)));
        } else if parts[0].starts_with('%') {
            insert_inputs(&mut input_map, &parts[0][1..], &outputs);
            machines.insert(&parts[0][1..], Box::new(FlipFlop::new(&parts[0][1..], outputs)));
        } else if parts[0].starts_with('&') {
            insert_inputs(&mut input_map, &parts[0][1..], &outputs);
            machines.insert(&parts[0][1..], Box::new(Conjunction::new(&parts[0][1..], outputs)));
        } else {
            panic!("invalid machine type");
        }
    }

    for (k, value) in input_map {
        if let Some(m) = machines.get_mut(k) {
            m.init_inputs(&value);
        }
    }

    //println!("machines: {:?}", machines.keys());
    let mut init_state = Vec::new();
    for m in machines.values() {
        m.state(&mut init_state);
    }
    //println!("count: 0, state: {init_state:?}");
    let mut queue = Queue::new();
    for _ in 0..1000 {
        queue.add_signals("button", &["broadcaster"], Pulse::Low);
        while !queue.inner.is_empty() {
            let signal = queue.inner.pop_front().unwrap();
            //println!("Signal: {signal:?}");
            if let Some(m) = machines.get_mut(signal.to) {
                m.process(&signal, &mut queue);
            }
        }
        let mut state = Vec::new();
        for m in machines.values() {
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
    let mut machines: HashMap<&str, Box<dyn Machine>> = HashMap::new();
    let mut input_map = HashMap::new();
    for line in &rules {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let outputs: Vec<&str> = parts[1].split(", ").collect();
        if parts[0] == "broadcaster" {
            insert_inputs(&mut input_map, parts[0], &outputs);
            machines.insert(parts[0], Box::new(Broadcaster::new(parts[0], outputs)));
        } else if parts[0].starts_with('%') {
            insert_inputs(&mut input_map, &parts[0][1..], &outputs);
            machines.insert(&parts[0][1..], Box::new(FlipFlop::new(&parts[0][1..], outputs)));
        } else if parts[0].starts_with('&') {
            insert_inputs(&mut input_map, &parts[0][1..], &outputs);
            machines.insert(&parts[0][1..], Box::new(Conjunction::new(&parts[0][1..], outputs)));
        } else {
            panic!("invalid machine type");
        }
    }

    for (k, value) in input_map {
        if let Some(m) = machines.get_mut(k) {
            m.init_inputs(&value);
        }
    }

    //println!("machines: {:?}", machines.keys());
    let mut queue = Queue::new();
    let mut count = 0;
    let mut no_rx = true;
    while no_rx {
        count += 1;
        queue.add_signals("button", &["broadcaster"], Pulse::Low);
        while !queue.inner.is_empty() {
            let signal = queue.inner.pop_front().unwrap();
            if signal.to == "rx" && signal.p == Pulse::Low {
                no_rx = false;
            }
            //println!("Signal: {signal:?}");
            if let Some(m) = machines.get_mut(signal.to) {
                m.process(&signal, &mut queue);
            }
        }
    }
    format!("{count}")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

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
