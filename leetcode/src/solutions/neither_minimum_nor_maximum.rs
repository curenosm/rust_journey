use crate::Solution;

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        let (min, max) = (*nums.iter().min().unwrap(), *nums.iter().max().unwrap());

        for n in nums {
            if n != min && n != max {
                return n;
            }
        }

        -1
    }
}
