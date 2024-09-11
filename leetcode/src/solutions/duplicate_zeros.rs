use crate::Solution;

impl Solution {
    pub fn shift_right(arr: &mut Vec<i32>, from: usize) {
        for i in (from..arr.len()).rev() {
            if i + 1 < arr.len() {
                arr[i + 1] = arr[i]
            }
        }
        arr[from] = 0;
    }

    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i = 0;

        while i < arr.len() {
            if arr[i] == 0 {
                Solution::shift_right(arr, i);
                i += 1;
            }

            i += 1;
        }
    }
}
