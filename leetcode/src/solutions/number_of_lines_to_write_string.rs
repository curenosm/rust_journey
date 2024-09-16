use crate::Solution;

impl Solution {
    const ABC: &str = "abcdefghijklmnopqrstuvwxyz";

    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut result: [i32; 2] = [1, 0];

        for c in s.chars() {
            let i = match Solution::ABC.find(c) {
                Some(i) => i,
                None => 0,
            };

            if result[1] + widths[i] <= 100 {
                result[1] += widths[i];
            } else {
                result[0] += 1;
                result[1] = widths[i];
            }
        }

        result.to_vec()
    }
}
