use crate::Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        Solution::gcd(
            nums.iter().min().cloned().unwrap_or(-1),
            nums.iter().max().cloned().unwrap_or(-1),
        )
    }

    pub fn gcd(a: i32, b: i32) -> i32 {
        if b != 0 {
            Solution::gcd(b, a % b)
        } else {
            a.abs()
        }
    }
}
