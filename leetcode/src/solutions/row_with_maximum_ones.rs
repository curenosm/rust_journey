use crate::Solution;
use std::cmp::max;

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![0 as i32; 2];

        for i in 0..mat.len() {
            res[1] = max(res[1], mat[i].iter().sum::<i32>());
        }

        for i in 0..mat.len() {
            if mat[i].iter().sum::<i32>() == res[1] {
                res[0] = i as i32;
                break;
            }
        }

        res
    }
}
