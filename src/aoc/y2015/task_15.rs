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

    let mut multiplyers = vec![((0f32..=100f32), (0f32..=100f32), (0f32..=100f32), (0f32..=100f32)); input.len()];

    for i in 0..4 {
        let (pos, neg): (f32, f32) = if i == 0 {
            let pos: i32 = input.iter().filter_map(|p| if p.capacity >= 0 { Some(p.capacity) } else { None }).sum();
            let neg: i32 = input.iter().filter_map(|p| if p.capacity < 0 { Some(p.capacity) } else { None }).sum();
            (pos as f32, neg as f32)
        } else if i == 1 {
            let pos: i32  = input.iter().filter_map(|p| if p.durability >= 0 { Some(p.durability) } else { None }).sum();
            let neg: i32  = input.iter().filter_map(|p| if p.durability < 0 { Some(p.durability) } else { None }).sum();
            (pos as f32, neg as f32)
        } else if i == 2 {
            let pos: i32  = input.iter().filter_map(|p| if p.flavor >= 0 { Some(p.flavor) } else { None }).sum();
            let neg: i32  = input.iter().filter_map(|p| if p.flavor < 0 { Some(p.flavor) } else { None }).sum();
            (pos as f32, neg as f32)
        } else if i == 3 {
            let pos: i32  = input.iter().filter_map(|p| if p.texture >= 0 { Some(p.texture) } else { None }).sum();
            let neg: i32  = input.iter().filter_map(|p| if p.texture < 0 { Some(p.texture) } else { None }).sum();
            (pos as f32, neg as f32)
        } else {
            (0f32, 0f32)
        };
        for j in 0..input.len() {
            if i == 0 {
                let t = if input[j].capacity == 0 {
                    0f32
                } else if input[j].capacity > 0 {
                    100f32 * input[j].capacity as f32 / (neg.abs() + pos)
                } else {
                    100f32 * input[j].capacity as f32 / pos
                };
                let r = if t >= 0f32 { (t..=100f32) } else { (0f32..=t.abs()) };
                multiplyers[j].0 = r;
            } else if i == 1 {
                let t = if input[j].durability == 0 {
                    0f32
                } else if input[j].durability > 0 {
                    100f32 * input[j].durability as f32 / (neg.abs() + pos)
                } else {
                    100f32 * input[j].durability as f32 / pos
                };
                let r = if t >= 0f32 { (t..=100f32) } else { (0f32..=t.abs()) };
                multiplyers[j].1 = r;
            } else if i == 2 {
                let t = if input[j].flavor == 0 {
                    0f32
                } else if input[j].flavor > 0 {
                    100f32 * input[j].flavor as f32 / (neg.abs() + pos)
                } else {
                    100f32 * input[j].flavor as f32 / pos
                };
                let r = if t >= 0f32 { (t..=100f32) } else { (0f32..=t.abs()) };
                multiplyers[j].2 = r;
            } else if i == 3 {
                let t = if input[j].texture == 0 {
                    0f32
                } else if input[j].texture > 0 {
                    100f32 * input[j].texture as f32 / (neg.abs() + pos)
                } else {
                    100f32 * input[j].texture as f32 / pos
                };
                let r = if t >= 0f32 { (t..=100f32) } else { (0f32..=t.abs()) };
                multiplyers[j].3 = r;
            }
        }
    }

    let mut table: HashMap<String, i32> = HashMap::new();
    for i in &input {
        table.insert(i.name.clone(), 0);
    }

    let (tx, rx) = channel();
    for p in 0..input.len() {
        let tx = tx.clone();
        let t = table.clone();
        let i = input.clone();
        thread::spawn(move || {
            let table = vec![0; i.len()];
            let result = evaluate(&i, &table, target - 1);
            tx.send(result);
        });
    }

    let mut result = 0;
    for p in &input {
        let r = rx.recv().unwrap();
        if result < r {
            result = r;
        }
    }
    println!("Result: {}", result);
}

fn evaluate(properties: &Vec<Properties>, result: &Vec<i32>, target: i32) -> i32 {
    if target == 0 {
        return evaluate_score(properties, result);
    }
    let mut max = 0;
    if target % 10 == 0 {
        println!("Evaluating: {}", target);
    }
    for i in 0..properties.len() {
        let mut result = result.clone();
        result[i] += 1;
        let r = evaluate(properties, &result, target - 1);
        if r > max {
            max = r;
        }
    }
    return max;
}

fn evaluate_score(properties: &Vec<Properties>, result: &Vec<i32>) -> i32 {
    let result = result.iter()
        .enumerate()
        .map(|(p, c)| {
            (&properties[p], c)
        })
        .map(|(p, c)| {
            let capacity = p.capacity * *c;
            let durability = p.durability * *c;
            let flavor = p.flavor * *c;
            let texture = p.texture * *c;
            (capacity, durability, flavor, texture)
        })
        .fold((0i32, 0i32, 0i32, 0i32), |mut acc, p| {
            acc.0 += p.0;
            acc.1 += p.1;
            acc.2 += p.2;
            acc.3 += p.3;
            acc
        });
    if result.0 <= 0 || result.1 <= 0 || result.2 <= 0 || result.3 <= 0 {
        0
    } else {
        result.0 * result.1 * result.2 * result.3
    }
}

pub fn run_e() {
    let input = File::open("src/aoc/y2015/task_15").unwrap();
    let input = BufReader::new(input);
}