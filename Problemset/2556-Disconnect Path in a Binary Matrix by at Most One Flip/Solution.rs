impl Solution {
    pub fn is_possible_to_cut_path(mut grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut connect = vec![vec![false; n]; m];
        connect[0][0] = true;

        for r in 0..m {
            for c in 0..n {
                connect[r][c] |= grid[r][c] == 1
                    && ((r > 0 && connect[r - 1][c]) || (c > 0 && connect[r][c - 1]));
            }
        }

        if !connect[m - 1][n - 1] {
            return true;
        }

        let mut r = m - 1;
        let mut c = n - 1;

        while r > 0 || c > 0 {
            if r > 0 && connect[r - 1][c] {
                r -= 1;
            } else {
                c -= 1;
            }
            grid[r][c] = 0;
        }

        connect = vec![vec![false; n]; m];
        connect[0][0] = true;

        for r in 0..m {
            for c in 0..n {
                connect[r][c] |= grid[r][c] == 1
                    && ((r > 0 && connect[r - 1][c]) || (c > 0 && connect[r][c - 1]));
            }
        }

        !connect[m - 1][n - 1]
    }
}
