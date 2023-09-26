impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut grid = vec![vec![[1; 5]; n]; n];
        let mut ret = 0;

        for mine in &mines {
            grid[mine[0] as usize][mine[1] as usize] = [0; 5];
        }

        for x in 0..n {
            for y in 1..n {
                if grid[x][y][0] == 1 {
                    grid[x][y][1] += grid[x][y - 1][1];
                }
                if grid[y][x][0] == 1 {
                    grid[y][x][2] += grid[y - 1][x][2];
                }
            }
            for y in (0..n - 1).rev() {
                if grid[x][y][0] == 1 {
                    grid[x][y][3] += grid[x][y + 1][3];
                }
                if grid[y][x][0] == 1 {
                    grid[y][x][4] += grid[y + 1][x][4];
                }
            }
        }

        for x in 0..n {
            for y in 0..n {
                if grid[x][y][0] == 1 {
                    ret = ret.max(*grid[x][y][1..].iter().min().unwrap());
                }
            }
        }

        ret
    }
}
