impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let k = (k as usize) % (m * n);
        let mut ret = vec![vec![0; n]; m];

        for pos in 0..(m * n) {
            let tmp = (pos + m * n - k) % (m * n);
            ret[pos / n][pos % n] = grid[tmp / n][tmp % n];
        }

        ret
    }
}
