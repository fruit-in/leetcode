use std::collections::VecDeque;

impl Solution {
    pub fn highest_ranked_k_items(
        grid: Vec<Vec<i32>>,
        pricing: Vec<i32>,
        start: Vec<i32>,
        k: i32,
    ) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let (low, high) = (pricing[0], pricing[1]);
        let (m, n) = (grid.len(), grid[0].len());
        let mut queue = VecDeque::from([(start[0] as usize, start[1] as usize, 0)]);
        let mut positions = vec![];

        while let Some((r, c, d)) = queue.pop_front() {
            if grid[r][c] == 0 {
                continue;
            }

            if grid[r][c] >= low && grid[r][c] <= high {
                positions.push((d, grid[r][c], r as i32, c as i32));
            }

            if r > 0 {
                queue.push_back((r - 1, c, d + 1));
            }
            if r < m - 1 {
                queue.push_back((r + 1, c, d + 1));
            }
            if c > 0 {
                queue.push_back((r, c - 1, d + 1));
            }
            if c < n - 1 {
                queue.push_back((r, c + 1, d + 1));
            }

            grid[r][c] = 0;
        }

        positions.sort_unstable();

        positions
            .iter()
            .take(k as usize)
            .map(|&(_, _, r, c)| vec![r, c])
            .collect()
    }
}
