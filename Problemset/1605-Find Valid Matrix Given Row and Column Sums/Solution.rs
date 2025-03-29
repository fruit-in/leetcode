impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut row = 0;
        let mut col = 0;
        let mut ret = vec![vec![0; col_sum.len()]; row_sum.len()];

        while row < row_sum.len() && col < col_sum.len() {
            if row_sum[row] < col_sum[col] {
                ret[row][col] = row_sum[row];
                col_sum[col] -= row_sum[row];
                row += 1;
            } else if row_sum[row] > col_sum[col] {
                ret[row][col] = col_sum[col];
                row_sum[row] -= col_sum[col];
                col += 1;
            } else {
                ret[row][col] = row_sum[row];
                row += 1;
                col += 1;
            }
        }

        ret
    }
}
