use crate::Solution;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        for n in nums.iter() {
            if n.abs() < min {
                min = n.abs()
            }
        }

        let mut closest = vec![];
        for n in nums {
            if n.abs() == min {
                closest.push(n)
            }
        }

        *closest.iter().max().unwrap()
    }
}
