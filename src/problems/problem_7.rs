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
//            if r {
//                return 0;
//            } else {
            result += input_s.get(i).unwrap_or(&'0').to_digit(10).unwrap() as u32 * m;
//            }
        }
        if input < 0 {
            return 0i32 - result as i32;
        } else {
            return result as i32;
        }
    }

    fn check(input: i32) -> bool {
        //2147483648
        let mut i = input.abs().to_string();
        i.extend(iter::repeat('0').take(10 - i.len()));
        let result = Solution::check_iter(i.chars().collect(), 0);

        println!("{}", i);
//        input.rev
//        let a = std::i32::MAX;
//        let b = std::i32::MIN;
        return true;
    }

    fn check_iter(input: Vec<char>, iter: usize) -> i8 {
        return 0;
    }
}