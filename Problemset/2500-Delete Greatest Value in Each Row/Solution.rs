impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();
        let mut ret = 0;

        grid.iter_mut().for_each(|row| row.sort_unstable());

        for col in 0..n {
            ret += (0..m).map(|row| grid[row][col]).max().unwrap();
        }

        ret
    }
}
