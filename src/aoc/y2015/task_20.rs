use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn run() {
    let input = File::open("src/aoc/y2015/task_20").unwrap();
    let input = BufReader::new(input);

    let target = input.lines().into_iter().filter_map(|l| l.ok()).next().unwrap().parse::<usize>().unwrap();

    let mut result: usize = 0;
    let mut i: usize = 0;
    while result < target / 10 {
        i += 1;
        result = number_of_presents(i);
    }
    println!("Result: {}", i);
}

fn number_of_presents(number: usize) -> usize {
    let mut result = 0;
    if number == 1 {
        return 1;
    }
    let mut i = 1;
    let mut s: Vec<(usize, usize)> = vec![];
    while i * i <= number && i <= number / 2 {
        if number % i == 0 {
            s.push((number / i, i));
        }
        i += 1;
    }
    for i in s {
        if i.0 == i.1 {
            result += i.0
        } else {
            result += i.0 + i.1;
        }
    }
    return result;
}


pub fn run_e() {
    let input = File::open("src/aoc/y2015/task_20").unwrap();
    let input = BufReader::new(input);
}