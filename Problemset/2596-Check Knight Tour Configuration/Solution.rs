impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        let mut movements = vec![(0, 0); grid.len().pow(2)];

        for row in 0..grid.len() {
            for col in 0..grid.len() {
                movements[grid[row][col] as usize] = (row as i32, col as i32);
            }
        }

        if movements[0] != (0, 0) {
            return false;
        }

        for i in 1..movements.len() {
            let x = (movements[i].0 - movements[i - 1].0).abs();
            let y = (movements[i].1 - movements[i - 1].1).abs();

            if x * y != 2 {
                return false;
            }
        }

        true
    }
}
