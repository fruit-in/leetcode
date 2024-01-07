impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut prefix_sum = vec![vec![0; cols]; rows];
        let mut ret = 0;

        for r in 0..rows {
            for c in 0..cols {
                if matrix[r][c] == '0' {
                    continue;
                }

                prefix_sum[r][c] = 1;

                if c > 0 {
                    prefix_sum[r][c] += prefix_sum[r][c - 1];
                }

                let mut min_w = i32::MAX;

                for h in 1..=r + 1 {
                    if prefix_sum[r + 1 - h][c] == 0 {
                        break;
                    }

                    min_w = min_w.min(prefix_sum[r + 1 - h][c]);
                    ret = ret.max(min_w * h as i32);
                }
            }
        }

        ret
    }
}
