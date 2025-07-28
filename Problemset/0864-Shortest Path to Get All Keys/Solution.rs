use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let grid = grid.iter().map(|row| row.as_bytes()).collect::<Vec<_>>();
        let (m, n) = (grid.len(), grid[0].len());
        let mut k = 0;
        let mut visited = HashSet::new();
        let mut deque = VecDeque::new();

        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == b'@' {
                    visited.insert((r, c, 0));
                    deque.push_back((r, c, 0, 0));
                } else if grid[r][c].is_ascii_lowercase() {
                    k += 1;
                }
            }
        }

        while let Some((r, c, keys, moves)) = deque.pop_front() {
            if keys == (1 << k) - 1 {
                return moves;
            }

            if r > 0
                && grid[r - 1][c] != b'#'
                && (!grid[r - 1][c].is_ascii_uppercase()
                    || (keys >> (grid[r - 1][c] - b'A')) & 1 == 1)
            {
                if grid[r - 1][c].is_ascii_lowercase()
                    && visited.insert((r - 1, c, keys | (1 << (grid[r - 1][c] - b'a'))))
                {
                    deque.push_back((r - 1, c, keys | (1 << (grid[r - 1][c] - b'a')), moves + 1));
                } else if !grid[r - 1][c].is_ascii_lowercase() && visited.insert((r - 1, c, keys)) {
                    deque.push_back((r - 1, c, keys, moves + 1));
                }
            }
            if r < m - 1
                && grid[r + 1][c] != b'#'
                && (!grid[r + 1][c].is_ascii_uppercase()
                    || (keys >> (grid[r + 1][c] - b'A')) & 1 == 1)
            {
                if grid[r + 1][c].is_ascii_lowercase()
                    && visited.insert((r + 1, c, keys | (1 << (grid[r + 1][c] - b'a'))))
                {
                    deque.push_back((r + 1, c, keys | (1 << (grid[r + 1][c] - b'a')), moves + 1));
                } else if !grid[r + 1][c].is_ascii_lowercase() && visited.insert((r + 1, c, keys)) {
                    deque.push_back((r + 1, c, keys, moves + 1));
                }
            }
            if c > 0
                && grid[r][c - 1] != b'#'
                && (!grid[r][c - 1].is_ascii_uppercase()
                    || (keys >> (grid[r][c - 1] - b'A')) & 1 == 1)
            {
                if grid[r][c - 1].is_ascii_lowercase()
                    && visited.insert((r, c - 1, keys | (1 << (grid[r][c - 1] - b'a'))))
                {
                    deque.push_back((r, c - 1, keys | (1 << (grid[r][c - 1] - b'a')), moves + 1));
                } else if !grid[r][c - 1].is_ascii_lowercase() && visited.insert((r, c - 1, keys)) {
                    deque.push_back((r, c - 1, keys, moves + 1));
                }
            }
            if c < n - 1
                && grid[r][c + 1] != b'#'
                && (!grid[r][c + 1].is_ascii_uppercase()
                    || (keys >> (grid[r][c + 1] - b'A')) & 1 == 1)
            {
                if grid[r][c + 1].is_ascii_lowercase()
                    && visited.insert((r, c + 1, keys | (1 << (grid[r][c + 1] - b'a'))))
                {
                    deque.push_back((r, c + 1, keys | (1 << (grid[r][c + 1] - b'a')), moves + 1));
                } else if !grid[r][c + 1].is_ascii_lowercase() && visited.insert((r, c + 1, keys)) {
                    deque.push_back((r, c + 1, keys, moves + 1));
                }
            }
        }

        -1
    }
}
