# 1621. 大小为 K 的不重叠线段的数目
给你一维空间的 `n` 个点，其中第 `i` 个点（编号从 `0` 到 `n-1`）位于 `x = i` 处，请你找到 **恰好** `k` **个不重叠** 线段且每个线段至少覆盖两个点的方案数。线段的两个端点必须都是 **整数坐标** 。这 `k` 个线段不需要全部覆盖全部 `n` 个点，且它们的端点 可以 重合。

请你返回 `k` 个不重叠线段的方案数。由于答案可能很大，请将结果对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/07/ex1.png)
<pre>
<strong>输入:</strong> n = 4, k = 2
<strong>输出:</strong> 5
<strong>解释:</strong> 如图所示，两个线段分别用红色和蓝色标出。
上图展示了 5 种不同的方案 {(0,2),(2,3)}，{(0,1),(1,3)}，{(0,1),(2,3)}，{(1,2),(2,3)}，{(0,1),(1,2)} 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, k = 1
<strong>输出:</strong> 3
<strong>解释:</strong> 总共有 3 种不同的方案 {(0,1)}, {(0,2)}, {(1,2)} 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 30, k = 7
<strong>输出:</strong> 796297179
<strong>解释:</strong> 画 7 条线段的总方案数为 3796297200 种。将这个数对 10<sup>9</sup> + 7 取余得到 796297179 。

#### 示例 4:
<pre>
<strong>输入:</strong> n = 5, k = 3
<strong>输出:</strong> 7

#### 示例 5:
<pre>
<strong>输入:</strong> n = 3, k = 2
<strong>输出:</strong> 1
</pre>

#### 提示:
* `2 <= n <= 1000`
* `1 <= k <= n-1`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def numberOfSets(self, n: int, k: int) -> int:
        dp = [[0] * (n + 1) for _ in range(k + 1)]

        for j in range(2, n + 1):
            dp[1][j] = dp[1][j - 1] + j - 1

        for i in range(2, k + 1):
            for j in range(i + 1, n + 1):
                dp[i][j] = dp[i - 1][j - 1] + 2 * dp[i][j - 1] - dp[i][j - 2]

        return dp[k][n] % 1000000007
```
