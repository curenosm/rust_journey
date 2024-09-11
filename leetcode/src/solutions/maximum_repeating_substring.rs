use crate::Solution;

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut res = 0;
        let mut cur = String::from("");

        for k in 0..sequence.len() {
            cur = format!("{}{}", cur, word);
            if sequence.contains(&cur) {
                res += 1
            } else {
                break;
            }
        }

        res
    }
}
