use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use itertools::Itertools;
use regex::Regex;
use std::ops::{Index, Deref};
use std::slice::SliceIndex;
use std::collections::{BTreeSet, HashSet};
use std::fmt::{Display, Formatter};
use core::fmt::{Debug, Write};
use core::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Element {
    name: String
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.name, f)
    }
}

impl Element {
    fn new(name: String) -> Self { Element { name } }
    fn len(&self) -> usize { self.name.len() }
}

//impl ToString for Element {
//    fn to_string(&self) -> String { self.name.clone() }
//}

impl FromStr for Element {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Element::new(s.to_string())) }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Chain {
    value: Vec<Element>
}

impl Display for Chain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_char('[');
        for i in 0 ..self.value.len() {
            Display::fmt(&self.value[i], f);
            f.write_str(", ");
        }
        f.write_char(']')
    }
}

impl IntoIterator for Chain {
    type Item = Element;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl Deref for Chain {
    type Target = [Element];

    fn deref(&self) -> &Self::Target {
        self.value.deref()
    }
}

impl FromStr for Chain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([A-Z][a-z]?)").unwrap();
        let mut result: Vec<Element> = vec![];
        for cap in re.captures_iter(s) {
            result.push(cap[0].parse().unwrap())
        }
        Ok(Chain { value: result })
    }
}

impl<I: SliceIndex<[Element]>> Index<I> for Chain {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.value.index(index)
    }
}

impl Chain {
    fn len(&self) -> usize { self.value.len() }
    fn replace(mut self, pos: usize, with: &Chain) -> Self {
        self.value.remove(pos);
        for i in 0..with.len() {
            self.value.insert(pos + i, with[i].clone());
        }
        self
    }
    fn contains(&self, other: &Element) -> bool {
        self.value.contains(other)
    }

    fn indexes_of_subchain(&self, other: &Chain) -> Vec<usize> {
        let mut result = vec![];
        for i in 0..self.len() - other.len() {
            let mut j = 0;
            while j < other.len() && self[i] == other[j] {
                j += 1;
            }
            if j == other.len() {
                result.push(i);
            }
        }
        return result;
    }
}

//impl Into<Chain> for Element {
//    fn into(self) -> Chain {
//        Chain { value: vec![self] }
//    }
//}

impl From<Element> for Chain {
    fn from(e: Element) -> Self {
        Chain { value: vec![e] }
    }
}

//impl ToString for Chain {
//    fn to_string(&self) -> String {
//        self.value.iter().map(|v| v.to_string()).join("")
//    }
//}

#[derive(Debug, Clone, PartialEq)]
struct Replacement {
    from: Element,
    to: Chain,
}

impl FromStr for Replacement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split(" => ").collect();
        let from: Element = s[0].parse().unwrap();
        let to: Chain = s[1].parse().unwrap();
        Result::Ok(Replacement { from, to })
    }
}

pub fn run() {
    let input = File::open("src/aoc/y2015/task_19").unwrap();
    let input = BufReader::new(input);

    let input: Vec<String> = input.lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()).collect();

    let mut input = input;
    let target: Chain = input.pop().unwrap().clone().parse().unwrap();

    let input: Vec<Replacement> = input.iter().filter_map(|l| l.parse::<Replacement>().ok()).collect();

    let mut result: BTreeSet<String> = BTreeSet::new();

    for i in 0..target.len() {
        for j in 0..input.len() {
            if target[i] == input[j].from {
                result.insert(target.clone().replace(i, &input[j].to).to_string());
            }
        }
    }

//    result.sort();
//    let result = result.into_iter().dedup().collect::<Vec<String>>();

    println!("Result: {}", result.len());
}

pub fn run_e() {
    let input = File::open("src/aoc/y2015/task_19").unwrap();
    let input = BufReader::new(input);

    let input: Vec<String> = input.lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()).collect();

    let mut input = input;

    let target: Chain = input.pop().unwrap().clone().parse().unwrap();

    let mut input: Vec<Replacement> = input.iter().filter_map(|l| l.parse::<Replacement>().ok()).collect();

    let mut elements = elements(&input);
    let terminal_elements = find_terminal_elements(&elements, &input);
    elements = elements.into_iter().filter(|p| !terminal_elements.contains(p)).collect();

    let terminal_mutations = find_mutations_with_elements(&terminal_elements, &input);
    let non_terminal_mutations: Vec<Replacement> = input.into_iter().filter(|m| !terminal_mutations.contains(&m)).collect();
//    let result = iterate(target.as_str(), &input, 0);

    let result = iterate(&terminal_mutations, &non_terminal_mutations, &target, 0).unwrap();
    println!("Result: {}", 0);
}

fn iterate(terminal_mutations: &Vec<Replacement>, non_terminal_mutations: &Vec<Replacement>, target: &Chain, step: usize) -> Option<usize> {
    let mut result = None;

    if step % 100 == 0 {
        println!("{}: {}", step, target);
    }

    if target.len() == 1 && &target[0].name == "e" {
        return Some(0);
    }

    let mut target = target.clone();
    for i in 0..terminal_mutations.len() {
        let indexes = target.indexes_of_subchain(&terminal_mutations[i].to);
        for ind in indexes {
            target = target.replace(ind, &terminal_mutations[i].from.clone().into());
        }
    }

    for i in 0..non_terminal_mutations.len() {
        let indexes = target.indexes_of_subchain(&non_terminal_mutations[i].to);
        for ind in indexes {
            let r = iterate(terminal_mutations, non_terminal_mutations, &target.clone().replace(ind, &non_terminal_mutations[i].from.clone().into()), step + 1);
            if r.is_some() && (result.is_none() || result.unwrap() > r.unwrap()) {
                result = r;
            }
        }
    }

    return result;
}

fn find_mutations_with_elements(elements: &Vec<Element>, mutations: &Vec<Replacement>) -> Vec<Replacement> {
    mutations.clone().into_iter().filter(|p| {
        elements.iter().any(|e| p.to.contains(e))
    }).collect()
}

fn find_terminal_elements(elements: &Vec<Element>, mutations: &Vec<Replacement>) -> Vec<Element> {
    let mut result = vec![];
    for e in elements {
        if mutations.iter().any(|p| p.to.contains(e)) && !mutations.iter().any(|p| &p.from == e) {
            result.push(e.clone())
        }
    }
    return result;
}

fn elements(mutations: &Vec<Replacement>) -> Vec<Element> {
    let mut result: HashSet<Element> = HashSet::new();
    for i in 0..mutations.len() {
        result.insert(mutations[i].from.clone());
        for j in 0..mutations[i].to.len() {
            result.insert(mutations[i].to[j].clone());
        }
    }
    return result.into_iter().collect();
}