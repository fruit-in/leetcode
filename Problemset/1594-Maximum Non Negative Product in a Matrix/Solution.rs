impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut products = vec![vec![[1, -1]; n]; m];
        products[0][0] = [grid[0][0].min(1) as i64, grid[0][0].max(-1) as i64];

        for i in 0..m {
            for j in 0..n {
                if i + 1 < m {
                    if grid[i + 1][j] > 0 {
                        products[i + 1][j][0] =
                            products[i + 1][j][0].min(products[i][j][0] * grid[i + 1][j] as i64);
                        products[i + 1][j][1] =
                            products[i + 1][j][1].max(products[i][j][1] * grid[i + 1][j] as i64);
                    } else if grid[i + 1][j] < 0 {
                        products[i + 1][j][0] =
                            products[i + 1][j][0].min(products[i][j][1] * grid[i + 1][j] as i64);
                        products[i + 1][j][1] =
                            products[i + 1][j][1].max(products[i][j][0] * grid[i + 1][j] as i64);
                    } else {
                        products[i + 1][j] = [0, 0];
                    }
                }
                if j + 1 < n {
                    if grid[i][j + 1] > 0 {
                        products[i][j + 1][0] =
                            products[i][j + 1][0].min(products[i][j][0] * grid[i][j + 1] as i64);
                        products[i][j + 1][1] =
                            products[i][j + 1][1].max(products[i][j][1] * grid[i][j + 1] as i64);
                    } else if grid[i][j + 1] < 0 {
                        products[i][j + 1][0] =
                            products[i][j + 1][0].min(products[i][j][1] * grid[i][j + 1] as i64);
                        products[i][j + 1][1] =
                            products[i][j + 1][1].max(products[i][j][0] * grid[i][j + 1] as i64);
                    } else {
                        products[i][j + 1] = [0, 0];
                    }
                }
            }
        }

        (products[m - 1][n - 1][1] % 1_000_000_007) as i32
    }
}
