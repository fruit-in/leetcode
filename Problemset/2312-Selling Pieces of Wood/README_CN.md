# 2312. 卖木头块
给你两个整数 `m` 和 `n` ，分别表示一块矩形木块的高和宽。同时给你一个二维整数数组 `prices` ，其中 <code>prices[i] = [h<sub>i</sub>, w<sub>i</sub>, price<sub>i</sub>]</code> 表示你可以以 <code>price<sub>i</sub></code> 元的价格卖一块高为 <code>h<sub>i</sub></code> 宽为 <code>w<sub>i</sub></code> 的矩形木块。

每一次操作中，你必须按下述方式之一执行切割操作，以得到两块更小的矩形木块：

* 沿垂直方向按高度 **完全** 切割木块，或
* 沿水平方向按宽度 **完全** 切割木块

在将一块木块切成若干小木块后，你可以根据 `prices` 卖木块。你可以卖多块同样尺寸的木块。你不需要将所有小木块都卖出去。你 **不能** 旋转切好后木块来交换它的高度值和宽度值。

请你返回切割一块大小为 `m x n` 的木块后，能得到的 **最多** 钱数。

注意你可以切割木块任意次。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/04/27/ex1.png)
<pre>
<strong>输入:</strong> m = 3, n = 5, prices = [[1,4,2],[2,2,7],[2,1,3]]
<strong>输出:</strong> 19
<strong>解释:</strong> 上图展示了一个可行的方案。包括：
- 2 块 2 x 2 的小木块，售出 2 * 7 = 14 元。
- 1 块 2 x 1 的小木块，售出 1 * 3 = 3 元。
- 1 块 1 x 4 的小木块，售出 1 * 2 = 2 元。
总共售出 14 + 3 + 2 = 19 元。
19 元是最多能得到的钱数。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/04/27/ex2new.png)
<pre>
<strong>输入:</strong> m = 4, n = 6, prices = [[3,2,10],[1,4,2],[4,1,3]]
<strong>输出:</strong> 32
<strong>解释:</strong> 上图展示了一个可行的方案。包括：
- 3 块 3 x 2 的小木块，售出 3 * 10 = 30 元。
- 1 块 1 x 4 的小木块，售出 1 * 2 = 2 元。
总共售出 30 + 2 = 32 元。
32 元是最多能得到的钱数。
注意我们不能旋转 1 x 4 的木块来得到 4 x 1 的木块。
</pre>

#### 提示:
* `1 <= m, n <= 200`
* <code>1 <= prices.length <= 2 * 10<sup>4</sup></code>
* `prices[i].length == 3`
* <code>1 <= h<sub>i</sub> <= m</code>
* <code>1 <= w<sub>i</sub> <= n</code>
* <code>1 <= price<sub>i</sub> <= 10<sup>6</sup></code>
* 所有 <code>(h<sub>i</sub>, w<sub>i</sub>)</code> 互不相同 。

## 题解 (Python)

### 1. 题解
```Python
from functools import cache


class Solution:
    def sellingWood(self, m: int, n: int, prices: List[List[int]]) -> int:
        prices = {(h, w): price for h, w, price in prices}

        @cache
        def maxSelling(m: int, n: int) -> int:
            ret = prices.get((m, n), 0)

            for h in range(1, m):
                ret = max(ret, maxSelling(h, n) + maxSelling(m - h, n))
            for w in range(1, n):
                ret = max(ret, maxSelling(m, w) + maxSelling(m, n - w))

            return ret

        return maxSelling(m, n)
```
