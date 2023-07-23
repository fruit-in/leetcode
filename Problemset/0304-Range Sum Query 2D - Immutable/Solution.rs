struct NumMatrix {
    mat: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut mat = matrix.clone();

        for r in 0..matrix.len() {
            let mut row_sum = 0;

            for c in 0..matrix[0].len() {
                if r > 0 {
                    mat[r][c] += mat[r - 1][c];
                }
                mat[r][c] += row_sum;
                row_sum += matrix[r][c];
            }
        }

        Self { mat }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;

        self.mat[row2][col2]
            + match (row1 > 0, col1 > 0) {
                (false, false) => 0,
                (true, false) => -self.mat[row1 - 1][col2],
                (false, true) => -self.mat[row2][col1 - 1],
                _ => {
                    self.mat[row1 - 1][col1 - 1]
                        - self.mat[row1 - 1][col2]
                        - self.mat[row2][col1 - 1]
                }
            }
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
