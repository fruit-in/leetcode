impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row_min = vec![100001; m];
        let mut col_max = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                row_min[i] = row_min[i].min(matrix[i][j]);
                col_max[j] = col_max[j].max(matrix[i][j]);
            }
        }

        row_min.drain(..).filter(|x| col_max.contains(x)).collect()
    }
}
