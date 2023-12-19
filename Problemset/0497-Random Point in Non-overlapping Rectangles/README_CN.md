# 497. 非重叠矩形中的随机点
给定一个由非重叠的轴对齐矩形的数组 `rects` ，其中 <code>rects[i] = [a<sub>i</sub>, b<sub>i</sub>, x<sub>i</sub>, y<sub>i</sub>]</code> 表示 <code>(a<sub>i</sub>, b<sub>i</sub>)</code> 是第 `i` 个矩形的左下角点，<code>(x<sub>i</sub>, y<sub>i</sub>)</code> 是第 `i` 个矩形的右上角点。设计一个算法来随机挑选一个被某一矩形覆盖的整数点。矩形周长上的点也算做是被矩形覆盖。所有满足要求的点必须等概率被返回。

在给定的矩形覆盖的空间内的任何整数点都有可能被返回。

**请注意** ，整数点是具有整数坐标的点。

实现 `Solution` 类:

* `Solution(int[][] rects)` 用给定的矩形数组 `rects` 初始化对象。
* `int[] pick()` 返回一个随机的整数点 `[u, v]` 在给定的矩形所覆盖的空间内。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/07/24/lc-pickrandomrec.jpg)
<pre>
<strong>输入:</strong>
["Solution", "pick", "pick", "pick", "pick", "pick"]
[[[[-2, -2, 1, 1], [2, 2, 4, 6]]], [], [], [], [], []]
<strong>输出:</strong>
[null, [1, -2], [1, -1], [-1, -2], [-2, -2], [0, 0]]
<strong>解释:</strong>
Solution solution = new Solution([[-2, -2, 1, 1], [2, 2, 4, 6]]);
solution.pick(); // 返回 [1, -2]
solution.pick(); // 返回 [1, -1]
solution.pick(); // 返回 [-1, -2]
solution.pick(); // 返回 [-2, -2]
solution.pick(); // 返回 [0, 0]
</pre>

#### 提示:
* `1 <= rects.length <= 100`
* `rects[i].length == 4`
* <code>-10<sup>9</sup> <= a<sub>i</sub> < x<sub>i</sub> <= 10<sup>9</sup></code>
* <code>-10<sup>9</sup> <= b<sub>i</sub> < y<sub>i</sub> <= 10<sup>9</sup></code>
* <code>x<sub>i</sub> - a<sub>i</sub> <= 2000</code>
* <code>y<sub>i</sub> - b<sub>i</sub> <= 2000</code>
* 所有的矩形不重叠。
* `pick` 最多被调用 <code>10<sup>4</sup></code> 次。

## 题解 (Python)

### 1. 题解
```Python
import random


class Solution:

    def __init__(self, rects: List[List[int]]):
        self.rects = []
        self.start = 0

        for a, b, x, y in rects:
            self.rects.append((self.start, a, b, x, y))
            self.start += (x - a + 1) * (y - b + 1)

    def pick(self) -> List[int]:
        n = random.randint(0, self.start - 1)
        i = bisect.bisect(self.rects, n, key=lambda r: r[0]) - 1
        start, a, b, x, _ = self.rects[i]
        n -= start

        return [a + n % (x - a + 1), b + n // (x - a + 1)]


# Your Solution object will be instantiated and called as such:
# obj = Solution(rects)
# param_1 = obj.pick()
```
