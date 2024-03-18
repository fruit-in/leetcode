# 956. 最高的广告牌
你正在安装一个广告牌，并希望它高度最大。这块广告牌将有两个钢制支架，两边各一个。每个钢支架的高度必须相等。

你有一堆可以焊接在一起的钢筋 `rods`。举个例子，如果钢筋的长度为 `1`、`2` 和 `3`，则可以将它们焊接在一起形成长度为 `6` 的支架。

返回 *广告牌的最大可能安装高度* 。如果没法安装广告牌，请返回 `0` 。

#### 示例 1:
<pre>
<strong>输入:</strong> rods = [1,2,3,6]
<strong>输出:</strong> 6
<strong>解释:</strong> 我们有两个不相交的子集 {1,2,3} 和 {6}，它们具有相同的和 sum = 6。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> rods = [1,2,3,4,5,6]
<strong>输出:</strong> 10
<strong>解释:</strong> 我们有两个不相交的子集 {2,3,5} 和 {4,6}，它们具有相同的和 sum = 10。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> rods = [1,2]
<strong>输出:</strong> 0
<strong>解释:</strong> 没法安装广告牌，所以返回 0。
</pre>

#### 提示:
* `1 <= rods.length <= 20`
* `1 <= rods[i] <= 1000`
* `sum(rods[i]) <= 5000`

## 题解 (Rust)

### 1. 题解
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
