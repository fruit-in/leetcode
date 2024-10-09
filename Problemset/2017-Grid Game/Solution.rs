impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();
        let mut prefix_sum = vec![0; n];
        let mut suffix_sum = vec![0; n];
        let mut ret = i64::MAX;

        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + grid[1][i - 1] as i64;
            suffix_sum[n - 1 - i] = suffix_sum[n - i] + grid[0][n - i] as i64;
        }

        for i in 0..n {
            ret = ret.min(prefix_sum[i].max(suffix_sum[i]));
        }

        ret
    }
}
