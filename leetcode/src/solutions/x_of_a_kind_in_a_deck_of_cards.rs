use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a.abs()
    }

    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut freq: HashMap<&i32, i32> = HashMap::new();

        for num in deck.iter() {
            *freq.entry(num).or_insert(0) += 1
        }

        let mut gcd = *freq.get(&deck[0]).unwrap_or(&0);

        for (k, v) in freq.iter() {
            if gcd == 1 {
                return false;
            } else {
                gcd = Solution::gcd(gcd, *v)
            }
        }

        gcd != 1
    }
}
