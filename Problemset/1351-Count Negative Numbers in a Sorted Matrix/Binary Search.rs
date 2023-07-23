impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut pos = 0;

        for row in grid {
            match row.binary_search_by(|probe| 0.cmp(&probe)) {
                Ok(i) => pos += i + 1,
                Err(i) => pos += i,
            }
        }

        (m * n - pos) as i32
    }
}
