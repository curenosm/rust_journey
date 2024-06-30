use crate::Solution;

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut result: String = String::from("");
        let bins: Vec<String> = nums.iter().map(|&n| format!("{:032b}", n)).collect();

        for i in 0..32 {
            let mut cur = 0;
            for n in &bins {
                if match n.chars().nth(i) {
                    Some(d) => d,
                    None => '0',
                } == '1'
                {
                    cur += 1;
                }
            }

            result += if cur >= k { "1" } else { "0" };
        }

        return match i32::from_str_radix(&result, 2) {
            Ok(res) => res,
            Err(e) => -1,
        };
    }
}
