impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut mat = vec![vec![0; n]; n];

        for query in &queries {
            let (row1, col1, row2, col2) = (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                query[3] as usize,
            );

            mat[row2][col2] += 1;
            if row1 > 0 {
                mat[row1 - 1][col2] -= 1;
            }
            if col1 > 0 {
                mat[row2][col1 - 1] -= 1;
            }
            if row1 > 0 && col1 > 0 {
                mat[row1 - 1][col1 - 1] += 1;
            }
        }

        for row in (0..n).rev() {
            for col in (0..n).rev() {
                if row > 0 {
                    mat[row - 1][col] += mat[row][col];
                }
                if col > 0 {
                    mat[row][col - 1] += mat[row][col];
                }
                if row > 0 && col > 0 {
                    mat[row - 1][col - 1] -= mat[row][col];
                }
            }
        }

        mat
    }
}
