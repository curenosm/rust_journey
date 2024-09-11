use crate::Solution;
use regex::Regex;

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let ones_count = Regex::new(r"0+")
            .unwrap()
            .split(&s)
            .map(|s| s.len())
            .max_by(|a, b| a.cmp(&b))
            .unwrap_or(0);

        let zeros_count = Regex::new(r"1+")
            .unwrap()
            .split(&s)
            .map(|s| s.len())
            .max_by(|a, b| a.cmp(&b))
            .unwrap_or(0);

        ones_count > zeros_count
    }
}
