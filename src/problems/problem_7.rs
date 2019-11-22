use std::iter;

pub struct Solution {}

impl Solution {
    pub fn reverse(input: i32) -> i32 {
        if !Solution::check(input) {
            return 0;
        }
        let mut result: u32 = 0;
        let input_s: Vec<char> = input.abs().to_string().chars().collect();
        for i in 0..input_s.len() {
            let m = 10u32.pow(i as u32);
            result += input_s.get(i).unwrap_or(&'0').to_digit(10).unwrap() as u32 * m;
        }
        if input < 0 {
            return 0i32 - result as i32;
        } else {
            return result as i32;
        }
    }

    fn check(input: i32) -> bool {
        let mut i: String = input.to_string();
        if input < 0 {
            i = i.chars().skip(1).collect();
        }
        if i.len() > 10 {
            return false;
        }
        i.extend(iter::repeat('0').take(10 - i.len()));
        return Solution::check_iter(input > 0, i.chars().rev().collect(), 0);
    }

    fn check_iter(is_pos: bool, input: Vec<char>, iter: usize) -> bool {
        let c: Vec<char> = std::i32::MAX.to_string().chars().collect();
        if iter == input.len() {
            return true;
        } else if input[iter] == c[iter] {
            return Solution::check_iter(is_pos, input, iter + 1);
        } else if input[iter] < c[iter] {
            return true;
        } else {
            return false;
        }
    }
}