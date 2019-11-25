pub struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1f64;
        } if n < 0 {
            let x = 1f64 / x;
            return Solution::pw(x, n);
        } else {
            return Solution::pw(x, n);
        }
    }

    fn pw(x: f64, n: i32) -> f64 {
        if n == 1 || n == -1 {
            return x;
        } else if n == 2 || n == -2 {
            return x*x;
        } else if n == 3 || n == -3 {
            return x*x*x;
        } else if n%2 == 0 {
            return Solution::pw(x*x, n/2);
        } else {
            return x*Solution::pw(x*x, n/2);
        }
    }
}