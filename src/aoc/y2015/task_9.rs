use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

#[derive(PartialEq, Clone, Debug, Copy)]
struct Distance(usize);

impl Distance {
    fn new(d: usize) -> Self { Distance(d) }
    fn empty() -> Self { Distance(0_usize) }
}

impl Add for Distance {
    type Output = Distance;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0 == 0_usize || rhs.0 == 0_usize {
            Distance(0_usize)
        } else {
            Distance(self.0 + rhs.0)
        }
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else if self.0 == 0_usize {
            Some(Ordering::Greater)
        } else if other.0 == 0_usize {
            Some(Ordering::Less)
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}

#[derive(Debug, Clone)]
struct Route {
    from: String,
    to: String,
    distance: usize,
}

impl Route {
    fn new(from: String, to: String, distance: usize) -> Self { Route { from, to, distance } }
}

pub fn run() {
    let input = File::open("src/aoc/y2015/task_9").unwrap();
    let input = BufReader::new(input);
    let routes: Vec<Route> = input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| {
            let route_and_distance: Vec<&str> = l.split(" = ").collect();
            let route_part: String = route_and_distance[0].to_string();
            let distance = route_and_distance[1].parse::<usize>().unwrap();
            let route: Vec<&str> = route_part.split(" to ").collect();
            Route::new(route[0].to_string(), route[1].to_string(), distance)
        })
        .collect();


    let counts = routes
        .iter()
        .fold(vec![], |mut acc, r| {
            if !acc.contains(&r.to) {
                acc.push(r.to.clone());
            }
            if !acc.contains(&r.from) {
                acc.push(r.from.clone());
            }
            acc
        });

    let mut result = 0;
    for i in 0..counts.len() {
        let from = &counts[i];
        let new_destinations: Vec<String> = counts.iter().filter(|r| r != &from).map(|s| s.clone()).collect();
        let found_length = find_route(&routes, &new_destinations, &from);
        if found_length.is_some() {
            let found_length = found_length.unwrap();
            if result == 0 || result > found_length {
                println!("Start: {}", from);
                result = found_length;
            }
        }
    }
    println!("{:?}", result)
}

fn find_route(routes: &Vec<Route>, destinations: &Vec<String>, from: &String) -> Option<usize> {
    let mut length = 0;
    if destinations.is_empty() {
        println!("Finish: {}", from);
        return Some(0);
    }
    for i in 0..destinations.len() {
        let to = &destinations[i];
        let route = &routes.into_iter().find(|r| {
            ((&r.from == from && &r.to == to) || (&r.from == to && &r.to == from)) && destinations.contains(to)
        });
        if route.is_some() {
            let route_length = route.unwrap().distance;
            let new_destinations: Vec<String> = destinations.iter().filter(|r| r != &to).map(|s| s.clone()).collect();
            let found_length = find_route(routes, &new_destinations, &to).map(|l| l + route_length);
            if found_length.is_some() {
                let found_length = found_length.unwrap();
                if length == 0 || length > found_length {
                    println!("To: {}", to);
                    length = found_length;
                }
            }
        }
    }
    if length != 0 {
        return Some(length);
    } else {
        return None;
    }
}

pub fn run_e() {
    let input = File::open("src/aoc/y2015/task_9").unwrap();
    let _input = BufReader::new(input);
}