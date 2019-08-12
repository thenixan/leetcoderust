use std::rc::Rc;

mod problem_5;
mod problem_226;
mod problem_337;
mod problem_561;
mod problem_70;

pub fn problem_5() {
    assert_eq!("ddtattarrattatdd".to_string(), problem_5::Solution::longest_palindrome("babaddtattarrattatddetartrateedredividerb".to_string()));
    assert_eq!("bb".to_string(), problem_5::Solution::longest_palindrome("abb".to_string()));
}

pub fn problem_226() {
    assert_eq!(None, problem_226::Solution::invert_tree(None));
}

pub fn problem_337() {
    assert_eq!(0, problem_337::Solution::rob(None));
}

pub fn problem_561() {
    assert_eq!(4, problem_561::Solution::array_pair_sum(vec![1, 4, 3, 2]));
}

pub fn problem_70() {
    assert_eq!(2, problem_70::Solution::climb_stairs(2));
    assert_eq!(3, problem_70::Solution::climb_stairs(3));
    assert_eq!(5, problem_70::Solution::climb_stairs(4));
    assert_eq!(8, problem_70::Solution::climb_stairs(5));
    assert_eq!(13, problem_70::Solution::climb_stairs(6));
    assert_eq!(21, problem_70::Solution::climb_stairs(7));
    assert_eq!(34, problem_70::Solution::climb_stairs(8));
    assert_eq!(1836311903, problem_70::Solution::climb_stairs(45));
}