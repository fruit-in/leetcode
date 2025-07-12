impl Solution {
    pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![i32::MAX; target + 1];
        dp[0] = 0;

        for i in 0..mat.len() {
            let mut tmp = vec![i32::MAX; target + 1];

            for j in 0..mat[0].len() {
                for k in 0..=target {
                    tmp[target.min(k + mat[i][j] as usize)] = tmp
                        [target.min(k + mat[i][j] as usize)]
                    .min(dp[k].saturating_add(mat[i][j]));
                }
            }

            dp = tmp;
        }

        dp.iter().map(|&x| (x - target as i32).abs()).min().unwrap()
    }
}
