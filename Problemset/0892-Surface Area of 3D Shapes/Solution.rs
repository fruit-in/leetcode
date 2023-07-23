impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut area = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] > 0 {
                    area += grid[i][j] * 2 + 1;
                    if i > 0 {
                        area -= grid[i - 1][j].min(grid[i][j]);
                    }
                    if j > 0 {
                        area -= grid[i][j - 1].min(grid[i][j]);
                    }
                }
            }
        }

        area * 2
    }
}
