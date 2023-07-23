impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_count = vec![0; grid.len()];
        let mut col_count = vec![0; grid[0].len()];
        let mut ret = 0;

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    row_count[r] += 1;
                    col_count[c] += 1;
                }
            }
        }

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 && (row_count[r] > 1 || col_count[c] > 1) {
                    ret += 1;
                }
            }
        }

        ret
    }
}
