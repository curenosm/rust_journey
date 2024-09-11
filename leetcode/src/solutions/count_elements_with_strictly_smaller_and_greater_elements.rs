use crate::Solution;

impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let (mut nums, mut res) = (nums, 0);
        nums.sort();

        for i in 1..nums.len() as i32 - 1 {
            let (mut l, mut r) = (i - 1, i + 1);

            while 0 <= l && r < nums.len() as i32 {
                if nums[l as usize] < nums[i as usize] && nums[i as usize] < nums[r as usize] {
                    res += 1;
                    break;
                } else {
                    l -= if nums[l as usize] >= nums[i as usize] {
                        1
                    } else {
                        0
                    };
                    r += if nums[i as usize] >= nums[r as usize] {
                        1
                    } else {
                        0
                    };
                }
            }
        }

        res
    }
}
