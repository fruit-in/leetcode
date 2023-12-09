use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut matrix = matrix;
        let mut count = HashMap::new();
        let mut ret = 0;

        for x1 in 0..m {
            for x2 in x1..m {
                count.insert((x1, x2, 0), 1);
            }
        }

        for x2 in 0..m {
            for y2 in 0..n {
                if x2 > 0 {
                    matrix[x2][y2] += matrix[x2 - 1][y2];
                }
                if y2 > 0 {
                    matrix[x2][y2] += matrix[x2][y2 - 1];
                }
                if x2 > 0 && y2 > 0 {
                    matrix[x2][y2] -= matrix[x2 - 1][y2 - 1];
                }

                for x1 in 0..=x2 {
                    let mut diff = matrix[x2][y2];
                    if x1 > 0 {
                        diff -= matrix[x1 - 1][y2];
                    }
                    ret += *count.get(&(x1, x2, diff - target)).unwrap_or(&0);
                    *count.entry((x1, x2, diff)).or_insert(0) += 1;
                }
            }
        }

        ret
    }
}
