# 879. 盈利计划
集团里有 `n` 名员工，他们可以完成各种各样的工作创造利润。

第 `i` 种工作会产生 `profit[i]` 的利润，它要求 `group[i]` 名成员共同参与。如果成员参与了其中一项工作，就不能参与另一项工作。

工作的任何至少产生 `minProfit` 利润的子集称为 **盈利计划** 。并且工作的成员总数最多为 `n` 。

有多少种计划可以选择？因为答案很大，所以 **返回结果模** `10^9 + 7` **的值**。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 5, minProfit = 3, group = [2,2], profit = [2,3]
<strong>输出:</strong> 2
<strong>解释:</strong> 至少产生 3 的利润，该集团可以完成工作 0 和工作 1 ，或仅完成工作 1 。
总的来说，有两种计划。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 10, minProfit = 5, group = [2,3,5], profit = [6,7,8]
<strong>输出:</strong> 7
<strong>解释:</strong> 至少产生 5 的利润，只要完成其中一种工作就行，所以该集团可以完成任何工作。
有 7 种可能的计划：(0)，(1)，(2)，(0,1)，(0,2)，(1,2)，以及 (0,1,2) 。
</pre>

#### 提示:
* `1 <= n <= 100`
* `0 <= minProfit <= 100`
* `1 <= group.length <= 100`
* `1 <= group[i] <= 100`
* `profit.length == group.length`
* `0 <= profit[i] <= 100`

## 题解 (Rust)

### 1. 题解
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
