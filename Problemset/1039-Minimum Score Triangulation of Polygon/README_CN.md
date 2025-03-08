# 1039. 多边形三角剖分的最低得分
你有一个凸的 `n` 边形，其每个顶点都有一个整数值。给定一个整数数组 `values` ，其中 `values[i]` 是第 `i` 个顶点的值（即 **顺时针顺序** ）。

假设将多边形 **剖分** 为 `n - 2` 个三角形。对于每个三角形，该三角形的值是顶点标记的**乘积**，三角剖分的分数是进行三角剖分后所有 `n - 2` 个三角形的值之和。

返回 *多边形进行三角剖分后可以得到的最低分* 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/02/25/shape1.jpg)
<pre>
<strong>输入:</strong> values = [1,2,3]
<strong>输出:</strong> 6
<strong>解释:</strong> 多边形已经三角化，唯一三角形的分数为 6。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/02/25/shape2.jpg)
<pre>
<strong>输入:</strong> values = [3,7,4,5]
<strong>输出:</strong> 144
<strong>解释:</strong> 有两种三角剖分，可能得分分别为：3*7*5 + 4*5*7 = 245，或 3*4*5 + 3*4*7 = 144。最低分数为 144。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/02/25/shape3.jpg)
<pre>
<strong>输入:</strong> values = [1,3,1,4,1,5]
<strong>输出:</strong> 13
<strong>解释:</strong> 最低分数三角剖分的得分情况为 1*1*3 + 1*1*4 + 1*1*5 + 1*1*1 = 13。
</pre>

#### 提示:
* `n == values.length`
* `3 <= n <= 50`
* `1 <= values[i] <= 100`

## 题解 (Python)

### 1. 题解
```Python
from functools import cache


class Solution:
    def minScoreTriangulation(self, values: List[int]) -> int:
        @cache
        def subMinScore(i: int, j: int) -> int:
            return min((values[i] * values[j] * values[k] + subMinScore(i, k) + subMinScore(k, j) for k in range(i + 1, j)), default=0)

        return subMinScore(0, len(values) - 1)
```
