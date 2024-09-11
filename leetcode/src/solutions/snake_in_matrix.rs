use crate::Solution;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn parse(s: &str) -> Direction {
        match s {
            "UP" => Direction::Up,
            "RIGHT" => Direction::Right,
            "DOWN" => Direction::Down,
            "LEFT" => Direction::Left,
            _ => Direction::Up,
        }
    }
}

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let matrix = {
            let mut mat: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];
            for i in 0..n {
                for j in 0..n {
                    mat[i as usize][j as usize] = i * n + j
                }
            }
            mat
        };

        let (mut i, mut j) = (0 as i32, 0 as i32);
        for command in commands {
            match Direction::parse(&command) {
                Direction::Up => i -= 1,
                Direction::Right => j += 1,
                Direction::Down => i += 1,
                Direction::Left => j -= 1,
                _ => {}
            }
        }

        matrix[i as usize][j as usize]
    }
}
