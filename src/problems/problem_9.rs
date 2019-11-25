pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let count = Solution::num_digits(x);
        let mid = count / 2;
        let mut i = 0usize;
        while i < mid {
            let l = Solution::digit_at(x, i);
            let r = Solution::digit_at(x, count - i - 1);
            if l != r {
                return false;
            }
            i += 1;
        }
        return true;
    }

    fn num_digits(x: i32) -> usize {
        let mut count = 0;
        let mut cur = x / 1;
        while cur != 0 {
            count += 1;
            cur = cur / 10i32;
        }
        return count as usize;
    }

    fn digit_at(x: i32, pos: usize) -> i32 {
        let mltplier = 10i32.pow(pos as u32);
        if pos == 9 {
            return x / mltplier;
        }
        let n_m = 10i32.pow((pos + 1) as u32);
        return x / mltplier - (x / n_m) * 10;
    }
}