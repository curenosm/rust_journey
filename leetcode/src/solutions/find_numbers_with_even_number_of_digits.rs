use crate::Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter()
            .map(|n| n.to_string())
            .filter(|s| s.len() % 2 == 0)
            .map(|s| s.len())
            .count() as i32
    }
}
