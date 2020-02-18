impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let row0 = matrix[0].contains(&0);
        let col0 = matrix.iter().map(|v| v[0]).any(|x| x == 0);
        let m = matrix.len();
        let n = matrix[0].len();

        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        for i in 1..m {
            if matrix[i][0] == 0 {
                for j in 1..n {
                    matrix[i][j] = 0;
                }
            }
        }
        for j in 1..n {
            if matrix[0][j] == 0 {
                for i in 1..m {
                    matrix[i][j] = 0;
                }
            }
        }

        if col0 {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
        if row0 {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
    }
}
