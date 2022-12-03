use super::*;
use bit_range::BitRange;

fn read_message(lines: io::Lines<io::BufReader<File>>) ->Vec<u8> {
    let message = lines.into_iter()
        .filter_map(|x| x.ok())
        .next()
        .unwrap();
    let message: Vec<char> = message.chars().collect();
    message
        .chunks(2)
        .map(|x| u8::from_str_radix(&x.iter().collect::<String>(), 16).unwrap() )
        .collect()
}

#[derive(Debug)]
enum PackageValue {
    Number(usize),
    PackageList(Vec<Package>),
}

#[derive(Debug)]
struct Package {
    version: u8,
    operation: u8,
    value: PackageValue,
}

fn read_bits(msg: &[u8], p: &mut u32, bits: u32) -> u32 {
    let val = msg.get_bit_range(*p..*p+bits);
    *p += bits;
    val
}

fn read_literal(msg: &[u8], p: &mut u32) -> usize {
    let mut num = 0;
    loop {
        let last_bits = read_bits(msg, p, 1) == 0;
        num <<= 4;
        num += read_bits(msg, p, 4) as usize;
        if last_bits {
            break;
        }
    }
    num
}

fn read_package_list(msg: &[u8], p: &mut u32) -> Vec<Package> {
    let is_bit_size = read_bits(msg, p, 1) == 0;
    let mut bit_size = 0;
    let mut num_packages = 0;
    if is_bit_size {
        bit_size = read_bits(msg, p, 15) as u32;
    } else {
        num_packages = read_bits(msg, p, 11) as usize;
    }

    let mut packages = Vec::new();
    let start_pointer = *p;
    loop {
        packages.push(parse_package(msg, p));
        if (is_bit_size && *p-start_pointer == bit_size) || (!is_bit_size && packages.len() == num_packages) {
            break;
        }
    }
    packages
}

fn parse_package(msg: &[u8], p: &mut u32) -> Package {
    let version = read_bits(msg, p, 3) as u8;
    let operation = read_bits(msg, p, 3) as u8;
    if operation == 4 {
        let value = PackageValue::Number(read_literal(msg, p));
        Package{
            version,
            operation,
            value,
        }
    } else {
        let value = PackageValue::PackageList(read_package_list(msg, p));
        Package {
            version,
            operation,
            value,
        }
    }
}

fn sum_versions(package: &Package) -> usize {
    let mut version = package.version as usize;
    match &package.value {
        PackageValue::Number(_) => version,
        PackageValue::PackageList(v) => {
            for p in v {
                version += sum_versions(&p);
            }
            version
        }
    }
}

pub fn riddle_16_1(lines: io::Lines<io::BufReader<File>>) {
    let message = read_message(lines);
    let mut p = 0;
    let package = parse_package(&message, &mut p);
    let version_sum = sum_versions(&package);
    println!("Sum of versions: {}", version_sum);
}

fn calc_value(package: &Package) -> usize {
    match package.operation {
        4 => match package.value {
            PackageValue::Number(x) => x,
            _ => panic!("Value expected!")
        },
        0 => match &package.value {
            PackageValue::Number(_) => panic!("Package list expected!"),
            PackageValue::PackageList(v) => {
                let mut value = 0;
                for p in v {
                    value += calc_value(&p);
                }
                value
            }
        },
        1 => match &package.value {
            PackageValue::Number(_) => panic!("Package list expected!"),
            PackageValue::PackageList(v) => {
                let mut value = 1;
                for p in v {
                    value *= calc_value(&p);
                }
                value
            }
        },
        2 => match &package.value {
            PackageValue::Number(_) => panic!("Package list expected!"),
            PackageValue::PackageList(v) => {
                let mut value = usize::MAX;
                for p in v {
                    value = value.min(calc_value(&p));
                }
                value
            }
        },
        3 => match &package.value {
            PackageValue::Number(_) => panic!("Package list expected!"),
            PackageValue::PackageList(v) => {
                let mut value = 0;
                for p in v {
                    value = value.max(calc_value(&p));
                }
                value
            }
        },
        5 => match &package.value {
            PackageValue::Number(_) => panic!("Package list expected!"),
            PackageValue::PackageList(v) => {
                if calc_value(&v[0]) > calc_value(&v[1]) {
                    1
                } else {
                    0
                }
            }
        },
        6 => match &package.value {
            PackageValue::Number(_) => panic!("Package list expected!"),
            PackageValue::PackageList(v) => {
                if calc_value(&v[0]) < calc_value(&v[1]) {
                    1
                } else {
                    0
                }
            }
        },
        7 => match &package.value {
            PackageValue::Number(_) => panic!("Package list expected!"),
            PackageValue::PackageList(v) => {
                if calc_value(&v[0]) == calc_value(&v[1]) {
                    1
                } else {
                    0
                }
            }
        },
        _ => panic!("Invalid opeation")
    }
}

pub fn riddle_16_2(lines: io::Lines<io::BufReader<File>>) {
    let message = read_message(lines);
    let mut p = 0;
    let package = parse_package(&message, &mut p);
    let value = calc_value(&package);
    println!("Result value: {}", value);
}
