use crate::Solution;

impl Solution {
    pub fn convert_coords(x: char, y: char) -> (usize, usize) {
        (
            match x {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                'h' => 7,
                _ => 0,
            },
            ((y.to_digit(10).unwrap_or(0) as i32) - 1) as usize,
        )
    }

    pub fn square_is_white(coordinates: String) -> bool {
        let mut matrix = vec![vec![false; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                if j % 2 == (i + 1) % 2 {
                    matrix[i][j] = true
                }
            }
        }

        let (row, col) = Solution::convert_coords(
            coordinates.chars().nth(0).unwrap_or('a'),
            coordinates.chars().nth(1).unwrap_or('0'),
        );

        matrix[row][col]
    }
}
