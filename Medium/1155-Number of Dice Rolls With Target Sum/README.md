# 1155. Number of Dice Rolls With Target Sum
You have `d` dice, and each die has `f` faces numbered `1, 2, ..., f`.

Return the number of possible ways (out of <code>f<sup>d</sup></code> total ways) **modulo `10^9 + 7`** to roll the dice so the sum of the face up numbers equals `target`.

#### Example 1:
<pre>
<strong>Input:</strong> d = 1, f = 6, target = 3
<strong>Output:</strong> 1
<strong>Explanation:</strong>
You throw one die with 6 faces.  There is only one way to get a sum of 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> d = 2, f = 6, target = 7
<strong>Output:</strong> 6
<strong>Explanation:</strong>
You throw two dice, each with 6 faces.  There are 6 ways to get a sum of 7:
1+6, 2+5, 3+4, 4+3, 5+2, 6+1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> d = 2, f = 5, target = 10
<strong>Output:</strong> 1
<strong>Explanation:</strong>
You throw two dice, each with 5 faces.  There is only one way to get a sum of 10: 5+5.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> d = 1, f = 2, target = 3
<strong>Output:</strong> 0
<strong>Explanation:</strong>
You throw one die with 2 faces.  There is no way to get a sum of 3.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> d = 30, f = 30, target = 500
<strong>Output:</strong> 222616187
<strong>Explanation:</strong>
The answer must be returned modulo 10^9 + 7.
</pre>

#### Constraints:
* `1 <= d, f <= 30`
* `1 <= target <= 1000`

## Solutions (Python)

### 1. Dynamic Programming
```Python
class Solution:
    def numRollsToTarget(self, d: int, f: int, target: int) -> int:
        if target < d or target > d * f:
            return 0

        dp = [[0] * (target + 1) for _ in range(d + 1)]
        dp[0][0] = 1

        for i in range(d):
            for j in range(max(i, target - (d - i) * f), min(i * f, target - d + i) + 1):
                for k in range(1, min(f, target - j) + 1):
                    dp[i + 1][j + k] += dp[i][j]
                    dp[i + 1][j + k] %= 1_000_000_007

        return dp[d][target]
```

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn num_rolls_to_target(d: i32, f: i32, target: i32) -> i32 {
        if target < d || target > d * f {
            return 0;
        }

        let d = d as usize;
        let f = f as usize;
        let target = target as usize;
        let mut dp = vec![vec![0; target + 1]; d + 1];
        dp[0][0] = 1;

        for i in 0..d {
            for j in i.max(target.saturating_sub((d - i) * f))..=(i * f).min(target - d + i) {
                for k in 1..=f.min(target - j) {
                    dp[i + 1][j + k] += dp[i][j];
                    dp[i + 1][j + k] %= 1_000_000_007;
                }
            }
        }

        dp[d][target]
    }
}
```
