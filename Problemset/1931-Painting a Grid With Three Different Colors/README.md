# 1931. Painting a Grid With Three Different Colors
You are given two integers `m` and `n`. Consider an `m x n` grid where each cell is initially white. You can paint each cell **red**, **green**, or **blue**. All cells **must** be painted.

Return *the number of ways to color the grid with **no two adjacent cells having the same color***. Since the answer can be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/06/22/colorthegrid.png)
<pre>
<strong>Input:</strong> m = 1, n = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong> The three possible colorings are shown in the image above.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/06/22/copy-of-colorthegrid.png)
<pre>
<strong>Input:</strong> m = 1, n = 2
<strong>Output:</strong> 6
<strong>Explanation:</strong> The six possible colorings are shown in the image above.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> m = 5, n = 5
<strong>Output:</strong> 580986
</pre>

#### Constraints:
* `1 <= m <= 5`
* `1 <= n <= 1000`

## Solutions (Python)

### 1. Solution
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
