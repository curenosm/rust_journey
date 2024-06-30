use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut set: HashSet<(i32, i32, i32)> = HashSet::new();
        let is_square_triplet = |(a, b, c)| {
            a * a + b * b == c * c
        };

        for a in 1..n + 1 {
            for b in 1..n + 1 {
                for c in 1..n + 1 {
                    let cur = (a, b, c);
                    if is_square_triplet(cur) && !set.contains(&cur) {
                        set.insert(cur);
                    }
                }
            }
        }
        
        return set.len() as i32;
    }
}