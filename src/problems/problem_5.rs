pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        return Self::iterate_string(&s, 0).to_string();
    }

    fn iterate_string(s: &str, gap: usize) -> &str {
        let len = s.len();

        let mut i: usize = 0;
        while i <= gap {
            let substring: &str = &s[i..len + i - gap];
            if Self::is_palindrome(substring) {
                return substring;
            }
            i += 1;
        }
        return Self::iterate_string(s, gap + 1);
    }

    fn is_palindrome(s: &str) -> bool {
        let mut i: usize = 0;
        let bytes = s.as_bytes();
        while i < bytes.len() / 2 {
            let j = bytes.len() - i - 1;
            if bytes[i] != bytes[j] {
                return false;
            }
            i += 1;
        }
        return true;
    }
}