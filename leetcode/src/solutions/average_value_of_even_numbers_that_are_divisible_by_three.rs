use crate::Solution;

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let valid: Vec<i32> = nums
            .into_iter()
            .filter(|n| n % 2 == 0 && n % 3 == 0)
            .collect();
        if valid.len() != 0 {
            (valid.iter().sum::<i32>() as f64 / valid.len() as f64).floor() as i32
        } else {
            0
        }
    }
}
