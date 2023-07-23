impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let k = k as usize;
        let mut row_dp = vec![vec![0; n]; m];
        let mut answer = vec![vec![0; n]; m];

        for i in 0..m {
            row_dp[i][0] = mat[i][..=(k.min(n - 1))].iter().sum();
            for j in 1..n {
                row_dp[i][j] = row_dp[i][j - 1];
                if j - 1 >= k {
                    row_dp[i][j] -= mat[i][j - 1 - k];
                }
                if j + k < n {
                    row_dp[i][j] += mat[i][j + k];
                }
            }
        }

        for j in 0..n {
            answer[0][j] = row_dp[..=(k.min(m - 1))].iter().map(|v| v[j]).sum();
            for i in 1..m {
                answer[i][j] = answer[i - 1][j];
                if i - 1 >= k {
                    answer[i][j] -= row_dp[i - 1 - k][j];
                }
                if i + k < m {
                    answer[i][j] += row_dp[i + k][j];
                }
            }
        }

        answer
    }
}
