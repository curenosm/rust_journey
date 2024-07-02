use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let sorted = {
            let mut vec = arr
                .iter()
                .cloned()
                .collect::<HashSet<i32>>()
                .into_iter()
                .collect::<Vec<i32>>();
            vec.sort();
            vec
        };

        return arr
            .into_iter()
            .map(|n| -> i32 {
                match sorted.iter().position(|x| *x == n) {
                    Some(i) => i as i32 + 1,
                    None => -1,
                }
            })
            .collect::<Vec<i32>>();
    }
}
