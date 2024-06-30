use crate::Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let (mut dir, mut missing) = (0 as i32, 0 as i32);

        for c in moves.chars() {
            match c {
                'L' => dir -= 1,
                'R' => dir += 1,
                _ => missing += 1,
            }
        }

        missing + dir.abs()
    }
}
