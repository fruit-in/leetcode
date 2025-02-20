# 1467. 两个盒子中球的颜色数相同的概率
桌面上有 `2n` 个颜色不完全相同的球，球的颜色共有 `k` 种。给你一个大小为 `k` 的整数数组 `balls` ，其中 `balls[i]` 是颜色为 `i` 的球的数量。

所有的球都已经 **随机打乱顺序** ，前 `n` 个球放入第一个盒子，后 `n` 个球放入另一个盒子（请认真阅读示例 2 的解释部分）。

**注意：**这两个盒子是不同的。例如，两个球颜色分别为 `a` 和 `b`，盒子分别为 `[]` 和 `()`，那么 `[a] (b)` 和 `[b] (a)` 这两种分配方式是不同的（请认真阅读示例的解释部分）。

请返回「两个盒子中球的颜色数相同」的情况的概率。答案与真实值误差在 <code>10<sup>-5</sup></code> 以内，则被视为正确答案

#### 示例 1:
<pre>
<strong>输入:</strong> balls = [1,1]
<strong>输出:</strong> 1.00000
<strong>解释:</strong> 球平均分配的方式只有两种：
- 颜色为 1 的球放入第一个盒子，颜色为 2 的球放入第二个盒子
- 颜色为 2 的球放入第一个盒子，颜色为 1 的球放入第二个盒子
这两种分配，两个盒子中球的颜色数都相同。所以概率为 2/2 = 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> balls = [2,1,1]
<strong>输出:</strong> 0.66667
<strong>解释:</strong> 球的列表为 [1, 1, 2, 3]
随机打乱，得到 12 种等概率的不同打乱方案，每种方案概率为 1/12 ：
[1,1 / 2,3], [1,1 / 3,2], [1,2 / 1,3], [1,2 / 3,1], [1,3 / 1,2], [1,3 / 2,1], [2,1 / 1,3], [2,1 / 3,1], [2,3 / 1,1], [3,1 / 1,2], [3,1 / 2,1], [3,2 / 1,1]
然后，我们将前两个球放入第一个盒子，后两个球放入第二个盒子。
这 12 种可能的随机打乱方式中的 8 种满足「两个盒子中球的颜色数相同」。
概率 = 8/12 = 0.66667
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> balls = [1,2,1,2]
<strong>输出:</strong> 0.60000
<strong>解释:</strong> 球的列表为 [1, 2, 2, 3, 4, 4]。要想显示所有 180 种随机打乱方案是很难的，但只检查「两个盒子中球的颜色数相同」的 108 种情况是比较容易的。
概率 = 108 / 180 = 0.6 。
</pre>

#### 提示:
* `1 <= balls.length <= 8`
* `1 <= balls[i] <= 6`
* `sum(balls)` 是偶数

## 题解 (Python)

### 1. 题解
```Python
from math import comb


class Solution:
    def getProbability(self, balls: List[int]) -> float:
        n = sum(balls) // 2
        k = len(balls)
        used = 0
        dp = [[[0] * (k * 2 + 1) for _ in range(n * 2 + 1)]
              for _ in range(n * 2 + 1)]
        dp[0][0][k] = 1

        for i in range(k):
            tmp = [[[0] * (k * 2 + 1) for _ in range(n * 2 + 1)]
                   for _ in range(n * 2 + 1)]

            for leftballs, rightballs in zip(range(used + 1), range(used, -1, -1)):
                for offsetdiff in range(len(dp[0][0])):
                    if rightballs + balls[i] < len(dp) and offsetdiff > 0:
                        tmp[leftballs][rightballs + balls[i]][offsetdiff -
                                                              1] += dp[leftballs][rightballs][offsetdiff] * comb(rightballs + balls[i], balls[i])
                    for toleft, toright in zip(range(1, balls[i]), range(balls[i] - 1, 0, -1)):
                        if leftballs + toleft < len(dp) and rightballs + toright < len(dp):
                            tmp[leftballs + toleft][rightballs + toright][offsetdiff] += dp[leftballs][rightballs][offsetdiff] * comb(
                                leftballs + toleft, toleft) * comb(rightballs + toright, toright)
                    if leftballs + balls[i] < len(dp) and offsetdiff + 1 < len(dp[0][0]):
                        tmp[leftballs + balls[i]][rightballs][offsetdiff +
                                                              1] += dp[leftballs][rightballs][offsetdiff] * comb(leftballs + balls[i], balls[i])

            used += balls[i]
            dp = tmp

        return dp[n][n][k] / sum(dp[n][n])
```
