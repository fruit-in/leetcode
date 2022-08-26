impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut count_neg_even = true;
        let mut min_abs = i32::MAX;
        let mut ret = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix.len() {
                if matrix[i][j] < 0 {
                    count_neg_even = !count_neg_even;
                }
                min_abs = min_abs.min(matrix[i][j].abs());
                ret += matrix[i][j].abs() as i64;
            }
        }

        if !count_neg_even {
            ret -= 2 * min_abs as i64;
        }

        ret
    }
}
