use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut freq: HashMap<_, _> = HashMap::new();
        for n in nums {
            *freq.entry(n).or_insert(0) += 1
        }

        for (k, v) in freq {
            if v % 2 != 0 {
                return false;
            }
        }

        true
    }
}
