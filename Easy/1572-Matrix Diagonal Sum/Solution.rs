impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut ret = 0;

        for i in 0..n {
            ret += mat[i][i] + mat[i][n - 1 - i];
        }

        if n % 2 == 1 {
            ret -= mat[n / 2][n / 2];
        }

        ret
    }
}
