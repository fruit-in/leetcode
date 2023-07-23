impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let mut r0 = true;
        let mut r1 = true;
        let mut r2 = true;
        let mut r3 = true;
        let n = mat.len();

        for r in 0..n {
            for c in 0..n {
                r0 &= mat[r][c] == target[r][c];
                r1 &= mat[r][c] == target[c][n - 1 - r];
                r2 &= mat[r][c] == target[n - 1 - r][n - 1 - c];
                r3 &= mat[r][c] == target[n - 1 - c][r];
            }
        }

        r0 || r1 || r2 || r3
    }
}
