use crate::Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns.iter().filter(|&s| word.contains(s)).count() as i32
    }
}
