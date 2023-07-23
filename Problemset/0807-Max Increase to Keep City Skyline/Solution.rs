impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_max = Vec::new();
        let mut col_max = Vec::new();
        let mut ret = 0;

        for i in 0..grid.len() {
            row_max.push(*grid[i].iter().max().unwrap());
            col_max.push(grid.iter().map(|v| v[i]).max().unwrap());
        }

        for i in 0..grid.len() {
            for j in 0..grid.len() {
                ret += row_max[i].min(col_max[j]) - grid[i][j];
            }
        }

        ret
    }
}
