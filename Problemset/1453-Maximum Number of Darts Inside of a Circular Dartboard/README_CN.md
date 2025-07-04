# 1453. 圆形靶内的最大飞镖数量
Alice 向一面非常大的墙上掷出 `n` 支飞镖。给你一个数组 `darts` ，其中 <code>darts[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> 表示 Alice 掷出的第 `i` 支飞镖落在墙上的位置。

Bob 知道墙上所有 `n` 支飞镖的位置。他想要往墙上放置一个半径为 `r` 的圆形靶。使 Alice 掷出的飞镖尽可能多地落在靶上。

给你整数 `r` ，请返回能够落在 **任意** 半径为 `r` 的圆形靶内或靶上的最大飞镖数。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/04/29/sample_1_1806.png)
<pre>
<strong>输入:</strong> darts = [[-2,0],[2,0],[0,2],[0,-2]], r = 2
<strong>输出:</strong> 4
<strong>解释:</strong> 如果圆形靶的圆心为 (0,0) ，半径为 2 ，所有的飞镖都落在靶上，此时落在靶上的飞镖数最大，值为 4 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/04/29/sample_2_1806.png)
<pre>
<strong>输入:</strong> darts = [[-3,0],[3,0],[2,6],[5,4],[0,9],[7,8]], r = 5
<strong>输出:</strong> 5
<strong>解释:</strong> 如果圆形靶的圆心为 (0,4) ，半径为 5 ，则除了 (7,8) 之外的飞镖都落在靶上，此时落在靶上的飞镖数最大，值为 5 。
</pre>

#### 提示:
* `1 <= darts.length <= 100`
* `darts[i].length == 2`
* <code>-10<sup>4</sup> <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>4</sup></code>
* `darts` 中的元素互不相同
* `1 <= r <= 5000`

## 题解 (Python)

### 1. 题解
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
