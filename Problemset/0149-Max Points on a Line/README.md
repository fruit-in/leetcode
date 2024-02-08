# 149. Max Points on a Line
Given an array of `points` where <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> represents a point on the **X-Y** plane, return *the maximum number of points that lie on the same straight line*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/02/25/plane1.jpg)
<pre>
<strong>Input:</strong> points = [[1,1],[2,2],[3,3]]
<strong>Output:</strong> 3
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/02/25/plane2.jpg)
<pre>
<strong>Input:</strong> points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
<strong>Output:</strong> 4
</pre>

#### Constraints:
* `1 <= points.length <= 300`
* `points[i].length == 2`
* <code>-10<sup>4</sup> <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>4</sup></code>
* All the `points` are **unique**.

## Solutions (Python)

### 1. Solution
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
