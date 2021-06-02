impl Solution {
    pub fn kth_largest_value(mut matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut vals = vec![];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i > 0 {
                    matrix[i][j] ^= matrix[i - 1][j];
                }
                if j > 0 {
                    matrix[i][j] ^= matrix[i][j - 1];
                }
                if i > 0 && j > 0 {
                    matrix[i][j] ^= matrix[i - 1][j - 1];
                }
                vals.push(matrix[i][j]);
            }
        }

        vals.sort_unstable_by(|a, b| b.cmp(a));

        vals[k as usize - 1]
    }
}
