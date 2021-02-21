# 1155. 掷骰子的N种方法
这里有 `d` 个一样的骰子，每个骰子上都有 `f` 个面，分别标号为 `1, 2, ..., f`。

我们约定：掷骰子的得到总点数为各骰子面朝上的数字的总和。

如果需要掷出的总点数为 `target`，请你计算出有多少种不同的组合情况（所有的组合情况总共有 `f^d` 种），**模 `10^9 + 7`** 后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> d = 1, f = 6, target = 3
<strong>输出:</strong> 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> d = 2, f = 6, target = 7
<strong>输出:</strong> 6
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> d = 2, f = 5, target = 10
<strong>输出:</strong> 1
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> d = 1, f = 2, target = 3
<strong>输出:</strong> 0
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> d = 30, f = 30, target = 500
<strong>输出:</strong> 222616187
</pre>

#### 提示:
* `1 <= d, f <= 30`
* `1 <= target <= 1000`

## 题解 (Python)

### 1. 动态规划
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

## 题解 (Rust)

### 1. 动态规划
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
