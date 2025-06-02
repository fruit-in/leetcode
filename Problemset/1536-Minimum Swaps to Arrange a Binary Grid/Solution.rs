impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut zeros = vec![0; n];
        let mut ret = 0;

        for i in 0..n {
            for j in (0..n).rev() {
                if grid[i][j] == 0 {
                    zeros[i] += 1;
                } else {
                    break;
                }
            }
        }

        for i in 0..n {
            if let Some(j) = zeros.iter().position(|&x| x >= n - 1 - i) {
                zeros.remove(j);
                ret += j;
            } else {
                return -1;
            }
        }

        ret as i32
    }
}
