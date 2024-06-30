use crate::Solution;
use std::cmp::Reverse;

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = {
            let mut nums: Vec<i32> = nums.iter().cloned().collect();
            nums.sort_by_key(|&n| Reverse(n));
            nums
        };

        for i in 0..nums.len() - 2 {
            let (j, k) = (i + 1, i + 2);
            if nums[k] + nums[j] > nums[i] {
                return nums[i] + nums[j] + nums[k];
            }
        }

        return 0;
    }
}
