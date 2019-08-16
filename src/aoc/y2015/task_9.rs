use std::fs::File;
use std::io::{BufReader, BufRead};
use std::rc::Rc;
use core::borrow::Borrow;
use std::cell::RefCell;

struct TreeNode {
    name: String,
    destinations: Vec<Destination>,
}

struct Destination {
    to: Rc<RefCell<TreeNode>>,
    distance: usize,
}

impl TreeNode {
    fn new(name: String) -> Self { TreeNode { name, destinations: vec![] } }

    fn add_destination(&mut self, tree_node: Rc<RefCell<TreeNode>>, distance: usize) {
        let destination = Destination { to: tree_node, distance };
        self.destinations.push(destination)
    }

    fn find(&self, name: &str) -> Option<Rc<Self>> {
        let mut result = None;
        for d in self.destinations {
            if d.to.borrow().name == name.to_string() {
                result = Some(d.to.borrow());
                break;
            }
        }
        for d in self.destinations {
            let r = d.to.borrow().find(name);
            if r.is_some() {
                result = r;
                break;
            }
        }
        return result;
    }
}

#[derive(Clone)]
struct Route {
    from: String,
    to: String,
    distance: usize,
}

impl Route {
    fn new(from: String, to: String, distance: usize) -> Self { Route { from, to, distance } }
}

fn distance_to(routes: &Vec<Route>, from: &String, to: &String) -> Option<usize> {
    routes.iter().find(|r| &r.from == from && &r.to == to).map(|r| r.distance)
}

pub fn run() {
    let input = File::open("src/aoc/y2015/task_9").unwrap();
    let input = BufReader::new(input);
    let routes: Vec<TreeNode> = input.lines()
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| {
            let route_and_distance: Vec<&str> = l.split(" = ").collect();
            let route_part: String = route_and_distance[0].to_string();
            let distance = route_and_distance[1].parse::<usize>().unwrap();
            let route: Vec<&str> = route_part.split(" to ").collect();
            Route::new(route[0].to_string(), route[1].to_string(), distance)
        })
        .fold(vec![], |mut acc: Vec<Rc<TreeNode>>, r: Route| {
            let mut from = acc
                .into_iter()
                .find(|tn| tn.name == r.from);
            if from.is_none() {
                from = acc
                    .into_iter()
                    .find(|tn| tn.find(&r.from).is_some());
            }
            let mut from = if from.is_none() {
                let tn = Rc::new(TreeNode::new(r.from));
                acc.push(tn);
                tn.borrow()
            } else {
                from.unwrap().borrow()
            };

            let mut to = acc
                .into_iter()
                .find(|tn| tn.name == r.to);
            if to.is_none() {
                to = acc
                    .into_iter()
                    .find(|tn| tn.find(&r.to).is_some());
            }
            let to = if to.is_none() {
                Rc::new(TreeNode::new(r.to))
            } else {
                to.unwrap().borrow()
            };

            from.add_destination(to, r.distance);

            acc
        });
}

pub fn run_e() {
    let input = File::open("src/aoc/y2015/task_9").unwrap();
    let input = BufReader::new(input);
}