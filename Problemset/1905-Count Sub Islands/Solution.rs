impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let m = grid1.len();
        let n = grid1[0].len();
        let mut grid2 = grid2;
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 0 {
                    continue;
                }

                let mut cells = vec![(i, j)];
                let mut is_sub = true;
                grid2[i][j] = 0;

                while let Some((i, j)) = cells.pop() {
                    is_sub &= grid1[i][j] == 1;

                    if i > 0 && grid2[i - 1][j] == 1 {
                        cells.push((i - 1, j));
                        grid2[i - 1][j] = 0;
                    }
                    if i + 1 < m && grid2[i + 1][j] == 1 {
                        cells.push((i + 1, j));
                        grid2[i + 1][j] = 0;
                    }
                    if j > 0 && grid2[i][j - 1] == 1 {
                        cells.push((i, j - 1));
                        grid2[i][j - 1] = 0;
                    }
                    if j + 1 < n && grid2[i][j + 1] == 1 {
                        cells.push((i, j + 1));
                        grid2[i][j + 1] = 0;
                    }
                }

                ret += is_sub as i32;
            }
        }

        ret
    }
}
