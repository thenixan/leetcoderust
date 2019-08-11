mod problem_5;
mod problem_561;

pub fn problem_5() {
    assert_eq!("ddtattarrattatdd".to_string(), problem_5::Solution::longest_palindrome("babaddtattarrattatddetartrateedredividerb".to_string()));
    assert_eq!("bb".to_string(), problem_5::Solution::longest_palindrome("abb".to_string()));
}

pub fn problem_561() {
    assert_eq!(4, problem_561::Solution::array_pair_sum(vec![1, 4, 3, 2]));
}