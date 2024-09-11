use crate::Solution;

impl Solution {
    pub fn is_valid_pair(w1: &str, w2: &str) -> bool {
        w1.chars().last().unwrap() == w2.chars().nth(0).unwrap()
    }

    pub fn is_circular_sentence(sentence: String) -> bool {
        let words: Vec<&str> = sentence.split(" ").collect();
        let n = words.len();

        for i in 0..n {
            if !Solution::is_valid_pair(&words[i], &words[(i + 1) % n]) {
                return false;
            }
        }

        true
    }
}
