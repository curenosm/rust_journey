use crate::Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut cur = String::from("");
        let k: usize = k as usize;

        for i in 0..s.len() {
            cur.push(s.chars().nth(i).unwrap_or(' '));
            if (i + 1) % k == 0 {
                result.push(cur);
                cur = String::from("");
            }
        }

        if cur.len() != 0 {
            while cur.len() < k {
                cur.push(fill)
            }
            result.push(cur);
        }

        result
    }
}
