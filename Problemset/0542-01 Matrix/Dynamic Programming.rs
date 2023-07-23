impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![10000; matrix[0].len()]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                match matrix[i][j] {
                    0 => ret[i][j] = 0,
                    _ => {
                        if i > 0 {
                            ret[i][j] = ret[i - 1][j] + 1;
                        }
                        if j > 0 {
                            ret[i][j] = ret[i][j].min(ret[i][j - 1] + 1);
                        }
                    }
                }
            }
        }

        for i in (0..matrix.len()).rev() {
            for j in (0..matrix[0].len()).rev() {
                if matrix[i][j] == 1 {
                    if i + 1 < matrix.len() {
                        ret[i][j] = ret[i][j].min(ret[i + 1][j] + 1);
                    }
                    if j + 1 < matrix[0].len() {
                        ret[i][j] = ret[i][j].min(ret[i][j + 1] + 1);
                    }
                }
            }
        }

        ret
    }
}
