use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut dests = vec![None; n * n];
        let mut visited = HashSet::from([0]);
        let mut deque = VecDeque::from([(0, 0)]);

        for i in 1..dests.len() - 1 {
            let r = n - 1 - i / n;
            let c = match i / n % 2 {
                0 => i % n,
                _ => n - 1 - i % n,
            };

            if board[r][c] > 0 {
                dests[i] = Some(board[r][c] as usize - 1);
            }
        }

        while let Some((i, m)) = deque.pop_front() {
            if i == n * n - 1 {
                return m;
            }

            for j in i + 1..(i + 7).min(n * n) {
                let j = dests[j].unwrap_or(j);

                if !visited.contains(&j) {
                    visited.insert(j);
                    deque.push_back((j, m + 1));
                }
            }
        }

        -1
    }
}
