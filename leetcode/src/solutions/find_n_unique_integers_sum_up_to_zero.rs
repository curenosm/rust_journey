use crate::Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut res = vec![0; n as usize];
        let (mut i, mut l, mut r) = (
            1,
            (n / 2 - 1) as usize,
            if n as usize % 2 == 1 {
                n as usize / 2 + 1
            } else {
                n as usize / 2
            },
        );

        while 0 <= l && r < n as usize {
            res[l] = -i;
            res[r] = i;
            l -= 1;
            r += 1;
            i += 1;
        }

        res
    }
}
