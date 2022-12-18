use super::*;

pub fn read_map(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<Vec<u8>>> {
    let mut dim = (0,0,0);
    let points: Vec<(usize,usize,usize)> = lines
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|s| {
            let mut parts = s.split(",");
            let x = parts.next().unwrap().parse::<usize>().unwrap()+1;
            let y = parts.next().unwrap().parse::<usize>().unwrap()+1;
            let z = parts.next().unwrap().parse::<usize>().unwrap()+1;
            dim.0 = dim.0.max(x+2);
            dim.1 = dim.1.max(y+2);
            dim.2 = dim.2.max(z+2);
            (x,y,z)
        })
        .collect();
    let mut droplet = Vec::new();
    for _ in 0..dim.0 {
        let mut plane = Vec::new();
        for _ in 0..dim.1 {
            plane.push(vec![1_u8; dim.2]);
        }
        droplet.push(plane);
    }
    for point in points {
        droplet[point.0][point.1][point.2] = 0_u8;
    }
    droplet
}

fn count_surfaces(x: usize, y: usize, z: usize, map: &Vec<Vec<Vec<u8>>>) -> usize {
    (map[x][y][z-1] + map[x][y][z+1]
        + map[x][y-1][z] + map[x][y+1][z]
        + map[x-1][y][z] + map[x+1][y][z]) as usize
}

pub fn riddle_18_1(lines: io::Lines<io::BufReader<File>>) {
    let map = read_map(lines);
    let mut sum = 0;
    for x in 1..map.len()-1 {
        for y in 1..map[0].len()-1 {
            for z in 1..map[0][0].len()-1 {
                if map[x][y][z] == 0 {
                    sum += count_surfaces(x,y,z, &map)
                }
            }
        }
    }

    println!("The solution is: {:?}", sum);
}

fn count_surfaces2(x: i64, y: i64, z: i64, map: &Vec<Vec<Vec<u8>>>) -> usize {
    let surfaces = vec![(0,0,-1), (0,0,1), (0,-1,0), (0,1,0), (-1,0,0), (1,0,0)];
    let mut count: usize = 0;

    for s in surfaces {
        if map[(x+s.0) as usize][(y+s.1) as usize][(z+s.2) as usize] == 2 {
            count += 1;
        }
    }
    count
}

pub fn mark_outside(map: &mut Vec<Vec<Vec<u8>>>, points: Vec<(i64, i64, i64)>) {
    let mut points = points;
    let surfaces = vec![(0,0,-1), (0,0,1), (0,-1,0), (0,1,0), (-1,0,0), (1,0,0)];

    let max = (map.len()-1, map[0].len()-1, map[0][0].len()-1);
    for y in 0..=max.1 {
        for z in 0..max.2 {
            map[0][y][z] = 2;
            map[max.0][y][z] = 2;
        }
    }
    for x in 0..=max.0 {
        for z in 0..max.2 {
            map[x][0][z] = 2;
            map[x][max.1][z] = 2;
        }
    }
    for x in 0..=max.0 {
        for y in 0..max.1 {
            map[x][y][0] = 2;
            map[x][y][max.2] = 2;
        }
    }

    loop {
        if points.is_empty() {
            break;
        }
        let mut new_points = Vec::new();
        for p in points {
            for s in &surfaces {
                let sp = (p.0+s.0,p.1+s.1,p.2+s.2);
                if map[sp.0 as usize][sp.1 as usize][sp.2 as usize] == 1 {
                    map[sp.0 as usize][sp.1 as usize][sp.2 as usize] = 2;
                    new_points.push(sp);
                }
            }
        }
        points = new_points;
    }
}

pub fn riddle_18_2(lines: io::Lines<io::BufReader<File>>) {
    let mut map = read_map(lines);
    let mut sum = 0;
    let next_points = vec![(1,1,1)];
    mark_outside(&mut map, next_points);
    for x in 1..map.len()-1 {
        for y in 1..map[0].len()-1 {
            for z in 1..map[0][0].len()-1 {
                if map[x][y][z] == 0 {
                    sum += count_surfaces2(x as i64,y as i64,z as i64, &map)
                }
            }
        }
    }

    println!("The solution is: {:?}", sum);
}
