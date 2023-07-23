impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }

                let mut area = 0;
                let mut cells = vec![(i, j)];

                while let Some((i, j)) = cells.pop() {
                    if grid[i][j] == 0 {
                        continue;
                    }

                    area += 1;
                    grid[i][j] = 0;

                    if i > 0 && grid[i - 1][j] == 1 {
                        cells.push((i - 1, j));
                    }
                    if i < m - 1 && grid[i + 1][j] == 1 {
                        cells.push((i + 1, j));
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        cells.push((i, j - 1));
                    }
                    if j < n - 1 && grid[i][j + 1] == 1 {
                        cells.push((i, j + 1));
                    }
                }

                ret = ret.max(area);
            }
        }

        ret
    }
}
