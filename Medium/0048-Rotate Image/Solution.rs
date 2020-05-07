impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for i in 0..n {
            for j in 0..i {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }

        for i in 0..n {
            for j in 0..(n / 2) {
                matrix[i].swap(j, n - 1 - j);
            }
        }
    }
}
