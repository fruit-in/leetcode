impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut matrix = matrix
            .iter()
            .map(|row| row.iter().map(|&c| c as i32 - 48).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut ret = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 1 && i > 0 && j > 0 {
                    matrix[i][j] += matrix[i - 1][j - 1]
                        .min(matrix[i][j - 1])
                        .min(matrix[i - 1][j]);
                }
                ret = ret.max(matrix[i][j]);
            }
        }

        ret * ret
    }
}
