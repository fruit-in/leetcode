impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut indices = vec![];
        let mut paths_count = vec![vec![1; n]; m];
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                indices.push((i, j));
            }
        }

        indices.sort_unstable_by_key(|&(i, j)| grid[i][j]);

        for (i, j) in indices {
            if i > 0 && grid[i - 1][j] < grid[i][j] {
                paths_count[i][j] = (paths_count[i][j] + paths_count[i - 1][j]) % 1_000_000_007;
            }
            if i < m - 1 && grid[i + 1][j] < grid[i][j] {
                paths_count[i][j] = (paths_count[i][j] + paths_count[i + 1][j]) % 1_000_000_007;
            }
            if j > 0 && grid[i][j - 1] < grid[i][j] {
                paths_count[i][j] = (paths_count[i][j] + paths_count[i][j - 1]) % 1_000_000_007;
            }
            if j < n - 1 && grid[i][j + 1] < grid[i][j] {
                paths_count[i][j] = (paths_count[i][j] + paths_count[i][j + 1]) % 1_000_000_007;
            }

            ret = (ret + paths_count[i][j]) % 1_000_000_007;
        }

        ret
    }
}
