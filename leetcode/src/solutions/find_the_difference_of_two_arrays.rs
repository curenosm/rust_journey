use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let (set_1, set_2): (HashSet<i32>, HashSet<i32>) =
            (nums1.into_iter().collect(), nums2.into_iter().collect());
        vec![
            set_1.difference(&set_2).cloned().collect(),
            set_2.difference(&set_1).cloned().collect(),
        ]
    }
}
