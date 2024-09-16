use crate::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<Vec<i32>> = vec![];

        for n in nums.iter() {
            freq.insert(
                *n,
                match freq.get(n) {
                    Some(n) => n + 1,
                    None => 1,
                },
            );
        }

        while !freq.is_empty() {
            let mut cur: Vec<i32> = vec![];

            for (key, value) in freq.iter_mut() {
                cur.push(*key);
                *value -= 1;
            }

            for (key, value) in freq.clone().iter_mut() {
                if *value <= 0 {
                    freq.remove(key);
                }
            }

            result.push(cur);
        }

        result
    }
}
