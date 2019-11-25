pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let size = nums1.len() + nums2.len();
        let (pos_l, pos_r) = Solution::median_pos(size);

        let mut i = 0;
        let mut j = 0;

        let mut l = 0;
        let mut r = 0;

        let mut found = false;

        while !found {
            if i < nums1.len() && (j >= nums2.len() || nums1[i] <= nums2[j]) {
                l = r;
                r = nums1[i];
                i += 1;
            } else {
                l = r;
                r = nums2[j];
                j += 1;
            }
            if i + j == pos_r {
                found = true;
            }
        }

        if pos_r == pos_l {
            l = r;
        }

        return (l + r) as f64 / 2f64;
    }

    fn median_pos(size: usize) -> (usize, usize) {
        if size % 2 == 0 {
            return (size / 2, (size / 2) + 1);
        } else {
            return ((size + 1) / 2, (size + 1) / 2);
        }
    }
}