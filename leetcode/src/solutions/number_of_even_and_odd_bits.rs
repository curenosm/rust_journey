use crate::Solution;

impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = [0, 0].to_vec();
        let bin: String = format!("{n:b}");

        for (i, c) in bin.chars().rev().enumerate() {
            res[i % 2] += if c == '1' { 1 } else { 0 };
        }

        return res;
    }
}
