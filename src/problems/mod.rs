mod problem_5;

pub fn problem_5() {
    assert_eq!("ddtattarrattatdd".to_string(), problem_5::Solution::longest_palindrome("babaddtattarrattatddetartrateedredividerb".to_string()));
    assert_eq!("bb".to_string(), problem_5::Solution::longest_palindrome("abb".to_string()));
}