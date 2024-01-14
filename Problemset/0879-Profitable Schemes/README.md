# 879. Profitable Schemes
There is a group of `n` members, and a list of various crimes they could commit. The <code>i<sup>th</sup></code> crime generates a `profit[i]` and requires `group[i]` members to participate in it. If a member participates in one crime, that member can't participate in another crime.

Let's call a **profitable scheme** any subset of these crimes that generates at least `minProfit` profit, and the total number of members participating in that subset of crimes is at most `n`.

Return the number of schemes that can be chosen. Since the answer may be very large, **return it modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 5, minProfit = 3, group = [2,2], profit = [2,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> To make a profit of at least 3, the group could either commit crimes 0 and 1, or just crime 1.
In total, there are 2 schemes.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 10, minProfit = 5, group = [2,3,5], profit = [6,7,8]
<strong>Output:</strong> 7
<strong>Explanation:</strong> To make a profit of at least 5, the group could commit any crimes, as long as they commit one.
There are 7 possible schemes: (0), (1), (2), (0,1), (0,2), (1,2), and (0,1,2).
</pre>

#### Constraints:
* `1 <= n <= 100`
* `0 <= minProfit <= 100`
* `1 <= group.length <= 100`
* `1 <= group[i] <= 100`
* `profit.length == group.length`
* `0 <= profit[i] <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = n as usize;
        let min_profit = min_profit as usize;
        let mut dp = vec![vec![0; n + 1]; min_profit + 1];
        dp[0][n] = 1;

        for i in 0..group.len() {
            for p in (0..=min_profit).rev() {
                for m in group[i] as usize..=n {
                    dp[min_profit.min(p + profit[i] as usize)][m - group[i] as usize] += dp[p][m];
                    dp[min_profit.min(p + profit[i] as usize)][m - group[i] as usize] %=
                        1_000_000_007;
                }
            }
        }

        dp[min_profit]
            .iter()
            .fold(0, |acc, x| (acc + x) % 1_000_000_007)
    }
}
```
