use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, Clone)]
enum Entity {
    Dir(Vec<usize>),
    File,
}

#[derive(Debug)]
struct Node {
    entity: Entity,
    name: String,
    size: Option<usize>,
    parent: Option<usize>,
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
    ListItem(Entity, String, Option<usize>),
}

fn add_node(files: &mut Vec<Node>, node: Node, dir_idx: usize) {
    let idx = files.len();
    files.push(node);
    match &mut files[dir_idx].entity {
        Entity::Dir(v) => v.push(idx),
        _ => panic!("Tried to add entity to none-dir"),
    }
}

fn parse_command(s: &str) -> Command {
    let mut parts = s.split(" ");
    match parts.next().unwrap() {
        "$" => match parts.next().unwrap() {
            "cd" => {
                let name = parts.next().unwrap().to_string();
                Command::Cd(name)
            }
            "ls" => Command::Ls,
            cmd => panic!("unknown command '{cmd}'"),
        },
        first_str => {
            let name = parts.next().unwrap().to_string();
            if let Ok(num) = first_str.parse() {
                Command::ListItem(Entity::File, name, Some(num))
            } else {
                Command::ListItem(Entity::Dir(Vec::new()), name, None)
            }
        }
    }
}

fn calc_dir_sizes(node_idx: usize, files: &mut Vec<Node>, total_sum: usize) -> (usize, usize) {
    match files[node_idx].entity.clone() {
        Entity::File => (files[node_idx].size.unwrap(), total_sum),
        Entity::Dir(v) => {
            if let Some(size) = files[node_idx].size {
                (size, total_sum)
            } else {
                let mut sum = 0;
                let mut total = total_sum;
                for idx in v {
                    let size_and_total = calc_dir_sizes(idx, files, total);
                    sum += size_and_total.0;
                    total = size_and_total.1;
                }
                files[node_idx].size = Some(sum);
                if sum < 100000 {
                    (sum, sum + total)
                } else {
                    (sum, total)
                }
            }
        }
    }
}

fn parse_file_structure(lines: io::Lines<io::BufReader<File>>) -> Vec<Node> {
    let lines: Vec<String> = lines.into_iter().map(|x| x.unwrap()).collect();
    let mut files = Vec::new();
    files.push(Node {
        entity: Entity::Dir(Vec::new()),
        name: "/".to_string(),
        size: None,
        parent: None,
    });
    let mut dir_idx = 0;
    for line in lines {
        match parse_command(&line) {
            Command::Cd(name) => match name.as_str() {
                "/" => {
                    dir_idx = 0;
                }
                ".." => {
                    dir_idx = files[dir_idx].parent.unwrap();
                }
                dir => match &files[dir_idx].entity {
                    Entity::Dir(entities) => {
                        for e in entities {
                            if files[*e].name == dir {
                                dir_idx = *e;
                            }
                        }
                    }
                    _ => panic!("{dir_idx} is no directory"),
                },
            },
            Command::Ls => {}
            Command::ListItem(entity, name, size) => add_node(
                &mut files,
                Node {
                    entity,
                    name,
                    size,
                    parent: Some(dir_idx),
                },
                dir_idx,
            ),
        }
    }

    files
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut files = parse_file_structure(lines);
    let (_size, total_sum) = calc_dir_sizes(0, &mut files, 0);
    format!("{total_sum}")
}

fn find_smallest_dir_size(files: &Vec<Node>, required_space: usize) -> usize {
    let mut smallest_dir_size = files[0].size.unwrap();
    for i in 1..files.len() {
        match &files[i].entity {
            Entity::File => {}
            Entity::Dir(_) => {
                let dir_size = files[i].size.unwrap();
                if dir_size >= required_space {
                    smallest_dir_size = smallest_dir_size.min(dir_size);
                }
            }
        }
    }
    smallest_dir_size
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let total_size = 70000000;
    let required_space = 30000000;
    let mut files = parse_file_structure(lines);
    let (size, _total_sum) = calc_dir_sizes(0, &mut files, 0);
    let to_be_removed = required_space - (total_size - size);

    let smallest_dir_size = find_smallest_dir_size(&files, to_be_removed);
    format!("{smallest_dir_size}")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2022_7_1() {
        let lines = read_lines("data/2022/7.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "7195");
    }

    #[test]
    fn test_2022_7_2() {
        let lines = read_lines("data/2022/7.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "33992866292225");
    }
}

