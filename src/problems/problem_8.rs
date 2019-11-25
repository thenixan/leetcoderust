pub struct Solution {}

impl Solution {
    pub fn my_atoi(input: String) -> i32 {

        let with_no_whitespace = Solution::remove_whitespaces(input);

        let with_no_trailing_text = Solution::remove_after_digits(with_no_whitespace);

        if with_no_trailing_text.is_empty() || with_no_trailing_text == "-".to_string() || with_no_trailing_text == "+".to_string() {
            return 0;
        }
        return match with_no_trailing_text.parse::<i32>() {
            Ok(r) => r,
            Err(_) => {
                if with_no_trailing_text.starts_with("-") {
                    std::i32::MIN
                } else {
                    std::i32::MAX
                }
            }
        };
    }

    fn remove_whitespaces(input: String) -> String {
        let mut chars: Vec<char> = input.chars().collect();
        let mut whitespacing = true;
        let mut i = 0;
        while i < chars.len() {
            if whitespacing && chars[i] == ' ' {
                chars.remove(i);
            } else if whitespacing && chars[i] != ' ' {
                whitespacing = false;
                i += 1;
            } else {
                i += 1;
            }
        }
        return chars.iter().collect();
    }

    fn remove_after_digits(input: String) -> String {
        let mut chars: Vec<char> = input.chars().collect();
        let mut digiting = true;
        let mut minusing = chars.len() > 0 && (chars[0] == '-' || chars[0] == '+');
        let mut i = 0;
        while i < chars.len() {
            if i == 0 && minusing {
                minusing = false;
                i += 1;
            } else if digiting && chars[i].is_numeric() {
                i += 1;
            } else if digiting && !chars[i].is_numeric() {
                digiting = false;
                chars.remove(i);
            } else {
                chars.remove(i);
            }
        }
        return chars.iter().collect();
    }
}