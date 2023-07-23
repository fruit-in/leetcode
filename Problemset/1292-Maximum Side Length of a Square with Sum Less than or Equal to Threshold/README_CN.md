# 1292. 元素和小于等于阈值的正方形的最大边长
给你一个大小为 `m x n` 的矩阵 `mat` 和一个整数阈值 `threshold`。

请你返回元素总和小于或等于阈值的正方形区域的最大边长；如果没有这样的正方形区域，则返回 **0** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/12/05/e1.png)
<pre>
<strong>输入:</strong> mat = [[1,1,3,2,4,3,2],[1,1,3,2,4,3,2],[1,1,3,2,4,3,2]], threshold = 4
<strong>输出:</strong> 2
<strong>解释:</strong> 总和小于或等于 4 的正方形的最大边长为 2，如图所示。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> mat = [[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2]], threshold = 1
<strong>输出:</strong> 0
</pre>

#### 提示:
* `m == mat.length`
* `n == mat[i].length`
* `1 <= m, n <= 300`
* <code>0 <= mat[i][j] <= 10<sup>4</sup></code>
* <code>0 <= threshold <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
import bisect


class Solution:
    def maxSideLength(self, mat: List[List[int]], threshold: int) -> int:
        m, n = len(mat), len(mat[0])
        prefixsum = [[0] * n for _ in range(m)]
        ret = 0

        for i in range(m):
            for j in range(n):
                prefixsum[i][j] = mat[i][j]
                prefixsum[i][j] += prefixsum[i - 1][j] if i > 0 else 0
                prefixsum[i][j] += prefixsum[i][j - 1] if j > 0 else 0
                prefixsum[i][j] -= prefixsum[i - 1][j - 1] \
                    if i > 0 and j > 0 else 0

                lengths = list(range(1, min(i, j) + 2))
                length = bisect.bisect_right(
                    lengths, threshold, key=lambda k: self.f(prefixsum, i, j, k))
                ret = max(ret, length)

        return ret

    def f(self, prefixsum, i, j, k):
        s = prefixsum[i][j]
        s -= prefixsum[i - k][j] if i >= k else 0
        s -= prefixsum[i][j - k] if j >= k else 0
        s += prefixsum[i - k][j - k] if i >= k and j >= k else 0

        return s
```
