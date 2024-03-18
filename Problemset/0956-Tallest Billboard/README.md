# 956. Tallest Billboard
You are installing a billboard and want it to have the largest height. The billboard will have two steel supports, one on each side. Each steel support must be an equal height.

You are given a collection of `rods` that can be welded together. For example, if you have rods of lengths `1`, `2`, and `3`, you can weld them together to make a support of length `6`.

Return *the largest possible height of your billboard installation*. If you cannot support the billboard, return `0`.

#### Example 1:
<pre>
<strong>Input:</strong> rods = [1,2,3,6]
<strong>Output:</strong> 6
<strong>Explanation:</strong> We have two disjoint subsets {1,2,3} and {6}, which have the same sum = 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> rods = [1,2,3,4,5,6]
<strong>Output:</strong> 10
<strong>Explanation:</strong> We have two disjoint subsets {2,3,5} and {4,6}, which have the same sum = 10.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> rods = [1,2]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The billboard cannot be supported, so we return 0.
</pre>

#### Constraints:
* `1 <= rods.length <= 20`
* `1 <= rods[i] <= 1000`
* `sum(rods[i]) <= 5000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let rods = rods.into_iter().map(|r| r as usize).collect::<Vec<_>>();
        let limit = rods.iter().sum::<usize>() / 2;
        let mut dp = vec![vec![0; limit + 1]; rods.len()];

        if rods[0] <= limit {
            dp[0][rods[0]] = rods[0];
        }

        for i in 1..rods.len() {
            for j in 0..=limit {
                if j == 0 || dp[i - 1][j] != 0 {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j]);
                    if (j + rods[i]).max(dp[i - 1][j] + rods[i]) <= limit {
                        dp[i][j + rods[i]] = dp[i][j + rods[i]].max(dp[i - 1][j] + rods[i]);
                    }
                    if j >= rods[i] {
                        dp[i][j - rods[i]] = dp[i][j - rods[i]].max(dp[i - 1][j]);
                    } else if dp[i - 1][j] + rods[i] - j <= limit {
                        dp[i][rods[i] - j] = dp[i][rods[i] - j].max(dp[i - 1][j] + rods[i] - j);
                    }
                }
            }
        }

        dp.last().unwrap()[0] as i32
    }
}
```
