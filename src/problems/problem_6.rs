pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut result: Vec<String> = vec![];

        for _ in 0..num_rows {
            result.push("".to_string());
        }

        let c: Vec<char> = s.chars().collect();

        let mut row: i32 = 0;
        let mut d: i32 = 1;
        for i in 0..c.len() {
            result[row as usize].push(c[i]);
            row += d;
            if num_rows == 1 {
                d = 0;
                row = 0;
            } else if row == 0 {
                d = 1;
            } else if row == num_rows - 1 {
                d = -1;
            }
        }
        return result.join("");
    }
}

//0 4 8 2
//135791
//2 6 0
//
//0  6
//1 57
//24
//3
//
//0   8
//1  7
//2 6
//35
//4