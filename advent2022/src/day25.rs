use super::*;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
enum Digit {
    Two,
    One,
    Zero,
    Minus,
    Minus2,
}

#[derive(Debug)]
enum DigitError {
    InvalidDigit,
}

impl FromStr for Digit {
    type Err = DigitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Digit::Two),
            "1" => Ok(Digit::One),
            "0" => Ok(Digit::Zero),
            "-" => Ok(Digit::Minus),
            "=" => Ok(Digit::Minus2),
            _ => Err(DigitError::InvalidDigit),
        }
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Digit::Two => write!(f, "2"),
            Digit::One => write!(f, "1"),
            Digit::Zero => write!(f, "0"),
            Digit::Minus => write!(f, "-"),
            Digit::Minus2 => write!(f, "="),
        }
    }
}

#[derive(Debug, Clone)]
struct Snafu {
    num: Vec<Digit>,
}

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for d in self.num.iter().rev() {
            write!(f, "{}", d)?;
        }
        Ok(())
    }
}

impl FromStr for Snafu {
    type Err = DigitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut snafu = Self { num: Vec::new() };
        for i in (0..s.len()).rev() {
            snafu.num.push(s[i..=i].parse::<Digit>()?);
        }
        Ok(snafu)
    }
}

impl From<&Snafu> for i64 {
    fn from(s: &Snafu) -> i64 {
        let mut num = 0;
        for d in s.num.iter().rev() {
            num *= 5;
            num += match d {
                Digit::Two => 2,
                Digit::One => 1,
                Digit::Zero => 0,
                Digit::Minus => -1,
                Digit::Minus2 => -2,
            };
        }
        num
    }
}

impl From<i64> for Snafu {
    fn from(mut x: i64) -> Snafu {
        let mut s = Vec::new();
        let mut r = false;
        while x > 0 {
            let digit = match x % 5 {
                2 => {
                    if r {
                        r = true;
                        Digit::Minus2
                    } else {
                        r = false;
                        Digit::Two
                    }
                }
                1 => {
                    if r {
                        r = false;
                        Digit::Two
                    } else {
                        r = false;
                        Digit::One
                    }
                }
                0 => {
                    if r {
                        r = false;
                        Digit::One
                    } else {
                        r = false;
                        Digit::Zero
                    }
                }
                4 => {
                    if r {
                        r = true;
                        Digit::Zero
                    } else {
                        r = true;
                        Digit::Minus
                    }
                }
                3 => {
                    if r {
                        r = true;
                        Digit::Minus
                    } else {
                        r = true;
                        Digit::Minus2
                    }
                }
                _ => panic!("invalid branch"),
            };
            x /= 5;
            s.push(digit);
        }
        if r {
            s.push(Digit::One)
        }
        if s.is_empty() {
            s.push(Digit::Zero);
        }
        Snafu { num: s }
    }
}

fn read_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<i64> {
    lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(|s| s.parse::<Snafu>().unwrap())
        .map(|s| (&s).into())
        .collect()
}

pub fn riddle_25_1(lines: io::Lines<io::BufReader<File>>) {
    let numbers = read_numbers(lines);
    let sum: i64 = numbers.iter().sum();
    let snafu_sum: Snafu = sum.into();
    println!("Solution: {}", snafu_sum);
}
