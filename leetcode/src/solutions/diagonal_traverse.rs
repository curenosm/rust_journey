use crate::Solution;

impl Solution {
    // dir is meant to be 1 or -1
    fn diagonal(mat: &Vec<Vec<i32>>, dir: i32, initial_row: i32, initial_col: i32) -> Vec<i32> {
        let (n, m) = (mat.len() as i32, mat[0].len() as i32);

        // Base case, right bottom corner
        if initial_row == n - 1 && initial_col == m - 1 {
            return vec![mat[initial_row as usize][initial_col as usize]];
        }

        let mut ans: Vec<i32> = Vec::new();
        let (mut cur_row, mut cur_col) = (initial_row, initial_col);

        // dir = 1 UPWARDS, dir = -1 DOWNWARDS
        while 0 <= cur_row && cur_row < n && 0 <= cur_col && cur_col < m {
            ans.push(mat[cur_row as usize][cur_col as usize]);

            cur_row -= dir;
            cur_col += dir;
        }

        if cur_row == -1 || cur_col == -1 || cur_row == n || cur_col == m {
            if cur_row == -1 {
                cur_row = if cur_col == m { cur_row + 2 } else { 0 };
                cur_col = if cur_col == m { cur_col - 1 } else { cur_col };
            } else if cur_col == -1 {
                cur_col = if cur_row == n { cur_col + 2 } else { 0 };
                cur_row = if cur_row == n { cur_row - 1 } else { cur_row };
            } else if cur_row == n {
                cur_row -= 1;
                cur_col += 2;
            } else if cur_col == m {
                cur_row += 2;
                cur_col -= 1;
            }

            ans.append(&mut Solution::diagonal(&mat, -dir, cur_row, cur_col));
        }

        ans
    }

    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        Solution::diagonal(&mat, 1, 0, 0)
    }
}
