# 2551. 将珠子放入背包中
你有 `k` 个背包。给你一个下标从 **0** 开始的整数数组 `weights` ，其中 `weights[i]` 是第 `i` 个珠子的重量。同时给你整数 `k` 。

请你按照如下规则将所有的珠子放进 `k` 个背包。

* 没有背包是空的。
* 如果第 `i` 个珠子和第 `j` 个珠子在同一个背包里，那么下标在 `i` 到 `j` 之间的所有珠子都必须在这同一个背包中。
* 如果一个背包有下标从 `i` 到 `j` 的所有珠子，那么这个背包的价格是 `weights[i] + weights[j]` 。

一个珠子分配方案的 **分数** 是所有 `k` 个背包的价格之和。

请你返回所有分配方案中，**最大分数** 与 **最小分数** 的 **差值** 为多少。

#### 示例 1:
<pre>
<strong>输入:</strong> weights = [1,3,5,1], k = 2
<strong>输出:</strong> 4
<strong>解释:</strong>
分配方案 [1],[3,5,1] 得到最小得分 (1+1) + (3+1) = 6 。
分配方案 [1,3],[5,1] 得到最大得分 (1+3) + (5+1) = 10 。
所以差值为 10 - 6 = 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> weights = [1, 3], k = 2
<strong>输出:</strong> 0
<strong>解释:</strong> 唯一的分配方案为 [1],[3] 。
最大最小得分相等，所以返回 0 。
</pre>

#### 提示:
* <code>1 <= k <= weights.length <= 10<sup>5</sup></code>
* <code>1 <= weights[i] <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def putMarbles(self, weights: List[int], k: int) -> int:
        weightsij = sorted(weights[i] + weights[i + 1]
                           for i in range(len(weights) - 1))

        return sum(weightsij[len(weights) - k:]) - sum(weightsij[:k - 1])
```
