# 1621. Number of Sets of K Non-Overlapping Line Segments
Given `n` points on a 1-D plane, where the <code>i<sup>th</sup></code> point (from `0` to `n-1`) is at `x = i`, find the number of ways we can draw **exactly** `k` **non-overlapping** line segments such that each segment covers two or more points. The endpoints of each segment must have **integral coordinates**. The `k` line segments **do not** have to cover all `n` points, and they are **allowed** to share endpoints.

Return *the number of ways we can draw* `k` *non-overlapping line segments*. Since this number can be huge, return it modulo <code>10<sup>9</sup> + 7</code>.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/07/ex1.png)
<pre>
<strong>Input:</strong> n = 4, k = 2
<strong>Output:</strong> 5
<strong>Explanation:</strong> The two line segments are shown in red and blue.
The image above shows the 5 different ways {(0,2),(2,3)}, {(0,1),(1,3)}, {(0,1),(2,3)}, {(1,2),(2,3)}, {(0,1),(1,2)}.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, k = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong> The 3 ways are {(0,1)}, {(0,2)}, {(1,2)}.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 30, k = 7
<strong>Output:</strong> 796297179
<strong>Explanation:</strong> The total number of possible ways to draw 7 line segments is 3796297200. Taking this number modulo 10<sup>9</sup> + 7 gives us 796297179.
</pre>

#### Constraints:
* `2 <= n <= 1000`
* `1 <= k <= n-1`

## Solutions (Python)

### 1. Solution
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
