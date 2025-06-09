impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let (m, n) = (points.len(), points[0].len());
        let mut dp = vec![0; n];

        for r in 0..m {
            let mut tmp = vec![0; n];
            let (mut x, mut i) = (dp[0], 0);
            let (mut y, mut j) = (dp[n - 1], n - 1);

            for (c1, c2) in (0..n).zip((0..n).rev()) {
                if x - dp[c1] < (c1 - i) as i64 {
                    x = dp[c1];
                    i = c1;
                }
                if y - dp[c2] < (j - c2) as i64 {
                    y = dp[c2];
                    j = c2;
                }

                tmp[c1] = tmp[c1].max(points[r][c1] as i64 + x - (c1 - i) as i64);
                tmp[c2] = tmp[c2].max(points[r][c2] as i64 + y - (j - c2) as i64);
            }

            dp = tmp;
        }

        *dp.iter().max().unwrap()
    }
}
