use crate::Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sorted = heights.clone();
        sorted.sort();
        let mut res = 0;
        for i in 0..sorted.len() {
            res += if sorted[i] != heights[i] { 1 } else { 0 }
        }
        res
    }
}
