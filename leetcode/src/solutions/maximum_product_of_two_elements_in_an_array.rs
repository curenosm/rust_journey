use crate::Solution;
use std::cmp;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                max = cmp::max(max, (nums[i] - 1) * (nums[j] - 1));
            }
        }

        return max;
    }
}
