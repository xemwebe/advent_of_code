use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, Eq)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

impl PartialOrd for Beacon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Beacon {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Ord for Beacon {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x == other.x {
            if self.y == other.y {
                self.z.cmp(&other.z)
            } else {
                self.y.cmp(&other.y)
            }
        } else {
            self.x.cmp(&other.x)
        }
    }
}

impl Beacon {
    fn dist(&self, other: &Beacon) -> i32 {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }

    fn transform(&self, t: &Transformation) -> Self {
        Beacon {
            x: t.xx * self.x + t.xy * self.y + t.xz * self.z + t.tx,
            y: t.yx * self.x + t.yy * self.y + t.yz * self.z + t.ty,
            z: t.zx * self.x + t.zy * self.y + t.zz * self.z + t.tz,
        }
    }
}

#[derive(Debug)]
enum MiniSet {
    None,
    One(usize),
    Two(usize, usize),
}

impl MiniSet {
    fn join(&self, other: (usize, usize)) -> Self {
        match *self {
            MiniSet::None => MiniSet::None,
            MiniSet::One(x) => {
                if x == other.0 || x == other.1 {
                    MiniSet::One(x)
                } else {
                    MiniSet::None
                }
            }
            MiniSet::Two(x, y) => {
                if x == other.0 || x == other.1 {
                    MiniSet::One(x)
                } else if y == other.0 || y == other.1 {
                    MiniSet::One(y)
                } else {
                    MiniSet::None
                }
            }
        }
    }
}

fn read_scanner_data(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<Beacon>> {
    let mut scanners = Vec::new();
    let mut beacons = Vec::new();
    for l in lines.into_iter().filter_map(|x| x.ok()).skip(1) {
        if l.is_empty() {
            continue;
        }
        if l.starts_with("---") {
            scanners.push(beacons);
            beacons = Vec::new();
            continue;
        }
        let beacon: Vec<i32> = l
            .split(",")
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        beacons.push(Beacon {
            x: beacon[0],
            y: beacon[1],
            z: beacon[2],
        });
    }
    scanners.push(beacons);
    scanners
}

fn calc_distances(scanners: &Vec<Vec<Beacon>>) -> Vec<HashMap<i32, (usize, usize)>> {
    let mut distances = Vec::new();
    for s in scanners {
        let mut distance_map = HashMap::new();
        let n = s.len();
        for i in 0..n {
            for j in i + 1..n {
                let distance = s[i].dist(&s[j]);
                distance_map.insert(distance, (i, j));
            }
        }
        distances.push(distance_map);
    }
    distances
}

fn find_matches(
    distances: &Vec<HashMap<i32, (usize, usize)>>,
    first: usize,
    second: usize,
) -> HashMap<usize, usize> {
    let mut matches: HashMap<usize, MiniSet> = HashMap::new();
    for k in distances[first].keys() {
        if distances[second].contains_key(k) {
            let (i, j) = distances[first][k];
            if matches.contains_key(&i) {
                *matches.get_mut(&i).unwrap() = matches[&i].join(distances[second][k]);
            } else {
                matches.insert(
                    i,
                    MiniSet::Two(distances[second][k].0, distances[second][k].1),
                );
            }
            if matches.contains_key(&j) {
                *matches.get_mut(&j).unwrap() = matches[&j].join(distances[second][k]);
            } else {
                matches.insert(
                    j,
                    MiniSet::Two(distances[second][k].0, distances[second][k].1),
                );
            }
        }
    }
    if matches.len() < 12 {
        return HashMap::new();
    }

    matches
        .into_iter()
        .map(|(k, x)| match x {
            MiniSet::One(num) => (k, num),
            _ => panic!("Invalid miniset"),
        })
        .collect()
}

#[derive(Debug)]
struct Transformation {
    xx: i32,
    xy: i32,
    xz: i32,
    yx: i32,
    yy: i32,
    yz: i32,
    zx: i32,
    zy: i32,
    zz: i32,
    tx: i32,
    ty: i32,
    tz: i32,
}

impl Transformation {
    fn new() -> Self {
        Transformation {
            xx: 1,
            xy: 0,
            xz: 0,
            yx: 0,
            yy: 1,
            yz: 0,
            zx: 0,
            zy: 0,
            zz: 1,
            tx: 0,
            ty: 0,
            tz: 0,
        }
    }

    fn transform(&self, other: &Self) -> Self {
        Transformation {
            xx: other.xx * self.xx + other.xy * self.yx + other.xz * self.zx,
            xy: other.xx * self.xy + other.xy * self.yy + other.xz * self.zy,
            xz: other.xx * self.xz + other.xy * self.yz + other.xz * self.zz,
            yx: other.yx * self.xx + other.yy * self.yx + other.yz * self.zx,
            yy: other.yx * self.xy + other.yy * self.yy + other.yz * self.zy,
            yz: other.yx * self.xz + other.yy * self.yz + other.yz * self.zz,
            zx: other.zx * self.xx + other.zy * self.yx + other.zz * self.zx,
            zy: other.zx * self.xy + other.zy * self.yy + other.zz * self.zy,
            zz: other.zx * self.xz + other.zy * self.yz + other.zz * self.zz,
            tx: other.xx * self.tx + other.xy * self.ty + other.xz * self.tz + other.tx,
            ty: other.yx * self.tx + other.yy * self.ty + other.yz * self.tz + other.ty,
            tz: other.zx * self.tx + other.zy * self.ty + other.zz * self.tz + other.tz,
        }
    }
}
fn calc_trafo(
    scanners: &Vec<Vec<Beacon>>,
    matches: &HashMap<usize, usize>,
    system1: usize,
    system2: usize,
) -> Transformation {
    let scanner1 = &scanners[system1];
    let mut diff1_x = 0;
    let mut diff1_y = 0;
    let mut diff1_z = 0;
    let mut first = 0;
    let mut second = 0;
    let mut first_found = false;
    // Find pair of Beacon with unique differences in system1
    for j in matches.keys() {
        if !first_found {
            first = *j;
            first_found = true;
        } else {
            diff1_x = scanner1[first].x - scanner1[*j].x;
            diff1_y = scanner1[first].y - scanner1[*j].y;
            diff1_z = scanner1[first].z - scanner1[*j].z;
            if diff1_x.abs() != diff1_y.abs()
                && diff1_x.abs() != diff1_z.abs()
                && diff1_y.abs() != diff1_z.abs()
            {
                second = *j;
                break;
            }
        }
    }

    // calculate difference of same pair in system2
    let scanner2 = &scanners[system2];
    let first2 = matches[&first];
    let second2 = matches[&second];
    let diff2_x = scanner2[first2].x - scanner2[second2].x;
    let diff2_y = scanner2[first2].y - scanner2[second2].y;
    let diff2_z = scanner2[first2].z - scanner2[second2].z;

    let mut trafo = Transformation::new();
    if diff1_x.abs() == diff2_x.abs() {
        if diff1_x != diff2_x {
            trafo.xx = -1;
        }
    } else if diff1_x.abs() == diff2_y.abs() {
        trafo.xx = 0;
        if diff1_x != diff2_y {
            trafo.xy = -1;
        } else {
            trafo.xy = 1;
        }
    } else if diff1_x.abs() == diff2_z.abs() {
        trafo.xx = 0;
        if diff1_x != diff2_z {
            trafo.xz = -1;
        } else {
            trafo.xz = 1;
        }
    }
    if diff1_y.abs() == diff2_y.abs() {
        if diff1_y != diff2_y {
            trafo.yy = -1;
        }
    } else if diff1_y.abs() == diff2_x.abs() {
        trafo.yy = 0;
        if diff1_y != diff2_x {
            trafo.yx = -1;
        } else {
            trafo.yx = 1;
        }
    } else if diff1_y.abs() == diff2_z.abs() {
        trafo.yy = 0;
        if diff1_y != diff2_z {
            trafo.yz = -1;
        } else {
            trafo.yz = 1;
        }
    }
    if diff1_z.abs() == diff2_z.abs() {
        if diff1_z != diff2_z {
            trafo.zz = -1;
        }
    } else if diff1_z.abs() == diff2_x.abs() {
        trafo.zz = 0;
        if diff1_z != diff2_x {
            trafo.zx = -1;
        } else {
            trafo.zx = 1;
        }
    } else if diff1_z.abs() == diff2_y.abs() {
        trafo.zz = 0;
        if diff1_z != diff2_y {
            trafo.zy = -1;
        } else {
            trafo.zy = 1;
        }
    }

    // calculate translation
    let rotated = scanner2[first2].transform(&trafo);
    trafo.tx = scanner1[first].x - rotated.x;
    trafo.ty = scanner1[first].y - rotated.y;
    trafo.tz = scanner1[first].z - rotated.z;
    trafo
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>)  -> String {
    let scanners = read_scanner_data(lines);
    let distances = calc_distances(&scanners);
    let mut all_beacons = BTreeSet::new();
    for b in scanners[0].iter() {
        all_beacons.insert(b.clone());
    }
    let mut next_scanners: Vec<usize> = vec![0; 1];
    let mut trafos = HashMap::new();
    trafos.insert(0, Transformation::new());
    loop {
        let test_scanners = next_scanners.clone();
        next_scanners = Vec::new();
        for s in test_scanners {
            for j in 1..scanners.len() {
                if !trafos.contains_key(&j) {
                    let matches = find_matches(&distances, s, j);
                    if matches.len() >= 12 {
                        let mut trafo = calc_trafo(&scanners, &matches, s, j);
                        if s != 0 {
                            trafo = trafo.transform(&trafos[&s]);
                        }
                        trafos.insert(j, trafo);
                        next_scanners.push(j);
                    }
                }
            }
        }
        if next_scanners.is_empty() || trafos.len() == scanners.len() {
            break;
        }
    }
    for j in 1..scanners.len() {
        for b in scanners[j].iter() {
            all_beacons.insert(b.transform(&trafos[&j]));
        }
    }
    format!("{}", all_beacons.len())
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>)  -> String {
    let scanners = read_scanner_data(lines);
    let distances = calc_distances(&scanners);
    let mut all_beacons = BTreeSet::new();
    for b in scanners[0].iter() {
        all_beacons.insert(b.clone());
    }
    let mut next_scanners: Vec<usize> = vec![0; 1];
    let mut trafos = HashMap::new();
    trafos.insert(0, Transformation::new());
    loop {
        let test_scanners = next_scanners.clone();
        next_scanners = Vec::new();
        for s in test_scanners {
            for j in 1..scanners.len() {
                if !trafos.contains_key(&j) {
                    let matches = find_matches(&distances, s, j);
                    if matches.len() >= 12 {
                        let mut trafo = calc_trafo(&scanners, &matches, s, j);
                        if s != 0 {
                            trafo = trafo.transform(&trafos[&s]);
                        }
                        trafos.insert(j, trafo);
                        next_scanners.push(j);
                    }
                }
            }
        }
        if next_scanners.is_empty() || trafos.len() == scanners.len() {
            break;
        }
    }

    // calculate largest manhattan distance
    let mut max_dist = 0;
    for i in 0..trafos.len() {
        for j in i..trafos.len() {
            let distance = (trafos[&i].tx - trafos[&j].tx).abs()
                + (trafos[&i].ty - trafos[&j].ty).abs()
                + (trafos[&i].tz - trafos[&j].tz).abs();
            max_dist = max_dist.max(distance);
        }
    }
    format!("{max_dist}")
}
