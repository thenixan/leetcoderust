use std::rc::Rc;

mod problem_5;
mod problem_70;
mod problem_226;
mod problem_337;
mod problem_857;

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

// 10/70 = 0.14
// 20/50 = 0.4
// 5/30 = 0.16

// 3/4 = 0.75
// 1/8 = 0.125
// 10/2 = 5
// 10/2 = 5
// 1/7 = 0.14

// 17/2 = 8.5
// 6/1 = 6
// 4/5 = 0.8
pub fn problem_584() {
    assert_eq!(105f64, problem_857::Solution::mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2));
    assert_eq!(25.5f64, problem_857::Solution::mincost_to_hire_workers(vec![2, 1, 5], vec![17, 6, 4], 2));
    assert_eq!(30.66667f64, problem_857::Solution::mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3));
    assert_eq!(1979.31429f64, problem_857::Solution::mincost_to_hire_workers(vec![25, 68, 35, 62, 52, 57, 35, 83, 40, 51], vec![147, 97, 251, 129, 438, 443, 120, 366, 362, 343], 6));
}