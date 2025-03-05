impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut dp = vec![[0, i32::MIN]; k + 1];
        let mut ret = 0;

        for i in 0..prices.len() {
            let mut tmp = dp.clone();

            for j in 0..=i.min(k) {
                if j < k {
                    tmp[j + 1][1] = tmp[j + 1][1].max(dp[j][0] - prices[i]);
                }
                tmp[j][0] = tmp[j][0].max(dp[j][1] + prices[i]);
                ret = ret.max(tmp[j][0]);
            }

            dp = tmp;
        }

        ret
    }
}
