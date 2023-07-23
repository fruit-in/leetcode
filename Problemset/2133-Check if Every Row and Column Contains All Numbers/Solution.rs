impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();

        for i in 0..n {
            let mut valid_r = vec![false; n];
            let mut valid_c = vec![false; n];

            for j in 0..n {
                valid_r[matrix[i][j] as usize - 1] = true;
                valid_c[matrix[j][i] as usize - 1] = true;
            }

            if !(0..n).all(|k| valid_r[k] && valid_c[k]) {
                return false;
            }
        }

        true
    }
}
