use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::{Add, AddAssign};
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Eq)]
struct Container(usize);

impl Add for Container {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Container(self.0 + rhs.0)
    }
}

impl Add<usize> for Container {
    type Output = usize;

    fn add(self, rhs: usize) -> Self::Output {
        self.0 + rhs
    }
}

impl Add<&Container> for usize {
    type Output = usize;

    fn add(self, rhs: &Container) -> Self::Output {
        self + rhs.0
    }
}

impl AddAssign<Container> for usize {
    fn add_assign(&mut self, rhs: Container) {
        *self += rhs.0
    }
}

impl FromStr for Container {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> { s.parse::<usize>().map(|c| Container(c)) }
}

impl PartialEq for Container {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<usize> for Container {
    fn eq(&self, other: &usize) -> bool {
        &self.0 == other
    }
}

pub fn run() {
    let input = File::open("src/aoc/y2015/task_17").unwrap();
    let input = BufReader::new(input);

    let input: Vec<Container> = input.lines()
        .filter_map(|l| l.ok())
        .map(|l| l.parse::<Container>())
        .filter_map(|c| c.ok())
        .collect();

    let result = iterate(150, &input, 0, 0);

    println!("Result: {}", result);
}

fn iterate(target: usize, containers: &Vec<Container>, current: usize, current_sum: usize) -> usize {
    let mut result = 0;
    if current_sum < target {
        for i in current..containers.len() {
            let sum = current_sum + &containers[i];
            if sum == target {
                result += 1;
            } else if sum < target {
                result += iterate(target, containers, i + 1, sum);
            }
        }
    }
    return result;
}

pub fn run_e() {
    let input = File::open("src/aoc/y2015/task_17").unwrap();
    let input = BufReader::new(input);
}