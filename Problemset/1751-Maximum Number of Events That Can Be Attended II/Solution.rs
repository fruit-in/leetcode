impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![[0, 0]]; k + 1];
        let mut ret = 0;

        events.sort_unstable_by_key(|e| e[1]);

        for i in 0..events.len() {
            let (start, end, value) = (events[i][0], events[i][1], events[i][2]);

            for j in (0..k.min(i + 1)).rev() {
                let value_sum =
                    dp[j][dp[j].binary_search(&[start - 1, i32::MAX]).unwrap_err() - 1][1] + value;

                if dp[j + 1].last().unwrap()[1] < value_sum {
                    if dp[j + 1].last().unwrap()[0] == end {
                        dp[j + 1].pop();
                    }
                    dp[j + 1].push([end, value_sum]);
                    ret = ret.max(value_sum);
                }
            }
        }

        ret
    }
}
