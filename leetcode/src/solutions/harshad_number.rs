use crate::Solution;

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let (mut sum, mut cur): (i32, i32) = (0, x);

        while cur != 0 {
            sum += cur % 10;
            cur /= 10;
        }

        if x % sum == 0 {
            sum
        } else {
            -1
        }
    }
}
