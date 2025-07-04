# 1453. Maximum Number of Darts Inside of a Circular Dartboard
Alice is throwing `n` darts on a very large wall. You are given an array `darts` where <code>darts[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> is the position of the <code>i<sup>th</sup></code> dart that Alice threw on the wall.

Bob knows the positions of the `n` darts on the wall. He wants to place a dartboard of radius `r` on the wall so that the maximum number of darts that Alice throws lie on the dartboard.

Given the integer `r`, return *the maximum number of darts that can lie on the dartboard*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/04/29/sample_1_1806.png)
<pre>
<strong>Input:</strong> darts = [[-2,0],[2,0],[0,2],[0,-2]], r = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> Circle dartboard with center in (0,0) and radius = 2 contain all points.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/04/29/sample_2_1806.png)
<pre>
<strong>Input:</strong> darts = [[-3,0],[3,0],[2,6],[5,4],[0,9],[7,8]], r = 5
<strong>Output:</strong> 5
<strong>Explanation:</strong> Circle dartboard with center in (0,4) and radius = 5 contain all points except the point (7,8).
</pre>

#### Constraints:
* `1 <= darts.length <= 100`
* `darts[i].length == 2`
* <code>-10<sup>4</sup> <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>4</sup></code>
* All the `darts` are unique
* `1 <= r <= 5000`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numPoints(self, darts: List[List[int]], r: int) -> int:
        ret = 1

        for x1, y1 in darts:
            for x2, y2 in darts:
                base = dist((x1, y1), (x2, y2))
                if base == 0 or base > 2 * r:
                    continue
                h = sqrt(r ** 2 - base ** 2 / 4)
                x0, y0 = (x1 + x2) / 2 + h * (y1 - y2) / \
                    base, (y1 + y2) / 2 + h * (x2 - x1) / base
                ret = max(ret, sum(dist((x3, y3), (x0, y0)) -
                          r <= .00001 for x3, y3 in darts))

        return ret
```
