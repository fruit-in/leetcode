# 149. 直线上最多的点数
给你一个数组 `points` ，其中 <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> 表示 **X-Y** 平面上的一个点。求最多有多少个点在同一条直线上。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/02/25/plane1.jpg)
<pre>
<strong>输入:</strong> points = [[1,1],[2,2],[3,3]]
<strong>输出:</strong> 3
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/02/25/plane2.jpg)
<pre>
<strong>输入:</strong> points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
<strong>输出:</strong> 4
</pre>

#### 提示:
* `1 <= points.length <= 300`
* `points[i].length == 2`
* <code>-10<sup>4</sup> <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>4</sup></code>
* `points` 中的所有点 **互不相同**

## 题解 (Python)

### 1. 题解
```Python
from math import gcd


class Solution:
    def maxPoints(self, points: List[List[int]]) -> int:
        ret = 1

        for i in range(len(points)):
            count = {}

            for j in range(i + 1, len(points)):
                dx = points[i][0] - points[j][0]
                dy = points[i][1] - points[j][1]

                if dx == 0:
                    k = None
                else:
                    neg = dx * dy < 0
                    dx = abs(dx)
                    dy = abs(dy)
                    g = gcd(dx, dy)
                    k = (neg, dy // g, dx // g)

                if k not in count:
                    count[k] = 1
                count[k] += 1
                ret = max(ret, count[k])

        return ret
```
