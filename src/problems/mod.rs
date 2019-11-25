use crate::problems::problem_337::TreeNode;

mod problem_4;
mod problem_5;
mod problem_6;
mod problem_7;
mod problem_8;
mod problem_9;
mod problem_11;
mod problem_13;
mod problem_50;
mod problem_70;
mod problem_226;
mod problem_337;
mod problem_561;
mod problem_838;
mod problem_857;
mod problem_1052;

pub fn problem_4() {
    assert_eq!(2f64, problem_4::Solution::find_median_sorted_arrays(vec![], vec![2]));
    assert_eq!(2f64, problem_4::Solution::find_median_sorted_arrays(vec![2], vec![]));
    assert_eq!(0f64, problem_4::Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]));
    assert_eq!(1.5f64, problem_4::Solution::find_median_sorted_arrays(vec![1], vec![2]));
    assert_eq!(2f64, problem_4::Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
    assert_eq!(2.5f64, problem_4::Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
}

pub fn problem_5() {
    assert_eq!("ddtattarrattatdd".to_string(), problem_5::Solution::longest_palindrome("babaddtattarrattatddetartrateedredividerb".to_string()));
    assert_eq!("bb".to_string(), problem_5::Solution::longest_palindrome("abb".to_string()));
}

pub fn problem_6() {
    assert_eq!("PAHNAPLSIIGYIR".to_string(), problem_6::Solution::convert("PAYPALISHIRING".to_string(), 3));
    assert_eq!("PINALSIGYAHRPI".to_string(), problem_6::Solution::convert("PAYPALISHIRING".to_string(), 4));
    assert_eq!("AB".to_string(), problem_6::Solution::convert("AB".to_string(), 1));
    assert_eq!("AB".to_string(), problem_6::Solution::convert("AB".to_string(), 2));
    assert_eq!("ABC".to_string(), problem_6::Solution::convert("ABC".to_string(), 3));
    assert_eq!("ACB".to_string(), problem_6::Solution::convert("ABC".to_string(), 2));
}

pub fn problem_7() {
    assert_eq!(321, problem_7::Solution::reverse(123));
    assert_eq!(0, problem_7::Solution::reverse(1534236469));
    assert_eq!(0, problem_7::Solution::reverse(2147483647));
    assert_eq!(0, problem_7::Solution::reverse(-2147483648));
    assert_eq!(-321, problem_7::Solution::reverse(-123));
    assert_eq!(21, problem_7::Solution::reverse(120));
}

pub fn problem_8() {
    assert_eq!(42, problem_8::Solution::my_atoi("42".to_string()));
    assert_eq!(-42, problem_8::Solution::my_atoi("    -42".to_string()));
    assert_eq!(4193, problem_8::Solution::my_atoi("4193 with words".to_string()));
    assert_eq!(0, problem_8::Solution::my_atoi("words and 987".to_string()));
    assert_eq!(-2147483648, problem_8::Solution::my_atoi("-91283472332".to_string()));
    assert_eq!(1, problem_8::Solution::my_atoi("+1".to_string()));
}

pub fn problem_9() {
    assert_eq!(true, problem_9::Solution::is_palindrome(121));
    assert_eq!(false, problem_9::Solution::is_palindrome(-121));
    assert_eq!(false, problem_9::Solution::is_palindrome(10));
    assert_eq!(true, problem_9::Solution::is_palindrome(1410110141));
}

pub fn problem_11() {
    assert_eq!(49, problem_11::Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}

pub fn problem_13() {
    assert_eq!(3, problem_13::Solution::roman_to_int("III".to_string()));
    assert_eq!(4, problem_13::Solution::roman_to_int("IV".to_string()));
    assert_eq!(9, problem_13::Solution::roman_to_int("IX".to_string()));
    assert_eq!(58, problem_13::Solution::roman_to_int("LVIII".to_string()));
    assert_eq!(1994, problem_13::Solution::roman_to_int("MCMXCIV".to_string()));
}

pub fn problem_50() {
    assert_eq!(1024f64, problem_50::Solution::my_pow(2f64, 10));
    assert_eq!(0.25f64, problem_50::Solution::my_pow(2f64, -2));
    assert_eq!(9.261f64, problem_50::Solution::my_pow(2.1f64, 3));
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

pub fn problem_226() {
    assert_eq!(None, problem_226::Solution::invert_tree(None));
}

pub fn problem_337() {
    assert_eq!(7, problem_337::Solution::rob(TreeNode::new_from_str("[2,1,3,null,4]")));
    assert_eq!(7, problem_337::Solution::rob(TreeNode::new_from_str("[3,2,3,null,3,null,null,null,1]")));
    assert_eq!(24, problem_337::Solution::rob(TreeNode::new_from_str("[8,5,null,4,null,16,null]")));
    assert_eq!(9, problem_337::Solution::rob(TreeNode::new_from_str("[3,4,5,1,3,null,null,null,null,null,1]")));
}

pub fn problem_561() {
    assert_eq!(4, problem_561::Solution::array_pair_sum(vec![1, 4, 3, 2]));
}

pub fn problem_584() {
    assert_eq!(105f64, problem_857::Solution::mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2));
    assert_eq!(25.5f64, problem_857::Solution::mincost_to_hire_workers(vec![2, 1, 5], vec![17, 6, 4], 2));
    assert_eq!(30.66667f64, problem_857::Solution::mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3));
    assert_eq!(1979.31429f64, problem_857::Solution::mincost_to_hire_workers(vec![25, 68, 35, 62, 52, 57, 35, 83, 40, 51], vec![147, 97, 251, 129, 438, 443, 120, 366, 362, 343], 6));
}

pub fn problem_838() {
    assert_eq!("LL.RR.LLRRLL..".to_string(), problem_838::Solution::push_dominoes(".L.R...LR..L..".to_string()));
    assert_eq!("RR.L".to_string(), problem_838::Solution::push_dominoes("RR.L".to_string()));
}

pub fn problem_1052() {
    assert_eq!(16, problem_1052::Solution::max_satisfied(vec![1, 0, 1, 2, 1, 1, 7, 5], vec![0, 1, 0, 1, 0, 1, 0, 1], 3));
}