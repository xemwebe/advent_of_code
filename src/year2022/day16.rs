use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

use bitfield::{bitfield_bitrange, Bit, BitMut};
use regex::Regex;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
struct Valve {
    pub id: usize,
    pub rate: i32,
    pub next: HashMap<String, i32>,
}

fn read_valves(lines: io::Lines<io::BufReader<File>>) -> HashMap<String, Valve> {
    let re = Regex::new(
        r"Valve ([A-Z]{2}) has flow rate=([0-9]*); tunnel[s]? lead[s]? to valve[s]? ([,A-Z ]*)",
    )
    .unwrap();
    let mut id = 0;
    lines
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| {
            let r = re.captures_iter(&x).into_iter().next().unwrap();
            let name = r.get(1).unwrap().as_str().to_owned();
            let rate = r.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let next_str = r.get(3).unwrap().as_str();
            let valve = Valve {
                id,
                rate,
                next: next_str.split(", ").map(|s| (s.to_owned(), 1)).collect(),
            };
            id += 1;
            (name, valve)
        })
        .collect()
}

#[derive(Debug, Clone)]
struct OpenStat(u64);
bitfield_bitrange! {struct OpenStat(u64)}

#[derive(Debug, Clone)]
struct State {
    open: OpenStat,
    ticker: i32,
    node: String,
}

fn calc_distances(valves: &HashMap<String, Valve>) -> HashMap<String, Valve> {
    let mut new_valves = HashMap::new();
    for valve in valves {
        let mut followers = valve.1.next.clone();
        let mut next_level = followers.clone();
        for i in 2..30 {
            let mut new_followers = HashMap::new();
            for node in &next_level {
                for n in &valves[node.0].next {
                    if !followers.contains_key(n.0) && n.0 != valve.0 {
                        new_followers.insert(n.0.to_owned(), i);
                        followers.insert(n.0.to_owned(), i);
                    }
                }
            }
            next_level = new_followers;
        }
        if valve.1.rate != 0 || valve.0 == "AA" {
            for v in valves {
                if v.1.rate == 0 {
                    followers.remove(v.0);
                }
            }
            new_valves.insert(
                valve.0.to_owned(),
                Valve {
                    id: valve.1.id,
                    rate: valve.1.rate,
                    next: followers,
                },
            );
        }
    }
    new_valves
}

fn calc_weights(state: &State, valves: &HashMap<String, Valve>) -> BTreeMap<i32, String> {
    let mut weights = BTreeMap::new();
    for v in &valves[&state.node].next {
        if ((state.ticker + *v.1) < 30) && !state.open.bit(valves[v.0].id) {
            let weight = (30 - (state.ticker + *v.1 + 1)) * valves[v.0].rate;
            weights.insert(weight, v.0.to_owned());
        }
    }
    weights
}

fn max_pressure(valves: &HashMap<String, Valve>, state: State) -> i32 {
    let prio_map = calc_weights(&state, valves);

    let mut max_released = 0;
    for p in prio_map.iter().rev() {
        let mut open = state.open.clone();
        open.set_bit(valves[p.1].id, true);
        max_released = max_released.max(
            *p.0 + max_pressure(
                valves,
                State {
                    open,
                    ticker: state.ticker + 1 + valves[&state.node].next[p.1],
                    node: p.1.to_owned(),
                },
            ),
        )
    }

    max_released
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let valves = read_valves(lines);
    let valves = calc_distances(&valves);
    let state = State {
        open: OpenStat(0),
        ticker: 0,
        node: "AA".to_owned(),
    };
    let max_released = max_pressure(&valves, state);
    format!("{max_released}")
}

#[derive(Debug, Clone)]
struct State2 {
    open: OpenStat,
    ticker: (i32, i32),
    node: (Option<String>, Option<String>),
}

fn calc_weights2(
    state: &State2,
    valves: &HashMap<String, Valve>,
) -> BTreeMap<i32, (Option<String>, Option<String>)> {
    let mut weights1 = BTreeMap::new();
    if let Some(node0) = &state.node.0 {
        for v in &valves[node0].next {
            if ((state.ticker.0 + *v.1) < 26) && !state.open.bit(valves[v.0].id) {
                let weight = (26 - (state.ticker.0 + *v.1 + 1)) * valves[v.0].rate;
                weights1.insert(weight, v.0.to_owned());
            }
        }
    }

    let mut weights2 = BTreeMap::new();
    if let Some(node1) = &state.node.1 {
        for v in &valves[node1].next {
            if ((state.ticker.1 + *v.1) < 26) && !state.open.bit(valves[v.0].id) {
                let weight = (26 - (state.ticker.1 + *v.1 + 1)) * valves[v.0].rate;
                weights2.insert(weight, v.0.to_owned());
            }
        }
    }
    let mut double_weights = BTreeMap::new();
    if !weights1.is_empty() || !weights2.is_empty() {
        if weights1.is_empty() {
            for w2 in &weights2 {
                double_weights.insert(*w2.0, (None, Some(w2.1.to_owned())));
            }
        }
        if weights2.is_empty() {
            for w1 in &weights1 {
                double_weights.insert(*w1.0, (Some(w1.1.to_owned()), None));
            }
        }
        for w1 in &weights1 {
            for w2 in &weights2 {
                if w2.1 != w1.1 {
                    double_weights.insert(
                        *w1.0 + *w2.0,
                        (Some(w1.1.to_owned()), Some(w2.1.to_owned())),
                    );
                }
            }
        }
    }
    double_weights
}

fn max_pressure2(valves: &HashMap<String, Valve>, state: State2) -> i32 {
    let prio_map = calc_weights2(&state, valves);

    let mut max_released = 0;
    for p in prio_map.iter().rev() {
        let mut open = state.open.clone();
        let mut new_ticker = (state.ticker.0, state.ticker.1);
        let nodes = state.node.clone();
        if let Some(node0) = &p.1 .0 {
            open.set_bit(valves[node0].id, true);
            new_ticker.0 += 1 + valves[&nodes.0.unwrap()].next[node0]
        }
        if let Some(node1) = &p.1 .1 {
            open.set_bit(valves[node1].id, true);
            new_ticker.1 += 1 + valves[&nodes.1.unwrap()].next[node1]
        }
        max_released = max_released.max(
            *p.0 + max_pressure2(
                valves,
                State2 {
                    open,
                    ticker: new_ticker,
                    node: (p.1 .0.to_owned(), p.1 .1.to_owned()),
                },
            ),
        );
    }

    max_released
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let valves = read_valves(lines);
    let valves = calc_distances(&valves);
    let state = State2 {
        open: OpenStat(0),
        ticker: (0, 0),
        node: (Some("AA".to_owned()), Some("AA".to_owned())),
    };
    let max_released = max_pressure2(&valves, state);
    format!("{max_released}")
}
