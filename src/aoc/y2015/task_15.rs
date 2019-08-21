use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;
use std::ops::{Range, RangeInclusive};

#[derive(Eq, Hash, PartialEq, Clone)]
struct Properties {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Properties {
    fn new(name: &str, capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self {
        Properties { name: name.clone().to_string(), capacity, durability, flavor, texture, calories }
    }

    fn evaluate(&self, x: i32) -> (i32, i32, i32, i32) { (x * self.capacity, x * self.durability, x * self.flavor, x * self.texture) }
}

impl FromStr for Properties {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split(": ").collect();
        let name = s[0];
        let p: Vec<&str> = s[1].split(", ").collect();

        let capacity = p[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let durability = p[1].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let flavor = p[2].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let texture = p[3].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let calories = p[4].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

        Ok(Properties::new(name, capacity, durability, flavor, texture, calories))
    }
}

pub fn run<'a>() {
    let input = File::open("src/aoc/y2015/task_15").unwrap();
    let input = BufReader::new(input);

    let input: Vec<Properties> = input
        .lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Properties>().ok())
        .collect();

    let target = 100;

    let mut multiplyers = vec![(0_f32, 0_f32, 0_f32, 0_f32); input.len()];

    for i in 0..4 {
        let (pos, neg): (f32, f32) = if i == 0 {
            let pos: i32 = input.iter().filter_map(|p| if p.capacity >= 0 { Some(p.capacity) } else { None }).sum();
            let neg: i32 = input.iter().filter_map(|p| if p.capacity < 0 { Some(p.capacity) } else { None }).sum();
            (pos as f32, neg as f32)
        } else if i == 1 {
            let pos: i32 = input.iter().filter_map(|p| if p.durability >= 0 { Some(p.durability) } else { None }).sum();
            let neg: i32 = input.iter().filter_map(|p| if p.durability < 0 { Some(p.durability) } else { None }).sum();
            (pos as f32, neg as f32)
        } else if i == 2 {
            let pos: i32 = input.iter().filter_map(|p| if p.flavor >= 0 { Some(p.flavor) } else { None }).sum();
            let neg: i32 = input.iter().filter_map(|p| if p.flavor < 0 { Some(p.flavor) } else { None }).sum();
            (pos as f32, neg as f32)
        } else if i == 3 {
            let pos: i32 = input.iter().filter_map(|p| if p.texture >= 0 { Some(p.texture) } else { None }).sum();
            let neg: i32 = input.iter().filter_map(|p| if p.texture < 0 { Some(p.texture) } else { None }).sum();
            (pos as f32, neg as f32)
        } else {
            (0f32, 0f32)
        };
        for j in 0..input.len() {
            if i == 0 {
                let t = if input[j].capacity == 0 {
                    0f32
                } else if input[j].capacity > 0 {
                    let mltpl = if pos == input[j].capacity as f32 { 1_f32 } else { (pos - input[j].capacity as f32) / pos };
                    let mltpl = mltpl * if neg == 0_f32 { input[j].capacity as f32 } else { neg.abs() };
                    mltpl / (neg.abs() + pos)
                } else {
                    let mltpl = if neg == input[j].capacity as f32 { 1_f32 } else { (neg - input[j].capacity as f32) / neg };
                    let mltpl = mltpl * if pos == 0_f32 { input[j].capacity as f32 } else { pos.abs() };
                    mltpl / (neg.abs() + pos)
                };
                multiplyers[j].0 = t;
            } else if i == 1 {
                let t = if input[j].durability == 0 {
                    0f32
                } else if input[j].durability > 0 {
                    let mltpl = if pos == input[j].durability as f32 { 1_f32 } else { (pos - input[j].durability as f32) / pos };
                    let mltpl = mltpl * if neg == 0_f32 { input[j].durability as f32 } else { neg.abs() };
                    mltpl / (neg.abs() + pos)
                } else {
                    let mltpl = if neg == input[j].durability as f32 { 1_f32 } else { (neg - input[j].durability as f32) / neg };
                    let mltpl = mltpl * if pos == 0_f32 { input[j].durability as f32 } else { pos.abs() };
                    mltpl / (neg.abs() + pos)
                };
                multiplyers[j].1 = t;
            } else if i == 2 {
                let t = if input[j].flavor == 0 {
                    0f32
                } else if input[j].flavor > 0 {
                    let mltpl = if pos == input[j].flavor as f32 { 1_f32 } else { (pos - input[j].flavor as f32) / pos };
                    mltpl * neg.abs() / (neg.abs() + pos)
                } else {
                    let mltpl = if neg == input[j].flavor as f32 { 1_f32 } else { (neg - input[j].flavor as f32) / neg };
                    mltpl * pos / (neg.abs() + pos)
                };
                multiplyers[j].2 = t;
            } else if i == 3 {
                let t = if input[j].texture == 0 {
                    0f32
                } else if input[j].texture > 0 {
                    let mltpl = if pos == input[j].texture as f32 { 1_f32 } else { (pos - input[j].texture as f32) / pos };
                    mltpl * neg.abs() / (neg.abs() + pos)
                } else {
                    let mltpl = if neg == input[j].texture as f32 { 1_f32 } else { (neg - input[j].texture as f32) / neg };
                    mltpl * pos / (neg.abs() + pos)
                };
                multiplyers[j].3 = t;
            }
        }
    }

    let mut table: HashMap<String, i32> = HashMap::new();
    for i in &input {
        table.insert(i.name.clone(), 0);
    }

    let coefficients: Vec<f32> = multiplyers.into_iter()
        .map(|m| {
            let sum = m.0 + m.1 + m.2 + m.3;
            let mut d = 0_f32;
            if m.0 != 0_f32 {
                d += 1_f32;
            }
            if m.1 != 0_f32 {
                d += 1_f32;
            }
            if m.2 != 0_f32 {
                d += 1_f32;
            }
            if m.3 != 0_f32 {
                d += 1_f32;
            }
            sum / d
        })
        .collect();

    let sum: f32 = coefficients.iter().sum();
    let max = target as f32 / sum;

    let x: Vec<f32> = coefficients.iter().map(|c| c * max).collect();
    let x: Vec<i32> = x.iter().map(|x| x.floor() as i32).collect();

    let result = x.iter()
        .zip(input.iter())
        .map(|(x, c)| { c.evaluate(*x) })
        .fold((0, 0, 0, 0), |mut acc, r| {
            acc.0 += r.0;
            acc.1 += r.1;
            acc.2 += r.2;
            acc.3 += r.3;
            acc
        });
    let result = {
        if result.0 <= 0 || result.1 <= 0 || result.2 <= 0 || result.3 <= 0 {
            0
        } else {
            result.0 * result.1 * result.2 * result.3
        }
    };

    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("src/aoc/y2015/task_15").unwrap();
    let input = BufReader::new(input);
}