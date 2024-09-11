use crate::Solution;

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut cur = String::from("");
        for i in 0..words.len() {
            cur = format!("{}{}", cur, words[i]);
            if cur == s {
                return true;
            }
        }
        false
    }
}
