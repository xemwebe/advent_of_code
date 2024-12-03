use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

use regex::Regex;

#[derive(Debug, Clone)]
struct Cost {
    pub ore: i32,
    pub clay: i32,
    pub obsidian: i32,
}

#[derive(Debug, Clone)]
struct BluePrint {
    pub id: i32,
    pub ore_robot_cost: Cost,
    pub clay_robot_cost: Cost,
    pub obsidian_robot_cost: Cost,
    pub geode_robot_cost: Cost,
}

impl BluePrint {
    fn max_ore(&self) -> i32 {
        self.clay_robot_cost
            .ore
            .max(self.obsidian_robot_cost.ore)
            .max(self.geode_robot_cost.ore)
    }
}

fn read_blueprints(lines: io::Lines<io::BufReader<File>>) -> Vec<BluePrint> {
    let re = Regex::new(
        r"Blueprint ([0-9]*):.* ([0-9]*) ore.* ([0-9]*) ore.* ([0-9]*) ore.* ([0-9]*) clay.* ([0-9]*) ore.* ([0-9]*) obsidian",
    )
    .unwrap();
    lines
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| {
            let r = re.captures_iter(&x).into_iter().next().unwrap();
            let id = r.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let ore1 = r.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let ore2 = r.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let ore3 = r.get(4).unwrap().as_str().parse::<i32>().unwrap();
            let clay = r.get(5).unwrap().as_str().parse::<i32>().unwrap();
            let ore4 = r.get(6).unwrap().as_str().parse::<i32>().unwrap();
            let obsidian = r.get(7).unwrap().as_str().parse::<i32>().unwrap();
            BluePrint {
                id,
                ore_robot_cost: Cost {
                    ore: ore1,
                    clay: 0,
                    obsidian: 0,
                },
                clay_robot_cost: Cost {
                    ore: ore2,
                    clay: 0,
                    obsidian: 0,
                },
                obsidian_robot_cost: Cost {
                    ore: ore3,
                    clay,
                    obsidian: 0,
                },
                geode_robot_cost: Cost {
                    ore: ore4,
                    clay: 0,
                    obsidian,
                },
            }
        })
        .collect()
}

#[derive(Debug, Clone, Hash)]
struct Inventory {
    pub ore_robots: i32,
    pub clay_robots: i32,
    pub obsidian_robots: i32,
    pub geode_robots: i32,
    pub ore: i32,
    pub clay: i32,
    pub obsidian: i32,
    pub geode: i32,
    pub time_left: i32,
}

impl Inventory {
    fn new(minutes: i32) -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            time_left: minutes,
        }
    }
    fn add_new_production(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
        self.time_left -= 1;
    }
}

fn round_up_div(x: i32, y: i32) -> i32 {
    if x < 0 {
        0
    } else if x % y != 0 {
        x / y + 1
    } else {
        x / y
    }
}

fn max_geode(minutes: i32, bp: &BluePrint) -> i32 {
    let mut state = vec![Inventory::new(minutes)];
    let mut max_geode = 0;
    loop {
        let mut branch = Vec::new();
        for v in &state {
            max_geode = max_geode.max(make_inv_branch(v, bp, &mut branch));
        }
        state = branch;
        if state.is_empty() {
            break;
        }
    }
    max_geode
}

fn make_inv_branch(inv: &Inventory, bp: &BluePrint, branch: &mut Vec<Inventory>) -> i32 {
    if inv.time_left <= 0 {
        return inv.geode;
    }

    // check if we are in state to produce each step a new geode robot and calculate the total outcome if so
    let mut max_geode = inv.geode + inv.geode_robots * inv.time_left;
    if inv.ore_robots >= bp.geode_robot_cost.ore
        && inv.obsidian_robots >= bp.geode_robot_cost.obsidian
        && inv.ore >= bp.geode_robot_cost.ore
        && inv.obsidian >= bp.geode_robot_cost.obsidian
    {
        max_geode += inv.time_left * (inv.time_left - 1) / 2;
        return max_geode;
    }

    // Try to build a geode_robot
    if inv.obsidian_robots > 0 {
        let mut inv = inv.clone();
        let steps =
            round_up_div(bp.geode_robot_cost.ore - inv.ore, inv.ore_robots).max(round_up_div(
                bp.geode_robot_cost.obsidian - inv.obsidian,
                inv.obsidian_robots,
            ));
        if steps < inv.time_left {
            for _ in 0..=steps {
                inv.add_new_production();
            }
            inv.ore -= bp.geode_robot_cost.ore;
            inv.obsidian -= bp.geode_robot_cost.obsidian;
            inv.geode_robots += 1;
            branch.push(inv);
        }
    }

    // Try to build a obsidian_robot, but only if we need more
    if inv.clay_robots > 0 && inv.obsidian_robots < bp.geode_robot_cost.obsidian {
        let mut inv = inv.clone();
        let steps = round_up_div(bp.obsidian_robot_cost.ore - inv.ore, inv.ore_robots).max(
            round_up_div(bp.obsidian_robot_cost.clay - inv.clay, inv.clay_robots),
        );
        if steps < inv.time_left {
            for _ in 0..=steps {
                inv.add_new_production();
            }
            inv.ore -= bp.obsidian_robot_cost.ore;
            inv.clay -= bp.obsidian_robot_cost.clay;
            inv.obsidian_robots += 1;
            max_geode = max_geode.max(inv.geode);
            branch.push(inv);
        }
    }

    // Try to build a clay_robot, but only if we need more
    if inv.clay_robots < bp.obsidian_robot_cost.clay {
        let mut inv = inv.clone();
        let steps = round_up_div(bp.clay_robot_cost.ore - inv.ore, inv.ore_robots);
        if steps < inv.time_left {
            for _ in 0..=steps {
                inv.add_new_production();
            }
            inv.ore -= bp.clay_robot_cost.ore;
            inv.clay_robots += 1;
            max_geode = max_geode.max(inv.geode);
            branch.push(inv);
        }
    }

    // Try to build a ore_robot, but only if we need more
    if inv.ore_robots < bp.max_ore() {
        let mut inv = inv.clone();
        let steps = round_up_div(bp.ore_robot_cost.ore - inv.ore, inv.ore_robots);
        if steps < inv.time_left {
            for _ in 0..=steps {
                inv.add_new_production();
            }
            inv.ore -= bp.ore_robot_cost.ore;
            inv.ore_robots += 1;
            max_geode = max_geode.max(inv.geode);
            branch.push(inv);
        }
    }
    return max_geode;
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let blue_prints = read_blueprints(lines);

    let mut quality_level = 0;
    for bp in &blue_prints {
        let ql = bp.id * max_geode(24, bp);
        quality_level += ql;
    }
    format!("{quality_level}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let blue_prints = read_blueprints(lines);

    let mut total_score = 1;
    for bp in &blue_prints[0..3] {
        let score = max_geode(32, bp);
        total_score *= score;
    }
    format!("{total_score}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2022_19_1() {
        let lines = read_lines("data/2022/19.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "1766");
    }

    #[test]
    fn test_2022_19_2() {
        let lines = read_lines("data/2022/19.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "30780");
    }
}
