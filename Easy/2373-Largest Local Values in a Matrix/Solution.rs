impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut max_local = vec![vec![0; n - 2]; n - 2];

        for i in 0..n - 2 {
            for j in 0..n - 2 {
                for a in 0..3 {
                    for b in 0..3 {
                        max_local[i][j] = max_local[i][j].max(grid[i + a][j + b]);
                    }
                }
            }
        }

        max_local
    }
}
