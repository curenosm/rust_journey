use crate::Solution;

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut sectors = vec![0; n];

        for i in 1..rounds.len() {
            let mut pos = (rounds[i - 1] - 1) as usize;
            while pos != (rounds[i] - 1) as usize {
                sectors[pos] += 1;
                pos = (pos + 1) % n
            }
        }
        sectors[(rounds.last().unwrap() - 1) as usize] += 1;

        let max = sectors[(0..n).max_by_key(|&i| sectors[i]).unwrap()];

        (0..n)
            .filter(|&i| sectors[i] == max)
            .map(|i| i as i32 + 1)
            .collect()
    }
}
