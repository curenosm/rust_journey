use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut set_res: HashSet<&i32> = nums[0].iter().collect();

        for arr in nums.iter() {
            set_res = set_res
                .intersection(&arr.iter().collect())
                .copied()
                .collect()
        }

        let mut res = set_res.into_iter().copied().collect::<Vec<_>>();
        res.sort();
        res
    }
}
