pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..(height.len() - 1) {
            for j in (i + 1)..height.len() {
                max = i32::max(max, Solution::calc_volume(height[i], i, height[j], j));
            }
        }
        return max;
    }

    fn calc_volume(left: i32, left_pos: usize, right: i32, right_pos: usize) -> i32 {
        let height = i32::min(left, right);
        return (right_pos - left_pos) as i32 * height;
    }
}