# 1931. 用三种不同颜色为网格涂色
给你两个整数 `m` 和 `n` 。构造一个 `m x n` 的网格，其中每个单元格最开始是白色。请你用 **红、绿、蓝** 三种颜色为每个单元格涂色。所有单元格都需要被涂色。

涂色方案需要满足：**不存在相邻两个单元格颜色相同的情况** 。返回网格涂色的方法数。因为答案可能非常大， 返回 对 <code>10<sup>9</sup> + 7</code> **取余** 的结果。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/06/22/colorthegrid.png)
<pre>
<strong>输入:</strong> m = 1, n = 1
<strong>输出:</strong> 3
<strong>解释:</strong> 如上图所示，存在三种可能的涂色方案。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/06/22/copy-of-colorthegrid.png)
<pre>
<strong>输入:</strong> m = 1, n = 2
<strong>输出:</strong> 6
<strong>解释:</strong> 如上图所示，存在六种可能的涂色方案。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> m = 5, n = 5
<strong>输出:</strong> 580986
</pre>

#### 提示:
* `1 <= m <= 5`
* `1 <= n <= 1000`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def colorTheGrid(self, m: int, n: int) -> int:
        patterns = list("rgb")
        nextpatterns = {}
        ways = {}

        for _ in range(m - 1):
            tmp = []

            for p in patterns:
                for c in "rgb":
                    if p[-1] != c:
                        tmp.append(p + c)

            patterns = tmp

        nextpatterns = {p: [] for p in patterns}
        ways = {p: 1 for p in patterns}

        for i in range(len(patterns)):
            for j in range(i + 1, len(patterns)):
                if all(c0 != c1 for c0, c1 in zip(patterns[i], patterns[j])):
                    nextpatterns[patterns[i]].append(patterns[j])
                    nextpatterns[patterns[j]].append(patterns[i])

        for _ in range(n - 1):
            tmp = {p: 0 for p in patterns}

            for p0 in patterns:
                for p1 in nextpatterns[p0]:
                    tmp[p1] = (tmp[p1] + ways[p0]) % 1000000007

            ways = tmp

        return sum(ways.values()) % 1000000007
```
