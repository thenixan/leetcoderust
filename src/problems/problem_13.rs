pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut digits: Vec<i32> = vec![];
        for i in 0..chars.len() {
            let v = match chars[i] {
                'M' => 1000,
                'D' => 500,
                'C' => 100,
                'L' => 50,
                'X' => 10,
                'V' => 5,
                'I' => 1,
                _ => 0,
            };
            digits.push(v);
        }
        let mut result = 0;
        for i in 0..digits.len() {
            if i + 1 == digits.len() || digits[i] >= digits[i+1] {
                result += digits[i];
            } else {
                result -= digits[i];
            }
        }
        return result;
    }
}