use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use itertools::Itertools;
use std::env::var;

#[derive(Debug)]
struct Replacement {
    from: String,
    to: String,
}

impl FromStr for Replacement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split(" => ").collect();
        Result::Ok(Replacement { from: s[0].to_string(), to: s[1].to_string() })
    }
}

pub fn run() {
    let input = File::open("src/aoc/y2015/task_19").unwrap();
    let input = BufReader::new(input);

    let input: Vec<String> = input.lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()).collect();

    let mut input = input;
    let target = input.pop().unwrap().clone();

    let input: Vec<Replacement> = input.iter().filter_map(|l| l.parse::<Replacement>().ok()).collect();

    let mut result: Vec<String> = vec![];

    let size = target.len();
    for i in 0..size {
        for j in 0..input.len() {
            if size - i >= input[j].from.len() && target[i..].starts_with(input[j].from.as_str()) {
                result.push(format!("{}{}{}", &target[..i], input[j].to, &target[i + input[j].from.len()..]));
            }
        }
    }

    result.sort();
    let result = result.into_iter().dedup().collect::<Vec<String>>();

    println!("Result: {}", result.len());
}

pub fn run_e() {
    let input = File::open("src/aoc/y2015/task_19").unwrap();
    let input = BufReader::new(input);

    let input: Vec<String> = input.lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()).collect();

    let mut input = input;
    let target = input.pop().unwrap().clone();

    let input: Vec<Replacement> = input.iter().filter_map(|l| l.parse::<Replacement>().ok()).collect();

    let result = iterate(target.as_str(), &input, 0, 0);

    println!("Result: {}", result.unwrap());
}

fn iterate(target: &str, variants: &Vec<Replacement>, count: usize, next: usize) -> Option<usize> {
    for i in 0..variants.len() {
        let mut iter = variants.iter().cycle();
        let mut target = target.to_string();
        let mut result = 0;
        let mut not_changed_for = 0;
        loop {
            if target == "e" {
                break;
            }
            if not_changed_for == variants.len() {
                break;
            }
            let next = iter.next().unwrap();
            let new_target = target.replacen(&next.to, &next.from, 1);
            if new_target != target {
                result += 1;
                target = new_target;
                println!("{}: {}", result, target);
                not_changed_for = 0;
            } else {
                not_changed_for += 1;
            }
        }
        println!("{}", i);
    }
    return None;
}
