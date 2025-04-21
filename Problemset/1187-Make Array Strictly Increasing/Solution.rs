impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        arr2.sort_unstable();
        arr2.dedup();

        let mut dp = vec![vec![i32::MAX; arr2.len() + 1]; arr1.len()];
        dp[0][arr2.len()] = 0;
        if arr2[0] < arr1[0] {
            dp[0][0] = 1;
        }

        for i in 1..arr1.len() {
            if arr1[i] > arr1[i - 1] {
                dp[i][arr2.len()] = dp[i - 1][arr2.len()];
            }

            for j in 0..arr2.len() {
                if arr2[j] < arr1[i] {
                    dp[i][arr2.len()] = dp[i][arr2.len()].min(dp[i - 1][j]);
                }
                if arr2[j] > arr1[i - 1] {
                    dp[i][j] = dp[i][j].min(dp[i - 1][arr2.len()].saturating_add(1));
                }
                if j < arr2.len() - 1 {
                    dp[i][j + 1] = dp[i][j + 1].min(dp[i - 1][j].saturating_add(1));
                }
            }
        }

        *dp[arr1.len() - 1]
            .iter()
            .filter(|&&x| x != i32::MAX)
            .min()
            .unwrap_or(&-1)
    }
}
