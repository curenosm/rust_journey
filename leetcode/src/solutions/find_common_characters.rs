use std::cmp::min;

impl Solution {
    const ABC: &str = "abcdefghijklmnopqrstuvwxyz";

    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        for c in Solution::ABC.chars() {
            let mut times = usize::MAX;
            for w in &words {
                let mut cur = 0;
                for x in w.chars() {
                    if x == c {
                        cur += 1;
                    }
                }
                times = min(times, cur);
            }

            while times > 0 {
                result.push(c.to_string());
                times -= 1;
            }
        }

        return result;
    }
}
