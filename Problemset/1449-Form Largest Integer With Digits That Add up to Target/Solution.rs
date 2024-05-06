impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let target = target as usize;
        let cost = cost.iter().map(|&x| x as usize).collect::<Vec<_>>();
        let mut dp = vec![[-1; 10]; target + 1];
        dp[0] = [0; 10];

        for i in 0..=target {
            if dp[i][0] == -1 {
                continue;
            }

            for j in 0..9 {
                let mut count = dp[i];
                count[9 - j] += 1;
                count[0] += 1;

                if i + cost[j] <= target && dp[i + cost[j]] < count {
                    dp[i + cost[j]] = count;
                }
            }
        }

        if dp[target][0] == -1 {
            return "0".to_string();
        }

        (0..9)
            .rev()
            .map(|i| vec![std::char::from_u32(49 + i as u32).unwrap(); dp[target][9 - i] as usize])
            .flatten()
            .collect()
    }
}
