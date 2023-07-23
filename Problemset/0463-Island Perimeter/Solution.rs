impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    perimeter += 2;
                    if i > 0 {
                        perimeter -= grid[i - 1][j];
                    }
                    if j > 0 {
                        perimeter -= grid[i][j - 1];
                    }
                }
            }
        }

        2 * perimeter
    }
}
