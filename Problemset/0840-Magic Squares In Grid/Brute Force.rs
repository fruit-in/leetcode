impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        for i in 2..grid.len() {
            for j in 2..grid[0].len() {
                if grid[i - 1][j - 1] == 5 &&
                   grid[i - 2][j - 2] + grid[i][j] == 10 &&
                   grid[i - 2][j] + grid[i][j - 2] == 10 &&
                   grid[i - 1][j - 2] + grid[i - 1][j] == 10 &&
                   grid[i - 2][j - 1] + grid[i][j - 1] == 10 &&
                   grid[i - 2][j - 2] + grid[i - 2][j - 1] + grid[i - 2][j] == 15 &&
                   grid[i - 2][j - 2] + grid[i - 1][j - 2] + grid[i][j - 2] == 15 {

                    let mut nums = Vec::new();
                    nums.extend_from_slice(&grid[i - 2][(j - 2)..(j + 1)]);
                    nums.extend_from_slice(&grid[i - 1][(j - 2)..(j + 1)]);
                    nums.extend_from_slice(&grid[i][(j - 2)..(j + 1)]);
                    nums.sort_unstable();

                    if nums == vec![1, 2, 3, 4, 5, 6, 7, 8, 9] {
                        ret += 1;
                    }
                }
            }
        }

        ret
    }
}
