# 2218. Maximum Value of K Coins From Piles
There are `n` **piles** of coins on a table. Each pile consists of a **positive number** of coins of assorted denominations.

In one move, you can choose any coin on **top** of any pile, remove it, and add it to your wallet.

Given a list `piles`, where `piles[i]` is a list of integers denoting the composition of the <code>i<sup>th</sup></code> pile from **top to bottom**, and a positive integer `k`, return *the **maximum total value** of coins you can have in your wallet if you choose **exactly*** `k` *coins optimally*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/11/09/e1.png)
<pre>
<strong>Input:</strong> piles = [[1,100,3],[7,8,9]], k = 2
<strong>Output:</strong> 101
<strong>Explanation:</strong>
The above diagram shows the different ways we can choose k coins.
The maximum total we can obtain is 101.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> piles = [[100],[100],[100],[100],[100],[100],[1,1,1,1,1,1,700]], k = 7
<strong>Output:</strong> 706
<strong>Explanation:</strong>
The maximum total can be obtained if we choose all coins from the last pile.
</pre>

#### Constraints:
* `n == piles.length`
* `1 <= n <= 1000`
* <code>1 <= piles[i][j] <= 10<sup>5</sup></code>
* `1 <= k <= sum(piles[i].length) <= 2000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = piles.len();
        let mut dp = vec![vec![0; k + 1]; n + 1];

        for i in 0..n {
            for j in 0..=k {
                let mut total = 0;

                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
                for m in 0..piles[i].len().min(k - j) {
                    total += piles[i][m];
                    dp[i + 1][j + m + 1] = dp[i + 1][j + m + 1].max(total + dp[i][j]);
                }
            }
        }

        dp[n][k]
    }
}
```
