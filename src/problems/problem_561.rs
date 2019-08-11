pub struct Solution {}

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        return nums
            .into_iter()
            .enumerate()
            .filter_map(|(pos, item)| {
                if pos % 2 == 0 {
                    Option::Some(item)
                } else {
                    Option::None
                }
            })
            .sum();
    }
}