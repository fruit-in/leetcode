# 2400. Number of Ways to Reach a Position After Exactly k Steps
You are given two **positive** integers `startPos` and `endPos`. Initially, you are standing at position `startPos` on an **infinite** number line. With one step, you can move either one position to the left, or one position to the right.

Given a positive integer `k`, return *the number of **different** ways to reach the position* `endPos` *starting from* `startPos`, *such that you perform **exactly*** `k` *steps*. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

Two ways are considered different if the order of the steps made is not exactly the same.

**Note** that the number line includes negative integers.

#### Example 1:
<pre>
<strong>Input:</strong> startPos = 1, endPos = 2, k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can reach position 2 from 1 in exactly 3 steps in three ways:
- 1 -> 2 -> 3 -> 2.
- 1 -> 2 -> 1 -> 2.
- 1 -> 0 -> 1 -> 2.
It can be proven that no other way is possible, so we return 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> startPos = 2, endPos = 5, k = 10
<strong>Output:</strong> 0
<strong>Explanation:</strong> It is impossible to reach position 5 from position 2 in exactly 10 steps.
</pre>

#### Constraints:
* `1 <= startPos, endPos, k <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        if start_pos - k > end_pos || start_pos + k < end_pos {
            return 0;
        }

        let mut dp = vec![vec![0; 2 * k as usize + 1]; k as usize + 1];
        dp[0][k as usize] = 1;

        for i in 0..k as usize {
            for j in k as usize - i..=k as usize + i {
                dp[i + 1][j - 1] = (dp[i + 1][j - 1] + dp[i][j]) % 1_000_000_007;
                dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j]) % 1_000_000_007;
            }
        }

        dp[k as usize][(end_pos - start_pos + k) as usize]
    }
}
```
